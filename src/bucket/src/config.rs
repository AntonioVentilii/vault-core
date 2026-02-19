use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub admins: Vec<Principal>,
    pub icp_ledger: Option<Principal>,
    pub ckusdc_ledger: Option<Principal>,
    pub read_only: bool,
    pub shared_secret: Vec<u8>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct InitArgs {
    pub admins: Vec<Principal>,
    pub icp_ledger: Principal,
    pub ckusdc_ledger: Principal,
    pub shared_secret: Vec<u8>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UpgradeArgs {
    pub admins: Option<Vec<Principal>>,
    pub icp_ledger: Option<Principal>,
    pub ckusdc_ledger: Option<Principal>,
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
            admins: args.admins,
            icp_ledger: Some(args.icp_ledger),
            ckusdc_ledger: Some(args.ckusdc_ledger),
            read_only: false,
            shared_secret: args.shared_secret,
        }
    }
}
