use std::cell::RefCell;

use candid::{decode_one, encode_one, Principal};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    Cell as StableCell, DefaultMemoryImpl, StableBTreeMap, Storable,
};

use crate::{
    config::Config,
    types::{ChunkKey, ChunkValue},
};

// Wrapper for Principal to make it Storable
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StorablePrincipal(pub Principal);

impl Storable for StorablePrincipal {
    const BOUND: ic_stable_structures::storable::Bound =
        ic_stable_structures::storable::Bound::Bounded {
            max_size: 29,
            is_fixed_size: false,
        };

    fn to_bytes(&self) -> std::borrow::Cow<'_, [u8]> {
        std::borrow::Cow::Borrowed(self.0.as_slice())
    }

    fn from_bytes(bytes: std::borrow::Cow<'_, [u8]>) -> Self {
        Self(Principal::from_slice(&bytes))
    }
}

type Memory = VirtualMemory<DefaultMemoryImpl>;
pub type ConfigCell = StableCell<Option<Config>, Memory>;

impl Storable for Config {
    const BOUND: ic_stable_structures::storable::Bound =
        ic_stable_structures::storable::Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<'_, [u8]> {
        std::borrow::Cow::Owned(encode_one(self).expect("failed to encode Config"))
    }

    fn from_bytes(bytes: std::borrow::Cow<'_, [u8]>) -> Self {
        decode_one(&bytes).expect("failed to decode Config")
    }
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub static CHUNKS: RefCell<StableBTreeMap<ChunkKey, ChunkValue, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))))
    );

    pub static CONFIG: RefCell<ConfigCell> = RefCell::new(
        StableCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), None).expect("failed to init ConfigCell")
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
