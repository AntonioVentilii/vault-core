use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum BucketError {
    PaymentFailed(String),
    InvalidSignature,
    TokenExpired,
    WrongBucket,
    ChunkNotAllowed(u32),
    InvalidFileId,
    ChunkNotFound,
    Unauthorized,
    Other(String),
}
