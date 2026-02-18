pub mod api;
pub mod config;
pub mod errors;
pub mod memory;
pub mod payments;
pub mod types;

pub use api::{
    commit_upload, delete_file, get_pricing, get_upload_tokens, get_usage, list_files,
    provision_bucket, report_chunk_uploaded, start_upload,
};
use candid::Principal;
use ic_cdk::export_candid;
use ic_cdk_macros::{init, post_upgrade};
pub use ic_papi_api::PaymentType;
use shared::types::{FileId, FileMeta, UploadSession, UploadToken};

use crate::{config::InitArgs, errors::DirectoryError, memory::set_config, types::UserState};

#[init]
fn init(args: InitArgs) {
    set_config(args);
}

#[post_upgrade]
fn post_upgrade(args: InitArgs) {
    set_config(args);
}

export_candid!();
