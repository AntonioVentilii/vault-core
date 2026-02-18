use ic_cdk::{api::time, call, eprintln, id, query, spawn, update};
use ic_papi_api::PaymentType;
use shared::{
    auth::verify_token,
    types::{FileId, UploadToken},
};

use crate::{
    memory::CHUNKS,
    payments::{SignerMethods, PAYMENT_GUARD},
    types::{ChunkKey, ChunkValue},
};

const SHARED_SECRET: &[u8] = b"v1_shared_secret_for_vault_core";

#[update]
pub async fn put_chunk(
    token: UploadToken,
    chunk_index: u32,
    bytes: Vec<u8>,
    payment: Option<PaymentType>,
) -> Result<u32, String> {
    // 1. PAPI Payment Deduction
    PAYMENT_GUARD
        .deduct(
            payment.unwrap_or(PaymentType::AttachedCycles),
            SignerMethods::PutChunk.fee(),
        )
        .await
        .map_err(|e| format!("Payment failed: {:?}", e))?;

    // 2. Verify Token Signature
    if !verify_token(&token, SHARED_SECRET) {
        return Err("Invalid upload token signature".to_string());
    }

    // 2. Verify Expiry
    if token.expires_at < time() {
        return Err("Upload token expired".to_string());
    }

    // 3. Verify Bucket ID (token must be for THIS bucket)
    if token.bucket_id != id() {
        return Err("Upload token issued for another bucket".to_string());
    }

    // 4. Verify Chunk Index
    if !token.allowed_chunks.contains(&chunk_index) {
        return Err(format!("Chunk index {} not allowed by token", chunk_index));
    }

    let file_id = &token.file_id;
    let owner_bytes = file_id.owner.as_slice();
    let mut owner = [0u8; 29];
    owner[..owner_bytes.len()].copy_from_slice(owner_bytes);

    let mut fid = [0u8; 16];
    if file_id.id.len() != 16 {
        return Err("Invalid file_id length".to_string());
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
        let res: Result<(Result<(), String>,), _> = call(
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

#[query]
pub fn get_chunk(file_id: FileId, chunk_index: u32) -> Result<Vec<u8>, String> {
    let owner_bytes = file_id.owner.as_slice();
    let mut owner = [0u8; 29];
    owner[..owner_bytes.len()].copy_from_slice(owner_bytes);

    let mut fid = [0u8; 16];
    if file_id.id.len() != 16 {
        return Err("Invalid file_id length".to_string());
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
            .ok_or_else(|| "Chunk not found".to_string())
    })
}

#[update]
pub fn delete_file(file_id: FileId) -> Result<(), String> {
    let owner_bytes = file_id.owner.as_slice();
    let mut owner_fixed = [0u8; 29];
    owner_fixed[..owner_bytes.len()].copy_from_slice(owner_bytes);

    let mut fid = [0u8; 16];
    if file_id.id.len() != 16 {
        return Err("Invalid file_id length".to_string());
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
                k.owner == owner_fixed && k.owner_len == owner_bytes.len() as u8 && k.file_id == fid
            })
            .map(|(k, _)| k.clone())
            .collect();

        for k in keys_to_delete {
            chunk_map.remove(&k);
        }
    });

    Ok(())
}

#[query]
pub fn stat() -> String {
    format!("Chunks stored: {}", CHUNKS.with(|c| c.borrow().len()))
}
