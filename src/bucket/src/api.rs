use candid::Principal;
use ic_cdk::{api::time, call, eprintln, id, query, spawn, update};
use ic_papi_api::PaymentType;
use shared::{
    auth::{verify_download_token, verify_token},
    types::{DownloadToken, FileId, UploadToken},
    CanisterStatus,
};

use crate::{
    errors::BucketError,
    memory::CHUNKS,
    payments::{SignerMethods, PAYMENT_GUARD},
    results::{AdminWithdrawResult, DeleteFileResult, GetChunkResult, PutChunkResult},
    types::{ChunkKey, ChunkValue},
    AdminSetReadOnlyResult,
};

#[update]
pub async fn put_chunk(
    token: UploadToken,
    chunk_index: u32,
    bytes: Vec<u8>,
    payment: Option<PaymentType>,
) -> PutChunkResult {
    let result: Result<u32, BucketError> = async {
        let ptype = payment.unwrap_or(PaymentType::AttachedCycles);
        // 1. PAPI Payment Deduction
        PAYMENT_GUARD
            .deduct(ptype.clone(), SignerMethods::PutChunk.fee(&ptype))
            .await
            .map_err(|e| BucketError::PaymentFailed(format!("Payment failed: {:?}", e)))?;

        // 2. Check Read-Only Mode
        if crate::memory::read_config(|c| c.read_only.unwrap_or(false)) {
            return Err(BucketError::ReadOnly);
        }

        // 3. Verify Token Signature
        let secret = crate::memory::read_config(|c| c.shared_secret.clone().unwrap_or_default());
        if !verify_token(&token, &secret) {
            return Err(BucketError::InvalidSignature);
        }

        // 2. Verify Expiry
        if token.expires_at < time() {
            return Err(BucketError::TokenExpired);
        }

        // 3. Verify Bucket ID (token must be for THIS bucket)
        if token.bucket_id != id() {
            return Err(BucketError::WrongBucket);
        }

        // 4. Verify Chunk Index
        if !token.allowed_chunks.contains(&chunk_index) {
            return Err(BucketError::ChunkNotAllowed(chunk_index));
        }

        let file_id = &token.file_id;
        let owner_bytes = file_id.owner.as_slice();
        let mut owner = [0u8; 29];
        owner[..owner_bytes.len()].copy_from_slice(owner_bytes);

        let mut fid = [0u8; 16];
        if file_id.id.len() != 16 {
            return Err(BucketError::InvalidFileId);
        }
        fid.copy_from_slice(&file_id.id);

        let key = ChunkKey {
            owner,
            owner_len: owner_bytes.len() as u8,
            file_id: fid,
            chunk_index,
        };

        let size = bytes.len() as u32;
        CHUNKS.with(|c| {
            c.borrow_mut().insert(key, ChunkValue(bytes));
        });

        // 5. Notify Directory (Async)
        let directory_id = token.directory_id;
        let upload_id = token.upload_id.clone();

        spawn(async move {
            // Ignore the response type using candid::Reserved
            let res: Result<(candid::Reserved,), _> = call(
                directory_id,
                "report_chunk_uploaded",
                (upload_id, chunk_index),
            )
            .await;
            if let Err((code, msg)) = res {
                eprintln!(
                    "Failed to report chunk upload to directory: {:?} {}",
                    code, msg
                );
            }
        });

        Ok(size)
    }
    .await;

    result.into()
}

#[query]
pub fn get_chunk(token: DownloadToken, chunk_index: u32) -> GetChunkResult {
    let result: Result<Vec<u8>, BucketError> = (|| {
        // 1. Verify Token Signature
        let secret = crate::memory::read_config(|c| c.shared_secret.clone().unwrap_or_default());
        if !verify_download_token(&token, &secret) {
            return Err(BucketError::InvalidSignature);
        }

        // 2. Verify Expiry
        if token.expires_at < time() {
            return Err(BucketError::TokenExpired);
        }

        // 3. Verify Bucket ID (token must be for THIS bucket)
        if token.bucket_id != id() {
            return Err(BucketError::WrongBucket);
        }

        let file_id = &token.file_id;
        let owner_bytes = file_id.owner.as_slice();
        let mut owner = [0u8; 29];
        owner[..owner_bytes.len()].copy_from_slice(owner_bytes);

        let mut fid = [0u8; 16];
        if file_id.id.len() != 16 {
            return Err(BucketError::InvalidFileId);
        }
        fid.copy_from_slice(&file_id.id);

        let key = ChunkKey {
            owner,
            owner_len: owner_bytes.len() as u8,
            file_id: fid,
            chunk_index,
        };

        CHUNKS.with(|c| {
            c.borrow()
                .get(&key)
                .map(|v| v.0.clone())
                .ok_or(BucketError::ChunkNotFound)
        })
    })();

    result.into()
}

#[update]
pub fn delete_file(file_id: FileId) -> DeleteFileResult {
    let result: Result<(), BucketError> = (|| {
        let owner_bytes = file_id.owner.as_slice();
        let mut owner_fixed = [0u8; 29];
        owner_fixed[..owner_bytes.len()].copy_from_slice(owner_bytes);

        let mut fid = [0u8; 16];
        if file_id.id.len() != 16 {
            return Err(BucketError::InvalidFileId);
        }
        fid.copy_from_slice(&file_id.id);

        let start_key = ChunkKey {
            owner: owner_fixed,
            owner_len: owner_bytes.len() as u8,
            file_id: fid,
            chunk_index: 0,
        };

        CHUNKS.with(|c| {
            let mut chunk_map = c.borrow_mut();
            let keys_to_delete: Vec<ChunkKey> = chunk_map
                .range(start_key..)
                .take_while(|(k, _)| {
                    k.owner == owner_fixed
                        && k.owner_len == owner_bytes.len() as u8
                        && k.file_id == fid
                })
                .map(|(k, _)| k.clone())
                .collect();

            for k in keys_to_delete {
                chunk_map.remove(&k);
            }
        });

        Ok(())
    })();

    result.into()
}

#[query]
pub fn stat() -> String {
    format!("Chunks stored: {}", CHUNKS.with(|c| c.borrow().len()))
}

fn is_admin(caller: Principal) -> bool {
    if ic_cdk::api::is_controller(&caller) {
        return true;
    }
    crate::memory::read_config(|c| {
        c.admins
            .as_ref()
            .map(|a| a.contains(&caller))
            .unwrap_or(false)
    })
}

#[update]
pub async fn admin_withdraw(ledger: Principal, amount: u64, to: Principal) -> AdminWithdrawResult {
    let result: Result<(), BucketError> = async {
        if !is_admin(ic_cdk::caller()) {
            return Err(BucketError::AdminOnly);
        }

        // ICRC-1 transfer call
        let arg = shared::types::Icrc1TransferArgs {
            from_subaccount: None,
            to: shared::types::Icrc1Account {
                owner: to,
                subaccount: None,
            },
            amount: amount.into(),
            fee: None,
            memo: None,
            created_at_time: None,
        };

        let res: Result<(shared::types::Icrc1TransferResult,), _> =
            ic_cdk::call(ledger, "icrc1_transfer", (arg,)).await;

        match res {
            Ok((shared::types::Icrc1TransferResult::Ok(_),)) => Ok(()),
            Ok((shared::types::Icrc1TransferResult::Err(e),)) => {
                Err(BucketError::PaymentFailed(format!("ICRC1 error: {:?}", e)))
            }
            Err((code, msg)) => Err(BucketError::PaymentFailed(format!(
                "Call error: {:?} {}",
                code, msg
            ))),
        }
    }
    .await;
    result.into()
}

#[update]
pub fn admin_set_read_only(read_only: bool) -> AdminSetReadOnlyResult {
    let result: Result<(), BucketError> = (|| {
        if !is_admin(ic_cdk::caller()) {
            return Err(BucketError::AdminOnly);
        }
        crate::memory::mutate_config(|c| c.read_only = Some(read_only));
        Ok(())
    })();

    result.into()
}

#[query]
pub fn get_status() -> CanisterStatus {
    CanisterStatus {
        version: env!("CARGO_PKG_VERSION").to_string(),
        cycles_balance: ic_cdk::api::canister_balance128(),
        memory_usage_bytes: ic_cdk::api::stable::stable64_size() * 64 * 1024,
        heap_memory_usage_bytes: 0, // Simplified for now
    }
}
