use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct UserState {
    pub used_bytes: u64,
    pub quota_bytes: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct BucketInfo {
    pub id: Principal,
    pub writable: bool,
    pub used_bytes: u64,
    pub soft_limit_bytes: u64,
    pub hard_limit_bytes: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum SignerMethods {
    StartUpload,
    PutChunk,
}

impl SignerMethods {
    pub fn fee(&self) -> u64 {
        match self {
            SignerMethods::StartUpload => 1_000_000, // 1M cycles/tokens fee
            SignerMethods::PutChunk => 100_000,
        }
    }
}
