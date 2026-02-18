pub mod api;
pub mod memory;
pub mod types;

pub use api::{delete_file, get_chunk, put_chunk, stat};
use ic_cdk::export_candid;
use shared::types::{FileId, UploadToken};

export_candid!();
