use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum DirectoryError {
    PaymentFailed(String),
    QuotaExceeded {
        used: u64,
        requested: u64,
        quota: u64,
    },
    UploadSessionNotFound,
    UploadIncomplete {
        uploaded: u32,
        expected: u32,
    },
    Unauthorized,
    FileNotFound,
    NoWritableBuckets,
    TransferFailed(String),
    InvalidRequest(String),
    LinkNotFound,
    LinkExpired,
    AccountExpired,
    AdminOnly,
    BucketAlreadyExists,
}
