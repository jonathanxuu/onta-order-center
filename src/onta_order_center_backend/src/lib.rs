mod memory;
use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};
use memory::{OrderInfo, StorableStr, StoreOrderListResult, ORDER_STORE};
const MAX_ORDER_PER_BATCH: usize = 200;

const AUTHORIZED_PRINCIPAL: &str =
    "nlptv-oct5y-qmful-kkutn-zghyv-npwrx-usyby-r7g3u-k4k44-3rq4o-mqe"; // The principal of the authorized admin who can change the rpc urls

#[ic_cdk::update]
fn store_user_list(order_list: Vec<OrderInfo>) -> StoreOrderListResult {
    // NOTES: add Operator check, only the operator can call this function to add user list, in dev and prod environment
    #[cfg(any(feature = "dev", feature = "prod"))]
    {
        let caller = ic_cdk::caller();
        let authorized_principal = match Principal::from_text(AUTHORIZED_PRINCIPAL) {
            Ok(principal) => principal,
            Err(_) => return StoreOrderListResult::Err("Invalid authorized principal".to_string()),
        };

        if caller != authorized_principal {
            return StoreOrderListResult::Err(
                "Unauthorized: only the admin can call this method".to_string(),
            );
        }
    }

    // Check if number of orders exceeds the limit
    if order_list.len() > MAX_ORDER_PER_BATCH {
        return StoreOrderListResult::Err(format!(
            "Too many users in batch. Maximum allowed is {}",
            MAX_ORDER_PER_BATCH
        ));
    }

    // Ensure not empty
    if order_list.is_empty() {
        return StoreOrderListResult::Err("Order list cannot be empty".to_string());
    }

    // Store orders in the stable storage
    let mut stored_count = 0u64;
    let mut duplicate_count = 0u64;

    ORDER_STORE.with(|store| {
        let mut store = store.borrow_mut();

        for order in order_list {
            let key = StorableStr::new(&order.order_id);

            if store.contains_key(&key) {
                duplicate_count += 1;
            } else {
                store.insert(key, order);
                stored_count += 1;
            }
        }
    });

    StoreOrderListResult::Ok {
        stored_count,
        duplicate_count,
    }
}

#[ic_cdk::query]
fn get_order(order_id: String) -> Option<OrderInfo> {
    let key = StorableStr::new(&order_id);
    ORDER_STORE.with(|store| {
        let store = store.borrow();
        store.get(&key)
    })
}

#[ic_cdk::query]
fn get_all_orders() -> Vec<OrderInfo> {
    ORDER_STORE.with(|store| {
        let store = store.borrow();
        store.iter().map(|(_, order)| order).collect()
    })
}

#[ic_cdk::query]
fn get_orders_count() -> u64 {
    ORDER_STORE.with(|store| {
        let store = store.borrow();
        store.len() as u64
    })
}

#[ic_cdk::update]
fn delete_order(order_ids: Vec<String>) -> Result<Vec<String>, String> {
    // NOTES: add Operator check, only the operator can call this function to delete orders, in dev and prod environment
    #[cfg(any(feature = "dev", feature = "prod"))]
    {
        let caller = ic_cdk::caller();
        let authorized_principal = match Principal::from_text(AUTHORIZED_PRINCIPAL) {
            Ok(principal) => principal,
            Err(_) => return Err("Invalid authorized principal".to_string()),
        };

        if caller != authorized_principal {
            return Err("Unauthorized: only the admin can call this method".to_string());
        }
    }

    // Check if order_ids is not empty
    if order_ids.is_empty() {
        return Err("Order IDs list cannot be empty".to_string());
    }

    let mut deleted_order_ids = Vec::new();

    ORDER_STORE.with(|store| {
        let mut store = store.borrow_mut();

        for order_id in order_ids {
            // Skip empty order IDs
            if order_id.is_empty() {
                continue;
            }

            let key = StorableStr::new(&order_id);
            if store.remove(&key).is_some() {
                deleted_order_ids.push(order_id);
            }
        }
    });

    if deleted_order_ids.is_empty() {
        Err("No orders were found and deleted".to_string())
    } else {
        Ok(deleted_order_ids)
    }
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}! Welcome to Onta Order Center", name)
}
