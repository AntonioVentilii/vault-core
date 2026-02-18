use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

pub type UserId = Principal;
pub type UploadId = Vec<u8>;
pub type BucketId = Principal;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileId {
    pub owner: UserId,
    pub id: Vec<u8>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum FileStatus {
    Pending,
    Ready,
    Deleted,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct FileMeta {
    pub file_id: FileId,
    pub name: String,
    pub mime: String,
    pub size_bytes: u64,
    pub chunk_size: u32,
    pub chunk_count: u32,
    pub created_at_ns: u64,
    pub updated_at_ns: u64,
    pub status: FileStatus,
    pub sha256: Option<Vec<u8>>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UploadSession {
    pub upload_id: UploadId,
    pub file_id: FileId,
    pub chunk_size: u32,
    pub expected_size_bytes: u64,
    pub expected_chunk_count: u32,
    pub uploaded_chunks: Vec<u32>,
    pub expires_at_ns: u64,
    pub reserved_credit: u128,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct DownloadPlan {
    pub chunk_count: u32,
    pub chunk_size: u32,
    pub locations: Vec<ChunkLocation>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ChunkLocation {
    pub chunk_index: u32,
    pub bucket: Principal,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UploadToken {
    pub upload_id: UploadId,
    pub file_id: FileId,
    pub bucket_id: BucketId,
    pub expires_at: u64,
    pub allowed_chunks: Vec<u32>,
    pub sig: Vec<u8>,
}
