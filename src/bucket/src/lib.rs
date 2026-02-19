pub mod api;
pub mod errors;
pub mod memory;
pub mod payments;
pub mod results;
pub mod types;

pub use api::{delete_file, get_chunk, put_chunk, stat};
use ic_cdk::export_candid;
pub use ic_papi_api::PaymentType;
use shared::types::{FileId, UploadToken};

use crate::results::{DeleteFileResult, GetChunkResult, PutChunkResult};

export_candid!();
