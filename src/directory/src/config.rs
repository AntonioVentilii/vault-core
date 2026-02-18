use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub cycles_ledger: Principal,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct InitArgs {
    pub cycles_ledger: Principal,
}

impl From<InitArgs> for Config {
    fn from(args: InitArgs) -> Self {
        Self {
            cycles_ledger: args.cycles_ledger,
        }
    }
}
