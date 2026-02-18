use candid::Principal;
use ic_cdk_macros::*;
use shared::types::*;

use crate::{memory::*, types::*};

#[query]
pub fn get_pricing() -> String {
    "Pricing: 100 cycles per GiB/month".to_string()
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
    // Skeleton implementation
    let caller = ic_cdk::caller();

    // Generate a simple ID based on time for the skeleton
    let mut id = vec![0u8; 16];
    let time_bytes = ic_cdk::api::time().to_be_bytes();
    id[..8].copy_from_slice(&time_bytes);

    let file_id = FileId {
        owner: caller,
        id: id.clone(),
    };
    let upload_id = id; // Use same ID for upload session for simplicity in skeleton

    let session = UploadSession {
        upload_id: upload_id.clone(),
        file_id,
        chunk_size: 1024 * 1024,
        expected_size_bytes: size_bytes,
        expected_chunk_count: ((size_bytes + 1024 * 1024 - 1) / (1024 * 1024)) as u32,
        uploaded_chunks: vec![],
        expires_at_ns: ic_cdk::api::time() + 3600 * 1_000_000_000,
        reserved_credit: 0,
    };

    UPLOADS.with(|u| u.borrow_mut().insert(upload_id, session.clone()));

    ic_cdk::println!("Started upload for file: {}", name);

    Ok(session)
}

#[update]
pub fn commit_upload(upload_id: Vec<u8>) -> Result<FileMeta, String> {
    let session = UPLOADS
        .with(|u| u.borrow_mut().remove(&upload_id))
        .ok_or_else(|| "Upload not found".to_string())?;

    let meta = FileMeta {
        file_id: session.file_id.clone(),
        name: "uploaded_file".to_string(), // In real impl, use name from start_upload
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
    FILES.with(|f| f.borrow_mut().remove(&file_id));
    Ok(())
}
