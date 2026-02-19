pub mod api;
pub mod config;
pub mod errors;
pub mod memory;
pub mod payments;
pub mod results;
pub mod types;

pub use api::{admin_set_read_only, admin_withdraw, delete_file, get_chunk, put_chunk, stat};
use candid::Principal;
use ic_cdk::export_candid;
use ic_cdk_macros::{init, post_upgrade};
pub use ic_papi_api::PaymentType;
use shared::types::{FileId, UploadToken};

use crate::{
    config::Args,
    memory::{mutate_config, set_config},
    results::{
        AdminSetReadOnlyResult, AdminWithdrawResult, DeleteFileResult, GetChunkResult,
        PutChunkResult,
    },
};

#[init]
fn init(args: Args) {
    match args {
        Args::Init(args) => set_config(args.into()),
        Args::Upgrade(_) => ic_cdk::trap("Use init to initialize the canister"),
    }
}

#[post_upgrade]
fn post_upgrade(args: Option<Args>) {
    if let Some(args) = args {
        match args {
            Args::Upgrade(Some(upgrade_args)) => {
                mutate_config(|config| {
                    if let Some(icp) = upgrade_args.icp_ledger {
                        config.icp_ledger = Some(icp);
                    }
                    if let Some(ckusdc) = upgrade_args.ckusdc_ledger {
                        config.ckusdc_ledger = Some(ckusdc);
                    }
                });
            }
            Args::Upgrade(None) => {}
            Args::Init(_) => ic_cdk::trap("Cannot use init variant in post_upgrade"),
        }
    }
}

export_candid!();
