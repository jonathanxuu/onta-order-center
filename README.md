# Onta Order Center

Onta Order Center is a backend canister built for the Internet Computer (IC) using Rust. It provides a simple, efficient, and secure way to store and query order information on-chain, leveraging stable structures for persistent storage.

## Features
- Store a batch of order records with duplicate detection
- Query a single order by ID
- Query all orders
- Get the total count of orders
- Admin-only batch import (in dev/prod)
- Built with Rust and IC stable structures

## Canister API

### Types
```candid
// Order information
record OrderInfo {
  order_id : text;
  location : text;
  create_time : nat64;
  currency : text;
  amount : text;
}

// Result of storing a batch of orders
variant StoreOrderListResult {
  Ok : record { stored_count : nat64; duplicate_count : nat64 };
  Err : text;
}
```

### Service Methods
- `greet(name: text) -> (text) query`  
  Returns a greeting message.

- `store_user_list(order_list: vec OrderInfo) -> (StoreOrderListResult)`  
  Store a batch of orders. In dev/prod, only the authorized admin can call this method. Returns the number of stored and duplicate orders.

- `get_order(order_id: text) -> (opt OrderInfo) query`  
  Query a single order by its ID.

- `get_all_orders() -> (vec OrderInfo) query`  
  Query all stored orders.

- `get_orders_count() -> (nat64) query`  
  Get the total number of stored orders.

## Development

### Prerequisites
- [DFINITY SDK (dfx)](https://internetcomputer.org/docs/current/developer-docs/setup/install/)
- Rust toolchain (for backend development)

### Build and Deploy
```bash
# Install dependencies (if needed)
dfx start --background

# Build the canister
dfx build

# Deploy locally
dfx deploy
```

### Project Structure
- `src/onta_order_center_backend/` - Rust backend canister source code
- `src/onta_order_center_backend/onta_order_center_backend.did` - Candid interface definition
- `dfx.json` - DFINITY project configuration

## Notes
- The batch import (`store_user_list`) is restricted to an authorized principal in dev/prod environments.
- Orders are stored using a stable BTreeMap for persistence across canister upgrades.

## License
MIT