use bucket::results::{GetChunkResult, PutChunkResult};
use candid::Principal;
use directory::results::{
    CommitUploadResult, CreateShareLinkResult, GetUploadTokensResult, ReportChunkUploadedResult,
    ResolveShareLinkResult, StartUploadResult,
};
use ic_papi_api::PaymentType;

use crate::util::{PicCanisterTrait, TestSetup};

#[test]
fn test_link_generation_and_access() {
    let setup = TestSetup::default();
    let owner = Principal::from_slice(&[1; 29]);
    let anon = Principal::anonymous();

    // 1. Upload a file via Proxy
    let start_res: StartUploadResult = setup
        .directory
        .update_with_cycles(
            &setup.proxy,
            owner,
            "start_upload",
            (
                "link.txt".to_string(),
                "text/plain".to_string(),
                10u64,
                None::<PaymentType>,
            ),
            200_000,
        )
        .unwrap();
    let session = match start_res {
        StartUploadResult::Ok(s) => s,
        _ => panic!("Start failed"),
    };

    let token_res: GetUploadTokensResult = setup
        .directory
        .update(
            setup.proxy.canister_id,
            "get_upload_tokens",
            (session.upload_id.clone(), vec![0u32]),
        )
        .unwrap();
    let token = match token_res {
        GetUploadTokensResult::Ok(t) => t[0].clone(),
        _ => panic!("Get tokens failed"),
    };

    let chunk_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let _: PutChunkResult = setup
        .bucket
        .update_with_cycles(
            &setup.proxy,
            owner,
            "put_chunk",
            (token, 0u32, chunk_data.clone(), None::<PaymentType>),
            100_000,
        )
        .unwrap();

    let _: ReportChunkUploadedResult = setup
        .directory
        .update(
            setup.proxy.canister_id,
            "report_chunk_uploaded",
            (session.upload_id.clone(), 0u32),
        )
        .unwrap();

    let commit_res: CommitUploadResult = setup
        .directory
        .update(
            setup.proxy.canister_id,
            "commit_upload",
            (session.upload_id.clone(),),
        )
        .unwrap();
    let meta = match commit_res {
        CommitUploadResult::Ok(m) => m,
        _ => panic!("Commit failed"),
    };

    // 2. Create Share Link
    let ttl_ns = 3600_000_000_000u64; // 1 hour
    let link_res: CreateShareLinkResult = setup
        .directory
        .update_with_cycles(
            &setup.proxy,
            owner,
            "create_share_link",
            (meta.file_id.clone(), ttl_ns),
            0,
        )
        .unwrap();
    let link_token = match link_res {
        CreateShareLinkResult::Ok(t) => t,
        CreateShareLinkResult::Err(e) => panic!("Create link failed: {:?}", e),
    };

    // 3. Resolve Share Link (Anonymous)
    let plan_res: ResolveShareLinkResult = setup
        .directory
        .query(anon, "resolve_share_link", (link_token.clone(),))
        .unwrap();
    let plan = match plan_res {
        ResolveShareLinkResult::Ok(p) => p,
        ResolveShareLinkResult::Err(e) => panic!("Resolve link failed: {:?}", e),
    };

    // 4. Download using the resolved token (Anonymous)
    let bucket_token = plan.auth[0].token.clone();
    let chunk_res: GetChunkResult = setup
        .bucket
        .query(anon, "get_chunk", (bucket_token, 0u32))
        .unwrap();
    match chunk_res {
        GetChunkResult::Ok(data) => assert_eq!(data, chunk_data),
        GetChunkResult::Err(e) => panic!("Download via link failed: {:?}", e),
    }

    // 5. Revoke link (Owner)
    let _: Result<(), directory::errors::DirectoryError> = setup
        .directory
        .update_with_cycles(
            &setup.proxy,
            owner,
            "revoke_share_link",
            (link_token.clone(),),
            0,
        )
        .unwrap();

    // 6. Verify link no longer resolves
    let plan_res_fail: ResolveShareLinkResult = setup
        .directory
        .query(anon, "resolve_share_link", (link_token,))
        .unwrap();
    assert!(matches!(
        plan_res_fail,
        ResolveShareLinkResult::Err(directory::errors::DirectoryError::LinkNotFound)
    ));
}
