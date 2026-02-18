pub mod api;
pub mod memory;
pub mod types;

pub use api::*;
use ic_cdk::export_candid;
use shared::{FileId, UploadToken};

export_candid!();
