use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

/// Configuration stored in stable storage.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    /// List of principals with administrative rights.
    pub admins: Option<Vec<Principal>>,
    /// Storage rate per GB per month in tokens (e.g., 100_000_000 for 0.1 tokens).
    pub rate_per_gb_per_month: Option<u64>,
    /// Secret used to sign tokens shared with bucket canisters.
    pub shared_secret: Option<Vec<u8>>,
}

/// Arguments for initializing the directory canister.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct InitArgs {
    /// Initial list of administrators.
    pub admins: Vec<Principal>,
    /// Initial storage rate per GB per month.
    pub rate_per_gb_per_month: u64,
    /// Initial shared secret for token signing.
    pub shared_secret: Vec<u8>,
}

/// Arguments for upgrading the directory canister.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UpgradeArgs {
    /// Optional update for the administrator list.
    pub admins: Option<Vec<Principal>>,
    /// Optional update for the storage rate.
    pub rate_per_gb_per_month: Option<u64>,
    /// Optional update for the shared secret.
    pub shared_secret: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize)]
pub enum Args {
    Init(InitArgs),
    Upgrade(Option<UpgradeArgs>),
}

impl From<InitArgs> for Config {
    fn from(args: InitArgs) -> Self {
        Self {
            admins: Some(args.admins),
            rate_per_gb_per_month: Some(args.rate_per_gb_per_month),
            shared_secret: Some(args.shared_secret),
        }
    }
}
