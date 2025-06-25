use candid::{CandidType, Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, StableCell, Storable};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, cell::RefCell};
// Memory configuration
pub type Memory = VirtualMemory<DefaultMemoryImpl>;
pub const MAX_VALUE_SIZE: u32 = 400000;

#[derive(Debug, Clone, CandidType)]
pub struct StorableStr([u8; 32]);

impl StorableStr {
    pub fn new(s: &str) -> Self {
        let mut arr = [0u8; 32];
        let bytes = s.as_bytes();
        let len = bytes.len().min(32);
        arr[..len].copy_from_slice(&bytes[..len]);
        StorableStr(arr)
    }
}

impl Storable for StorableStr {
    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Borrowed(&self.0)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes[..32]);
        StorableStr(arr)
    }
}

impl PartialEq for StorableStr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for StorableStr {}

impl PartialOrd for StorableStr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StorableStr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    pub static ORDER_STORE: RefCell<StableBTreeMap<StorableStr, OrderInfo, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );

}

#[derive(CandidType, Clone, Debug, Serialize, Deserialize)]
pub struct OrderInfo {
    pub order_id: String,
    pub location: String,
    pub create_time: u64,
    pub currency: String,
    pub amount: String,
}

impl Storable for OrderInfo {
    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

#[derive(CandidType)]
pub enum StoreOrderListResult {
    Ok {
        stored_count: u64,
        duplicate_count: u64,
    },
    Err(String),
}
