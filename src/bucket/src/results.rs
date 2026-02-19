use candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::errors::BucketError;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum PutChunkResult {
    Ok(u32),
    Err(BucketError),
}
impl From<Result<u32, BucketError>> for PutChunkResult {
    fn from(value: Result<u32, BucketError>) -> Self {
        match value {
            Ok(v) => PutChunkResult::Ok(v),
            Err(e) => PutChunkResult::Err(e),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum GetChunkResult {
    Ok(Vec<u8>),
    Err(BucketError),
}
impl From<Result<Vec<u8>, BucketError>> for GetChunkResult {
    fn from(value: Result<Vec<u8>, BucketError>) -> Self {
        match value {
            Ok(v) => GetChunkResult::Ok(v),
            Err(e) => GetChunkResult::Err(e),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum DeleteFileResult {
    Ok,
    Err(BucketError),
}

impl From<Result<(), BucketError>> for DeleteFileResult {
    fn from(value: Result<(), BucketError>) -> Self {
        match value {
            Ok(_) => DeleteFileResult::Ok,
            Err(e) => DeleteFileResult::Err(e),
        }
    }
}
