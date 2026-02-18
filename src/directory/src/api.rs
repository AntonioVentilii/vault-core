use candid::Principal;
use ic_cdk::{api::time, id, println};
use ic_cdk_macros::{query, update};
use ic_papi_api::PaymentType;
use shared::{
    auth::sign_token,
    types::{FileId, FileMeta, FileStatus, UploadSession, UploadToken},
};

use crate::{
    memory::{StorablePrincipal, BUCKETS, FILES, FILE_TO_BUCKET, UPLOADS, USERS},
    payments::{SignerMethods, PAYMENT_GUARD},
    types::{BucketInfo, UserState},
};

const GIB: u64 = 1024 * 1024 * 1024;

#[query]
pub fn get_pricing() -> String {
    "PAPI enabled. Pricing is determined by the PaymentGuard configuration.".to_string()
}

// get_balance and top_up_credits are removed as we use PAPI/Ledger for balances

#[query]
pub fn get_usage(user: Option<Principal>) -> UserState {
    let caller = user.unwrap_or_else(ic_cdk::caller);
    USERS.with(|u| {
        u.borrow()
            .get(&StorablePrincipal(caller))
            .unwrap_or_default()
    })
}

#[query]
pub fn list_files() -> Vec<FileMeta> {
    let caller = ic_cdk::caller();
    FILES.with(|f| {
        f.borrow()
            .iter()
            .filter(|(fid, _)| fid.owner == caller)
            .map(|(_, meta)| meta)
            .collect()
    })
}

#[update]
pub async fn start_upload(
    name: String,
    size_bytes: u64,
    payment: Option<PaymentType>,
) -> Result<UploadSession, String> {
    let caller = ic_cdk::caller();
    let key = StorablePrincipal(caller);

    // 1. PAPI Payment Deduction
    PAYMENT_GUARD
        .deduct(
            payment.unwrap_or(PaymentType::AttachedCycles),
            SignerMethods::StartUpload.fee(),
        )
        .await
        .map_err(|e| format!("Payment failed: {:?}", e))?;

    // 2. Check Quota
    let user_state = USERS.with(|u| {
        u.borrow().get(&key).unwrap_or(UserState {
            used_bytes: 0,
            quota_bytes: 10 * 1024 * 1024 * 1024, // 10GiB default
        })
    });

    if user_state.used_bytes + size_bytes > user_state.quota_bytes {
        return Err(format!(
            "Quota exceeded. Used: {}, Requested: {}, Quota: {}",
            user_state.used_bytes, size_bytes, user_state.quota_bytes
        ));
    }

    // 3. Create Session
    let mut id = vec![0u8; 16];
    let time_bytes = time().to_be_bytes();
    id[..8].copy_from_slice(&time_bytes);

    let file_id = FileId {
        owner: caller,
        id: id.clone(),
    };
    let upload_id = id;

    let session = UploadSession {
        upload_id: upload_id.clone(),
        file_id,
        chunk_size: 1024 * 1024,
        expected_size_bytes: size_bytes,
        expected_chunk_count: ((size_bytes + 1024 * 1024 - 1) / (1024 * 1024)) as u32,
        uploaded_chunks: vec![],
        expires_at_ns: time() + 3600 * 1_000_000_000,
    };

    UPLOADS.with(|u| u.borrow_mut().insert(upload_id, session.clone()));

    println!("Started upload for file: {}.", name);

    Ok(session)
}

#[update]
pub fn report_chunk_uploaded(upload_id: Vec<u8>, chunk_index: u32) -> Result<(), String> {
    UPLOADS.with(|u| {
        let mut map = u.borrow_mut();
        if let Some(mut session) = map.get(&upload_id) {
            if !session.uploaded_chunks.contains(&chunk_index) {
                session.uploaded_chunks.push(chunk_index);
                map.insert(upload_id, session);
            }
            Ok(())
        } else {
            Err("Upload session not found".to_string())
        }
    })
}

#[update]
pub fn commit_upload(upload_id: Vec<u8>) -> Result<FileMeta, String> {
    let session = UPLOADS
        .with(|u| u.borrow().get(&upload_id))
        .ok_or_else(|| "Upload not found".to_string())?;

    // 1. Verify Completion
    if session.uploaded_chunks.len() < session.expected_chunk_count as usize {
        return Err(format!(
            "Upload incomplete. Received {}/{} chunks.",
            session.uploaded_chunks.len(),
            session.expected_chunk_count
        ));
    }

    // Success - remove session
    UPLOADS.with(|u| u.borrow_mut().remove(&upload_id));

    // 2. Update User Usage
    let caller = session.file_id.owner;
    let key = StorablePrincipal(caller);
    USERS.with(|u| {
        let mut map = u.borrow_mut();
        if let Some(mut state) = map.get(&key) {
            state.used_bytes += session.expected_size_bytes;
            map.insert(key, state);
        }
    });

    // 3. Create File Meta
    let meta = FileMeta {
        file_id: session.file_id.clone(),
        name: "uploaded_file".to_string(), // TODO: Store name in session
        mime: "application/octet-stream".to_string(),
        size_bytes: session.expected_size_bytes,
        chunk_size: session.chunk_size,
        chunk_count: session.expected_chunk_count,
        created_at_ns: time(),
        updated_at_ns: time(),
        status: FileStatus::Ready,
        sha256: None,
    };

    FILES.with(|f| f.borrow_mut().insert(session.file_id, meta.clone()));

    Ok(meta)
}

#[update]
pub fn delete_file(file_id: FileId) -> Result<(), String> {
    if file_id.owner != ic_cdk::caller() {
        return Err("Unauthorized".to_string());
    }

    let meta = FILES
        .with(|f| f.borrow_mut().remove(&file_id))
        .ok_or_else(|| "File not found".to_string())?;

    // Refund/Update usage
    let key = StorablePrincipal(file_id.owner);
    USERS.with(|u| {
        let mut map = u.borrow_mut();
        if let Some(mut state) = map.get(&key) {
            state.used_bytes = state.used_bytes.saturating_sub(meta.size_bytes);
            map.insert(key, state);
        }
    });

    Ok(())
}

// Shared secret for v1 (to be improved in later phases)
const SHARED_SECRET: &[u8] = b"v1_shared_secret_for_vault_core";

#[update]
pub fn provision_bucket(bucket_id: Principal) -> Result<(), String> {
    // In a real system, only admins can provision buckets
    BUCKETS.with(|b| {
        let mut map = b.borrow_mut();
        map.insert(
            StorablePrincipal(bucket_id),
            BucketInfo {
                id: bucket_id,
                writable: true,
                used_bytes: 0,
                soft_limit_bytes: 100 * GIB, // 100GiB soft limit per bucket for v1
                hard_limit_bytes: 105 * GIB,
            },
        );
    });
    Ok(())
}

#[update]
pub fn get_upload_tokens(upload_id: Vec<u8>, chunks: Vec<u32>) -> Result<Vec<UploadToken>, String> {
    let session = UPLOADS
        .with(|u| u.borrow().get(&upload_id))
        .ok_or_else(|| "Upload session not found".to_string())?;

    // Auth check: caller must be owner
    if session.file_id.owner != ic_cdk::caller() {
        return Err("Unauthorized".to_string());
    }

    // Pick a bucket (strategy: use first writable bucket for v1)
    let bucket_id = BUCKETS
        .with(|b| {
            b.borrow()
                .iter()
                .find(|(_, info)| info.writable)
                .map(|(_, info)| info.id)
        })
        .ok_or_else(|| "No writable buckets available".to_string())?;

    // Record the assignment
    FILE_TO_BUCKET.with(|ftb| {
        ftb.borrow_mut()
            .insert(session.file_id.clone(), StorablePrincipal(bucket_id))
    });

    // Issue tokens. For v1 we can batch all chunks into one token or one per chunk.
    // Let's do batch for efficiency if chunks are provided.
    let mut token = UploadToken {
        upload_id: session.upload_id.clone(),
        file_id: session.file_id.clone(),
        bucket_id,
        directory_id: id(),
        expires_at: session.expires_at_ns,
        allowed_chunks: chunks,
        sig: vec![],
    };

    sign_token(&mut token, SHARED_SECRET);

    Ok(vec![token])
}
