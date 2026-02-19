use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

/// Configuration stored in stable storage.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    /// List of principals who can perform admin actions (e.g., emergency cleanup, withdrawing
    /// cycles).
    pub admins: Option<Vec<Principal>>,
    /// Whether the bucket is in read-only mode (prevents new uploads).
    pub read_only: Option<bool>,
    /// Secret used to verify the authenticity of tokens issued by the directory.
    pub shared_secret: Option<Vec<u8>>,
}

/// Arguments for initializing the bucket canister.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct InitArgs {
    /// Initial list of administrators for this bucket.
    pub admins: Vec<Principal>,
    /// Initial shared secret used to authenticate directory requests.
    pub shared_secret: Vec<u8>,
}

/// Arguments for upgrading the bucket canister.
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UpgradeArgs {
    /// Optional update for the administrator list.
    pub admins: Option<Vec<Principal>>,
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
            read_only: Some(false),
            shared_secret: Some(args.shared_secret),
        }
    }
}
