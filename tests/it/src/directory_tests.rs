use candid::Principal;
use directory::results::StartUploadResult;
use ic_papi_api::PaymentType;

use crate::util::{PicCanisterTrait, TestSetup};

#[test]
fn test_start_upload() {
    let setup = TestSetup::default();
    let caller = Principal::from_slice(&[1; 29]);

    let res: StartUploadResult = setup
        .directory
        .update_with_cycles(
            &setup.proxy,
            caller,
            "start_upload",
            (
                "test.txt".to_string(),
                "text/plain".to_string(),
                1024u64,
                None::<PaymentType>,
            ),
            200_000,
        )
        .unwrap();

    match res {
        StartUploadResult::Ok(session) => {
            assert_eq!(session.name, "test.txt");
        }
        StartUploadResult::Err(e) => panic!("Start upload failed: {:?}", e),
    }
}

#[test]
fn test_list_files_empty() {
    let setup = TestSetup::default();
    let caller = Principal::from_slice(&[1; 29]);
    let files: Vec<shared::types::FileMeta> =
        setup.directory.update(caller, "list_files", ()).unwrap();
    assert!(files.is_empty());
}
