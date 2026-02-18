use ic_cdk::{query, update};
use shared::{FileId, UploadToken};

use crate::{
    memory::CHUNKS,
    types::{ChunkKey, ChunkValue},
};

#[update]
pub fn put_chunk(token: UploadToken, chunk_index: u32, bytes: Vec<u8>) -> Result<u32, String> {
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
