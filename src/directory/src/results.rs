use candid::{CandidType, Deserialize};
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
