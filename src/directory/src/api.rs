use candid::Principal;
use ic_cdk_macros::*;
use shared::types::*;

use crate::{billing::*, memory::*, types::*};

#[query]
pub fn get_pricing() -> String {
    format!(
        "Storage cost: {} cycles per GiB/month. Min credit to upload: {} cycles.",
        CYCLES_PER_GIB_MONTH, MIN_CREDIT_TO_START_UPLOAD
    )
}

#[query]
pub fn get_balance(user: Option<Principal>) -> u128 {
    let caller = user.unwrap_or_else(ic_cdk::caller);
    USERS.with(|u| {
        u.borrow()
            .get(&StorablePrincipal(caller))
            .map(|s| s.credit_cycles)
            .unwrap_or(0)
    })
}

#[update]
pub fn top_up_credits(amount: u128) -> u128 {
    let caller = ic_cdk::caller();
    USERS.with(|u| {
        let mut map = u.borrow_mut();
        let key = StorablePrincipal(caller);
        let mut state = map.get(&key).unwrap_or(UserState {
            used_bytes: 0,
            quota_bytes: 10 * GIB, // 10GiB default quota for v1
            credit_cycles: 0,
        });
        state.credit_cycles += amount;
        map.insert(key, state.clone());
        state.credit_cycles
    })
}

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
pub fn start_upload(name: String, size_bytes: u64) -> Result<UploadSession, String> {
    let caller = ic_cdk::caller();
    let key = StorablePrincipal(caller);

    // 1. Check Quota & Balance
    let required_credit = calculate_reservation_cost(size_bytes);
    
    let mut user_state = USERS.with(|u| {
        u.borrow().get(&key).ok_or_else(|| "User not found. Top up first.".to_string())
    })?;

    if user_state.used_bytes + size_bytes > user_state.quota_bytes {
        return Err(format!(
            "Quota exceeded. Used: {}, Requested: {}, Quota: {}",
            user_state.used_bytes, size_bytes, user_state.quota_bytes
        ));
    }

    if user_state.credit_cycles < required_credit || user_state.credit_cycles < MIN_CREDIT_TO_START_UPLOAD {
        return Err(format!(
            "Insufficient credits. Required: {}, Available: {}",
            required_credit, user_state.credit_cycles
        ));
    }

    // 2. Reserve Credits
    user_state.credit_cycles -= required_credit;
    USERS.with(|u| u.borrow_mut().insert(key, user_state));

    // 3. Create Session
    let mut id = vec![0u8; 16];
    let time_bytes = ic_cdk::api::time().to_be_bytes();
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
        expires_at_ns: ic_cdk::api::time() + 3600 * 1_000_000_000,
        reserved_credit: required_credit,
    };

    UPLOADS.with(|u| u.borrow_mut().insert(upload_id, session.clone()));

    ic_cdk::println!("Started upload for file: {}. Reserved {} credits.", name, required_credit);

    Ok(session)
}

#[update]
pub fn commit_upload(upload_id: Vec<u8>) -> Result<FileMeta, String> {
    let session = UPLOADS
        .with(|u| u.borrow_mut().remove(&upload_id))
        .ok_or_else(|| "Upload not found".to_string())?;

    // In a real system, we'd adjust the reserved credit if the final size differed
    // For v1, we assume expected == final.

    // 1. Update User Usage
    let caller = session.file_id.owner;
    let key = StorablePrincipal(caller);
    USERS.with(|u| {
        let mut map = u.borrow_mut();
        if let Some(mut state) = map.get(&key) {
            state.used_bytes += session.expected_size_bytes;
            map.insert(key, state);
        }
    });

    // 2. Create File Meta
    let meta = FileMeta {
        file_id: session.file_id.clone(),
        name: "uploaded_file".to_string(), // TODO: Store name in session
        mime: "application/octet-stream".to_string(),
        size_bytes: session.expected_size_bytes,
        chunk_size: session.chunk_size,
        chunk_count: session.expected_chunk_count,
        created_at_ns: ic_cdk::api::time(),
        updated_at_ns: ic_cdk::api::time(),
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

    let meta = FILES.with(|f| f.borrow_mut().remove(&file_id))
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
