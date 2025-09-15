// Soroban Oracle Contract (Rust)
// Minimal, auditable on-chain oracle for price feeds.
// Features:
// - set_price(asset: Symbol, price: i128, ts: u64, valid_for: u64, nonce: i128)
//   Only callable by authorized updaters.
// - get_price(asset: Symbol) -> (price: i128, ts: u64, valid_for: u64, nonce: i128)
// - authorize_updater(addr: Address) / revoke_updater(addr: Address) (admin only)
// - admin set on contract init (owner)
// - emits PriceUpdate events
// - simple historical storage (optional) and getters

use soroban_sdk::{contractimpl, Env, Symbol, Address, Vec, map, BytesN};


pub struct OracleContract;

#[contractimpl]
impl OracleContract {
    // Initialize contract owner. Called once by deployer.
    pub fn init(env: Env, owner: Address) {
        // require not initialized
        if env.storage().has(&Symbol::new(&env, "owner")) {
            panic!("already initialized");
        }
        env.storage().set(&Symbol::new(&env, "owner"), &owner);
    }

    // Authorize an updater (owner only)
    pub fn authorize_updater(env: Env, updater: Address) {
        let owner: Address = env.storage().get_unchecked(&Symbol::new(&env, "owner")).unwrap();
        if env.invoker() != owner {
            panic!("only owner can authorize updaters");
        }
        let key = (Symbol::new(&env, "updater"), updater.clone());
        env.storage().set(&key, &true);
    }

    // Revoke updater
    pub fn revoke_updater(env: Env, updater: Address) {
        let owner: Address = env.storage().get_unchecked(&Symbol::new(&env, "owner")).unwrap();
        if env.invoker() != owner {
            panic!("only owner can revoke updaters");
        }
        let key = (Symbol::new(&env, "updater"), updater.clone());
        env.storage().remove(&key);
    }

    // Check if caller is authorized updater
    fn is_authorized_updater(env: &Env, caller: &Address) -> bool {
        let key = (Symbol::new(env, "updater"), caller.clone());
        env.storage().get(&key).unwrap_or(false)
    }

    // Set price for an asset. Only updaters allowed.
    pub fn set_price(env: Env, asset: Symbol, price: i128, timestamp: u64, valid_for: u64, nonce: i128) {
        let caller = env.invoker();
        if !OracleContract::is_authorized_updater(&env, &caller) {
            panic!("caller not authorized to set price");
        }
        if price <= 0 {
            panic!("price must be positive");
        }
        // store values
        let k_price = (asset.clone(), Symbol::new(&env, "price"));
        let k_ts = (asset.clone(), Symbol::new(&env, "ts"));
        let k_valid = (asset.clone(), Symbol::new(&env, "valid_for"));
        let k_nonce = (asset.clone(), Symbol::new(&env, "nonce"));

        env.storage().set(&k_price, &price);
        env.storage().set(&k_ts, &timestamp);
        env.storage().set(&k_valid, &valid_for);
        env.storage().set(&k_nonce, &nonce);

        // Emit PriceUpdate event
        let topic = Symbol::new(&env, "PriceUpdate");
        let mut data = Vec::<(Symbol, i128)>::new(&env);
        // we emit a simple structured event as bytes: (asset, price, ts, valid_for, nonce)
        // Soroban events can be any serializable values; here we store a blob-like tuple as separate publishes
        env.events().publish((topic.clone(), asset.clone()), (price, timestamp, valid_for, nonce));
    }

    // Get price; returns (price, timestamp, valid_for, nonce)
    pub fn get_price(env: Env, asset: Symbol) -> (i128, u64, u64, i128) {
        let k_price = (asset.clone(), Symbol::new(&env, "price"));
        let k_ts = (asset.clone(), Symbol::new(&env, "ts"));
        let k_valid = (asset.clone(), Symbol::new(&env, "valid_for"));
        let k_nonce = (asset.clone(), Symbol::new(&env, "nonce"));

        let price: Option<i128> = env.storage().get(&k_price);
        let ts: Option<u64> = env.storage().get(&k_ts);
        let valid: Option<u64> = env.storage().get(&k_valid);
        let nonce: Option<i128> = env.storage().get(&k_nonce);

        match (price, ts, valid, nonce) {
            (Some(p), Some(t), Some(v), Some(n)) => (p, t, v, n),
            _ => panic!("price not available for asset"),
        }
    }

    // Getter to check if address is updater
    pub fn is_updater(env: Env, addr: Address) -> bool {
        OracleContract::is_authorized_updater(&env, &addr)
    }

    // Allow owner to change owner
    pub fn transfer_ownership(env: Env, new_owner: Address) {
        let owner: Address = env.storage().get_unchecked(&Symbol::new(&env, "owner")).unwrap();
        if env.invoker() != owner {
            panic!("only owner can transfer ownership");
        }
        env.storage().set(&Symbol::new(&env, "owner"), &new_owner);
    }
}

// NOTE: This is a minimal contract and should be extended with richer event types,
// replay protections, historical price arrays, and explicit getters that return Option types
// instead of panicking. Add unit tests and follow Soroban SDK patterns when compiling.

// ---------------------------
// Deployment & feeder (off-chain) notes (place in README):
// 1) Build and deploy contract using `soroban` CLI:
//    - `cargo build --target wasm32-unknown-unknown --release`
//    - `soroban contract deploy --wasm target/wasm32-unknown-unknown/release/<contract>.wasm`
// 2) Initialize owner (caller) with `init(owner_address)` once.
// 3) Owner calls `authorize_updater(updater_address)` to allow feeder to call set_price.
// 4) Feeder service periodically calls `set_price(asset_symbol, price, ts, valid_for, nonce)`.
//    - The feeder may be a Node.js/Python service that reads official PLD sources, formats price
//      into integer micro-units and submits transaction signed by the updater key.
// 5) Market contract will call `get_price(asset_symbol)` when performing buys/liquidations.

// ---------------------------
// Example feeder behavior:
// - query CCEE API / file for PLD
// - compute integer price (e.g., micro-USDT per token)
// - send transaction to call oracle_contract.set_price(...)
// - log tx hash and response for audit

// Security & enhancements suggestions:
// - Use multisig / committee for updates (N-of-M) or require multiple updater signatures
//   and aggregate them in contract before accepting a price.
// - Store historical prices (ring buffer) for auditability.
// - Implement `pause()` and `emergency_withdraw()` for owner.
// - Add defensive checks: max change per update, whitelist of assets, replay protection by `nonce`.
