use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use shared::types::{DownloadPlan, FileMeta, UploadSession, UploadToken};

use crate::errors::DirectoryError;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum StartUploadResult {
    Ok(UploadSession),
    Err(DirectoryError),
}
impl From<Result<UploadSession, DirectoryError>> for StartUploadResult {
    fn from(value: Result<UploadSession, DirectoryError>) -> Self {
        match value {
            Ok(v) => StartUploadResult::Ok(v),
            Err(e) => StartUploadResult::Err(e),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ReportChunkUploadedResult {
    Ok,
    Err(DirectoryError),
}
impl From<Result<(), DirectoryError>> for ReportChunkUploadedResult {
    fn from(value: Result<(), DirectoryError>) -> Self {
        match value {
            Ok(_) => ReportChunkUploadedResult::Ok,
            Err(e) => ReportChunkUploadedResult::Err(e),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum CommitUploadResult {
    Ok(FileMeta),
    Err(DirectoryError),
}
impl From<Result<FileMeta, DirectoryError>> for CommitUploadResult {
    fn from(value: Result<FileMeta, DirectoryError>) -> Self {
        match value {
            Ok(v) => CommitUploadResult::Ok(v),
            Err(e) => CommitUploadResult::Err(e),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum AbortUploadResult {
    Ok,
    Err(DirectoryError),
}
impl From<Result<(), DirectoryError>> for AbortUploadResult {
    fn from(value: Result<(), DirectoryError>) -> Self {
        match value {
            Ok(_) => AbortUploadResult::Ok,
            Err(e) => AbortUploadResult::Err(e),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum GetFileMetaResult {
    Ok(FileMeta),
    Err(DirectoryError),
}
impl From<Result<FileMeta, DirectoryError>> for GetFileMetaResult {
    fn from(value: Result<FileMeta, DirectoryError>) -> Self {
        match value {
            Ok(v) => GetFileMetaResult::Ok(v),
            Err(e) => GetFileMetaResult::Err(e),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum GetDownloadPlanResult {
    Ok(DownloadPlan),
    Err(DirectoryError),
}
impl From<Result<DownloadPlan, DirectoryError>> for GetDownloadPlanResult {
    fn from(value: Result<DownloadPlan, DirectoryError>) -> Self {
        match value {
            Ok(v) => GetDownloadPlanResult::Ok(v),
            Err(e) => GetDownloadPlanResult::Err(e),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum DeleteFileResult {
    Ok,
    Err(DirectoryError),
}
impl From<Result<(), DirectoryError>> for DeleteFileResult {
    fn from(value: Result<(), DirectoryError>) -> Self {
        match value {
            Ok(_) => DeleteFileResult::Ok,
            Err(e) => DeleteFileResult::Err(e),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ProvisionBucketResult {
    Ok,
    Err(DirectoryError),
}
impl From<Result<(), DirectoryError>> for ProvisionBucketResult {
    fn from(value: Result<(), DirectoryError>) -> Self {
        match value {
            Ok(_) => ProvisionBucketResult::Ok,
            Err(e) => ProvisionBucketResult::Err(e),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ListBucketResult {
    Ok(Vec<Principal>),
    Err(DirectoryError),
}
impl From<Result<Vec<Principal>, DirectoryError>> for ListBucketResult {
    fn from(value: Result<Vec<Principal>, DirectoryError>) -> Self {
        match value {
            Ok(v) => ListBucketResult::Ok(v),
            Err(e) => ListBucketResult::Err(e),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum GetUploadTokensResult {
    Ok(Vec<UploadToken>),
    Err(DirectoryError),
}
impl From<Result<Vec<UploadToken>, DirectoryError>> for GetUploadTokensResult {
    fn from(value: Result<Vec<UploadToken>, DirectoryError>) -> Self {
        match value {
            Ok(v) => GetUploadTokensResult::Ok(v),
            Err(e) => GetUploadTokensResult::Err(e),
        }
    }
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum TopUpBalanceResult {
    Ok(u64), // New expiry
    Err(DirectoryError),
}
impl From<Result<u64, DirectoryError>> for TopUpBalanceResult {
    fn from(value: Result<u64, DirectoryError>) -> Self {
        match value {
            Ok(v) => TopUpBalanceResult::Ok(v),
            Err(e) => TopUpBalanceResult::Err(e),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum AdminWithdrawResult {
    Ok,
    Err(DirectoryError),
}
impl From<Result<(), DirectoryError>> for AdminWithdrawResult {
    fn from(value: Result<(), DirectoryError>) -> Self {
        match value {
            Ok(_) => AdminWithdrawResult::Ok,
            Err(e) => AdminWithdrawResult::Err(e),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum CreateShareLinkResult {
    Ok(Vec<u8>),
    Err(DirectoryError),
}
impl From<Result<Vec<u8>, DirectoryError>> for CreateShareLinkResult {
    fn from(value: Result<Vec<u8>, DirectoryError>) -> Self {
        match value {
            Ok(v) => CreateShareLinkResult::Ok(v),
            Err(e) => CreateShareLinkResult::Err(e),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum ResolveShareLinkResult {
    Ok(DownloadPlan),
    Err(DirectoryError),
}
impl From<Result<DownloadPlan, DirectoryError>> for ResolveShareLinkResult {
    fn from(value: Result<DownloadPlan, DirectoryError>) -> Self {
        match value {
            Ok(v) => ResolveShareLinkResult::Ok(v),
            Err(e) => ResolveShareLinkResult::Err(e),
        }
    }
}
