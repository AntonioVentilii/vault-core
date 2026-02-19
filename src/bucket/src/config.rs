use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub icp_ledger: Option<Principal>,
    pub ckusdc_ledger: Option<Principal>,
    pub read_only: bool,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct InitArgs {
    pub icp_ledger: Principal,
    pub ckusdc_ledger: Principal,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UpgradeArgs {
    pub icp_ledger: Option<Principal>,
    pub ckusdc_ledger: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub enum Args {
    Init(InitArgs),
    Upgrade(Option<UpgradeArgs>),
}

impl From<InitArgs> for Config {
    fn from(args: InitArgs) -> Self {
        Self {
            icp_ledger: Some(args.icp_ledger),
            ckusdc_ledger: Some(args.ckusdc_ledger),
            read_only: false,
        }
    }
}
