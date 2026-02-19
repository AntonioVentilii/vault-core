use bucket::results::{GetChunkResult, PutChunkResult};
use candid::Principal;
use directory::results::{
    CommitUploadResult, GetDownloadPlanResult, GetUploadTokensResult, ReportChunkUploadedResult,
    StartUploadResult,
};
use ic_papi_api::PaymentType;

use crate::util::{PicCanisterTrait, TestSetup};

#[test]
fn test_full_upload_flow() {
    let setup = TestSetup::default();
    let caller = Principal::from_slice(&[1; 29]);

    // 1. Start Upload
    let start_res: StartUploadResult = setup
        .directory
        .update_with_cycles(
            &setup.proxy,
            caller,
            "start_upload",
            (
                "flow.txt".to_string(),
                "text/plain".to_string(),
                10u64,
                None::<PaymentType>,
            ),
            200_000,
        )
        .unwrap();
    let session = match start_res {
        StartUploadResult::Ok(s) => s,
        StartUploadResult::Err(e) => panic!("Start upload failed: {:?}", e),
    };

    // 2. Get Upload Tokens
    let token_res: GetUploadTokensResult = setup
        .directory
        .update(
            setup.proxy.canister_id,
            "get_upload_tokens",
            (session.upload_id.clone(), vec![0u32]),
        )
        .unwrap();
    let tokens = match token_res {
        GetUploadTokensResult::Ok(t) => t,
        GetUploadTokensResult::Err(e) => panic!("Get tokens failed: {:?}", e),
    };
    let token = tokens[0].clone();

    // 3. Put Chunk in Bucket
    let chunk_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let put_res: PutChunkResult = setup
        .bucket
        .update_with_cycles(
            &setup.proxy,
            caller,
            "put_chunk",
            (token, 0u32, chunk_data.clone(), None::<PaymentType>),
            100_000,
        )
        .unwrap();
    assert!(matches!(put_res, PutChunkResult::Ok(10)));

    // 4. Report Chunk to Directory
    let _: ReportChunkUploadedResult = setup
        .directory
        .update(
            setup.proxy.canister_id,
            "report_chunk_uploaded",
            (session.upload_id.clone(), 0u32),
        )
        .unwrap();

    // 5. Commit Upload
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
        CommitUploadResult::Err(e) => panic!("Commit failed: {:?}", e),
    };
    assert_eq!(meta.name, "flow.txt");

    // 6. Get Download Plan
    let plan_res: GetDownloadPlanResult = setup
        .directory
        .update(
            setup.proxy.canister_id,
            "get_download_plan",
            (meta.file_id.clone(),),
        )
        .unwrap();
    let plan = match plan_res {
        GetDownloadPlanResult::Ok(p) => p,
        GetDownloadPlanResult::Err(e) => panic!("Get download plan failed: {:?}", e),
    };

    // Find token for the bucket
    let bucket_token = plan
        .auth
        .iter()
        .find(|a| a.bucket_id == setup.bucket.canister_id)
        .expect("No auth for bucket")
        .token
        .clone();

    // 7. Download Chunk from Bucket
    let chunk_res: GetChunkResult = setup
        .bucket
        .query(caller, "get_chunk", (bucket_token, 0u32))
        .unwrap();
    match chunk_res {
        GetChunkResult::Ok(data) => assert_eq!(data, chunk_data),
        GetChunkResult::Err(e) => panic!("Get chunk failed: {:?}", e),
    }
}
