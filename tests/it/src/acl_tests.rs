use candid::Principal;
use directory::results::{
    CommitUploadResult, DeleteFileResult, GetFileMetaResult, StartUploadResult,
};
use ic_papi_api::PaymentType;
use shared::types::FileRole;

use crate::util::{PicCanisterTrait, TestSetup};

#[test]
fn test_acl_add_remove_access() {
    let setup = TestSetup::default();
    let caller = Principal::from_slice(&[1; 29]); // Caller of the proxy
    let viewer = Principal::from_slice(&[2; 29]); // Viewer identity

    // 1. Upload a file via Proxy (Owner = Proxy Canister ID)
    let start_res: StartUploadResult = setup
        .directory
        .update_with_cycles(
            &setup.proxy,
            caller,
            "start_upload",
            (
                "acl_test.txt".to_string(),
                "text/plain".to_string(),
                0u64,
                None::<PaymentType>,
            ),
            200_000,
        )
        .unwrap();

    let session = match start_res {
        StartUploadResult::Ok(s) => s,
        StartUploadResult::Err(e) => panic!("Start upload failed: {:?}", e),
    };

    let commit_res: CommitUploadResult = setup
        .directory
        .update_with_cycles(
            &setup.proxy,
            caller,
            "commit_upload",
            (session.upload_id.clone(),),
            0,
        )
        .unwrap();

    let meta = match commit_res {
        CommitUploadResult::Ok(m) => m,
        CommitUploadResult::Err(e) => panic!("Commit failed: {:?}", e),
    };
    let file_id = meta.file_id;

    // 2. Verify viewer CANNOT read yet (Direct Query)
    let meta_res_fail: GetFileMetaResult = setup
        .directory
        .query(viewer, "get_file_meta", (file_id.clone(),))
        .unwrap();
    assert!(
        matches!(meta_res_fail, GetFileMetaResult::Err(_)),
        "Viewer should NOT have access yet"
    );

    // 3. Owner (Proxy) adds viewer (via Proxy)
    let add_res: DeleteFileResult = setup
        .directory
        .update_with_cycles(
            &setup.proxy,
            caller,
            "add_file_access",
            (file_id.clone(), viewer, FileRole::Reader),
            0,
        )
        .unwrap();
    assert!(matches!(add_res, DeleteFileResult::Ok));

    // 4. Verify viewer CAN read now (Direct Query)
    let meta_res_ok: GetFileMetaResult = setup
        .directory
        .query(viewer, "get_file_meta", (file_id.clone(),))
        .unwrap();
    match meta_res_ok {
        GetFileMetaResult::Ok(m) => {
            assert!(m.readers.contains(&viewer));
        }
        GetFileMetaResult::Err(e) => panic!("Viewer should have access now: {:?}", e),
    }

    // 5. Owner (Proxy) removes viewer (via Proxy)
    let remove_res: DeleteFileResult = setup
        .directory
        .update_with_cycles(
            &setup.proxy,
            caller,
            "remove_file_access",
            (file_id.clone(), viewer),
            0,
        )
        .unwrap();
    assert!(matches!(remove_res, DeleteFileResult::Ok));

    // 6. Verify viewer CANNOT read anymore (Direct Query)
    let meta_res_fail_again: GetFileMetaResult = setup
        .directory
        .query(viewer, "get_file_meta", (file_id.clone(),))
        .unwrap();
    assert!(
        matches!(meta_res_fail_again, GetFileMetaResult::Err(_)),
        "Viewer should NOT have access anymore"
    );
}

#[test]
fn test_acl_writer_delete() {
    let setup = TestSetup::default();
    let caller = Principal::from_slice(&[1; 29]); // Caller of the proxy
    let other_user = Principal::from_slice(&[3; 29]); // Other user identity

    // 1. Upload a file via Proxy
    let start_res: StartUploadResult = setup
        .directory
        .update_with_cycles(
            &setup.proxy,
            caller,
            "start_upload",
            (
                "writer_test.txt".to_string(),
                "text/plain".to_string(),
                0u64,
                None::<PaymentType>,
            ),
            200_000,
        )
        .unwrap();
    let session = match start_res {
        StartUploadResult::Ok(s) => s,
        StartUploadResult::Err(e) => panic!("Start upload failed: {:?}", e),
    };
    let commit_res: CommitUploadResult = setup
        .directory
        .update_with_cycles(
            &setup.proxy,
            caller,
            "commit_upload",
            (session.upload_id.clone(),),
            0,
        )
        .unwrap();
    let file_id = match commit_res {
        CommitUploadResult::Ok(m) => m.file_id,
        CommitUploadResult::Err(e) => panic!("Commit failed: {:?}", e),
    };

    // 2. Verify other user cannot delete
    let del_res_fail: DeleteFileResult = setup
        .directory
        .update(other_user, "delete_file", (file_id.clone(),))
        .unwrap();
    assert!(matches!(del_res_fail, DeleteFileResult::Err(_)));

    // 3. Grant Writer access to other user
    let add_res: DeleteFileResult = setup
        .directory
        .update_with_cycles(
            &setup.proxy,
            caller,
            "add_file_access",
            (file_id.clone(), other_user, FileRole::Writer),
            0,
        )
        .unwrap();
    assert!(matches!(add_res, DeleteFileResult::Ok));

    // 4. Verify other user CAN read and delete
    let meta_res: GetFileMetaResult = setup
        .directory
        .query(other_user, "get_file_meta", (file_id.clone(),))
        .unwrap();
    assert!(matches!(meta_res, GetFileMetaResult::Ok(_)));

    let del_res_ok: DeleteFileResult = setup
        .directory
        .update(other_user, "delete_file", (file_id.clone(),))
        .unwrap();
    assert!(matches!(del_res_ok, DeleteFileResult::Ok));

    // 5. Verify file is gone
    let meta_res_gone: GetFileMetaResult = setup
        .directory
        .query(other_user, "get_file_meta", (file_id.clone(),))
        .unwrap();
    assert!(matches!(meta_res_gone, GetFileMetaResult::Err(_)));
}
