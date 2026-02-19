pub mod api;
pub mod config;
pub mod errors;
pub mod memory;
pub mod payments;
pub mod results;
pub mod types;

pub use api::{
    add_file_access, admin_set_pricing, admin_set_quota, admin_withdraw, commit_upload,
    create_share_link, delete_file, estimate_upload_cost, garbage_collect, get_pricing, get_status,
    get_upload_tokens, get_usage, list_files, provision_bucket, reap_expired_uploads,
    remove_file_access, report_chunk_uploaded, resolve_share_link, revoke_share_link, start_upload,
    top_up_balance,
};
use candid::Principal;
use ic_cdk::{export_candid, spawn};
use ic_cdk_macros::{heartbeat, init, post_upgrade};
pub use ic_papi_api::PaymentType;
use shared::{
    types::{FileId, FileMeta, FileRole, PricingConfig, UserId},
    CanisterStatus,
};

use crate::{
    config::Args,
    errors::DirectoryError,
    memory::{mutate_config, set_config},
    results::{
        AbortUploadResult, AdminWithdrawResult, CommitUploadResult, CreateShareLinkResult,
        DeleteFileResult, GetDownloadPlanResult, GetFileMetaResult, GetUploadTokensResult,
        ProvisionBucketResult, ReportChunkUploadedResult, ResolveShareLinkResult,
        StartUploadResult, TopUpBalanceResult,
    },
    types::UserState,
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
                    if let Some(admins) = upgrade_args.admins {
                        config.admins = Some(admins);
                    }
                    if let Some(rate) = upgrade_args.rate_per_gb_per_month {
                        config.rate_per_gb_per_month = Some(rate);
                    }
                    if let Some(secret) = upgrade_args.shared_secret {
                        config.shared_secret = Some(secret);
                    }
                });
            }
            Args::Upgrade(None) => {}
            Args::Init(_) => ic_cdk::trap("Cannot use init variant in post_upgrade"),
        }
    }
}

#[heartbeat]
fn heartbeat() {
    // Only run garbage collection occasionally (e.g., every 1000 heartbeats)
    thread_local! {
        static TICK: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    }

    TICK.with(|t| {
        let current = t.get();
        if current % 1000 == 0 {
            spawn(garbage_collect());
        }
        t.set(current + 1);
    });
}

export_candid!();
