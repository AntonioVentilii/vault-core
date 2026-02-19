use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct UserState {
    pub used_bytes: u64,
    pub quota_bytes: u64,
    pub expires_at_ns: Option<u64>,
    pub prepaid_balance: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct BucketInfo {
    pub id: Principal,
    pub writable: bool,
    pub used_bytes: u64,
    pub soft_limit_bytes: u64,
    pub hard_limit_bytes: u64,
}
