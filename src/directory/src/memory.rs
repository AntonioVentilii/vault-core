use std::{borrow::Cow, cell::RefCell};

use candid::{decode_one, encode_one, Principal};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    storable::Bound,
    Cell as StableCell, DefaultMemoryImpl, StableBTreeMap, Storable,
};
use shared::types::{FileId, FileMeta, LinkInfo, UploadSession};

use crate::{
    config::Config,
    types::{BucketInfo, UserState},
};

type Memory = VirtualMemory<DefaultMemoryImpl>;
pub type ConfigCell = StableCell<Option<Config>, Memory>;

impl Storable for UserState {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(encode_one(self).expect("failed to encode UserState"))
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        decode_one(&bytes).expect("failed to decode UserState")
    }
}

impl Storable for BucketInfo {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(encode_one(self).expect("failed to encode BucketInfo"))
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        decode_one(&bytes).expect("failed to decode BucketInfo")
    }
}

impl Storable for Config {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(encode_one(self).expect("failed to encode Config"))
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        decode_one(&bytes).expect("failed to decode Config")
    }
}

// Wrapper for Principal to make it Storable
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StorablePrincipal(pub Principal);

impl Storable for StorablePrincipal {
    const BOUND: Bound = Bound::Bounded {
        max_size: 29,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(self.0.as_slice())
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Self(Principal::from_slice(&bytes))
    }
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub static USERS: RefCell<StableBTreeMap<StorablePrincipal, UserState, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))))
    );

    pub static FILES: RefCell<StableBTreeMap<FileId, FileMeta, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
    );

    pub static UPLOADS: RefCell<StableBTreeMap<Vec<u8>, UploadSession, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))))
    );

    pub static FILE_TO_BUCKET: RefCell<StableBTreeMap<FileId, StorablePrincipal, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))))
    );

    pub static BUCKETS: RefCell<StableBTreeMap<StorablePrincipal, BucketInfo, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))))
    );

    pub static CONFIG: RefCell<ConfigCell> = RefCell::new(
        StableCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))), None).expect("failed to init ConfigCell")
    );

    pub static LINKS: RefCell<StableBTreeMap<Vec<u8>, LinkInfo, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6))))
    );
}

pub fn read_config<R>(f: impl FnOnce(&Config) -> R) -> R {
    CONFIG.with(|cell| {
        f(cell
            .borrow()
            .get()
            .as_ref()
            .expect("config is not initialized"))
    })
}

pub fn set_config(config: Config) {
    CONFIG.with(|cell| {
        cell.borrow_mut()
            .set(Some(config))
            .expect("failed to set config");
    });
}

pub fn mutate_config(f: impl FnOnce(&mut Config)) {
    CONFIG.with(|cell| {
        let mut config = cell
            .borrow()
            .get()
            .clone()
            .expect("config is not initialized");
        f(&mut config);
        cell.borrow_mut()
            .set(Some(config))
            .expect("failed to set config");
    });
}

pub fn icp_ledger() -> Principal {
    read_config(|config| config.icp_ledger.expect("icp_ledger is not set"))
}

pub fn ckusdc_ledger() -> Principal {
    read_config(|config| config.ckusdc_ledger.expect("ckusdc_ledger is not set"))
}
