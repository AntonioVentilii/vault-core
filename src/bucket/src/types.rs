use std::borrow::Cow;

use ic_stable_structures::{storable::Bound, Storable};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChunkKey {
    pub owner: [u8; 29], // Max principal length
    pub owner_len: u8,
    pub file_id: [u8; 16],
    pub chunk_index: u32,
}

impl Storable for ChunkKey {
    const BOUND: Bound = Bound::Bounded {
        max_size: 29 + 1 + 16 + 4,
        is_fixed_size: true,
    };

    fn to_bytes(&self) -> Cow<'_, [u8]> {
        let mut bytes = Vec::with_capacity(29 + 1 + 16 + 4);
        bytes.extend_from_slice(&self.owner);
        bytes.push(self.owner_len);
        bytes.extend_from_slice(&self.file_id);
        bytes.extend_from_slice(&self.chunk_index.to_be_bytes());
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        let mut owner = [0u8; 29];
        owner.copy_from_slice(&bytes[0..29]);
        let owner_len = bytes[29];
        let mut file_id = [0u8; 16];
        file_id.copy_from_slice(&bytes[30..46]);
        let mut chunk_index_bytes = [0u8; 4];
        chunk_index_bytes.copy_from_slice(&bytes[46..50]);
        let chunk_index = u32::from_be_bytes(chunk_index_bytes);
        Self {
            owner,
            owner_len,
            file_id,
            chunk_index,
        }
    }
}

// Wrapper for blob because Vec<u8> doesn't implement Storable
pub struct ChunkValue(pub Vec<u8>);

impl Storable for ChunkValue {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(&self.0)
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Self(bytes.into_owned())
    }
}
