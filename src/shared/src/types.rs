use std::borrow::Cow;

use candid::{decode_one, encode_one, CandidType, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

pub type UserId = Principal;
pub type UploadId = Vec<u8>;
pub type BucketId = Principal;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileId {
    pub owner: UserId,
    pub id: Vec<u8>,
}

impl Storable for FileId {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(encode_one(self).expect("failed to encode FileId"))
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        decode_one(&bytes).expect("failed to decode FileId")
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum FileStatus {
    Pending,
    Ready,
    Deleted,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum FileRole {
    Reader,
    Writer,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct PricingConfig {
    pub rate_per_gb_per_month: u64,
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
    pub readers: Vec<UserId>,
    pub writers: Vec<UserId>,
}

impl Storable for FileMeta {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(encode_one(self).expect("failed to encode FileMeta"))
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        decode_one(&bytes).expect("failed to decode FileMeta")
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UploadSession {
    pub upload_id: UploadId,
    pub file_id: FileId,
    pub name: String,
    pub mime: String,
    pub chunk_size: u32,
    pub expected_size_bytes: u64,
    pub expected_chunk_count: u32,
    pub uploaded_chunks: Vec<u32>,
    pub expires_at_ns: u64,
}

impl Storable for UploadSession {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(encode_one(self).expect("failed to encode UploadSession"))
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        decode_one(&bytes).expect("failed to decode UploadSession")
    }
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
    pub directory_id: Principal,
    pub expires_at: u64,
    pub allowed_chunks: Vec<u32>,
    pub sig: Vec<u8>,
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Icrc1Account {
    pub owner: Principal,
    pub subaccount: Option<Vec<u8>>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Icrc1TransferArgs {
    pub from_subaccount: Option<Vec<u8>>,
    pub to: Icrc1Account,
    pub amount: candid::Nat,
    pub fee: Option<candid::Nat>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum Icrc1TransferResult {
    Ok(candid::Nat),
    Err(Icrc1TransferError),
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub enum Icrc1TransferError {
    BadFee {
        expected_fee: candid::Nat,
    },
    BadBurn {
        min_burn_amount: candid::Nat,
    },
    InsufficientFunds {
        balance: candid::Nat,
    },
    TooOld,
    CreatedInFuture {
        ledger_time: u64,
    },
    Duplicate {
        duplicate_of: candid::Nat,
    },
    TemporarilyUnavailable,
    GenericError {
        error_code: candid::Nat,
        message: String,
    },
}
