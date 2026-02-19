use candid::Principal;

use crate::util::{PicCanisterTrait, TestSetup};

#[test]
fn test_bucket_stat() {
    let setup = TestSetup::default();
    let res: String = setup
        .bucket
        .update(Principal::anonymous(), "stat", ())
        .unwrap();
    assert!(res.contains("Chunks stored"));
}

// More complex bucket tests will be in flow_tests.rs because they need tokens from directory
