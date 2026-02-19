use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub icp_ledger: Principal,
    pub ckusdc_ledger: Principal,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct InitArgs {
    pub icp_ledger: Principal,
    pub ckusdc_ledger: Principal,
}

impl From<InitArgs> for Config {
    fn from(args: InitArgs) -> Self {
        Self {
            icp_ledger: args.icp_ledger,
            ckusdc_ledger: args.ckusdc_ledger,
        }
    }
}
