// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Stellar Soroban Contracts ^0.4.1
#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, Address};

#[contract]
pub struct OracleContract;

#[contractimpl]
impl OracleContract {
    // Initialize contract owner. Called once by deployer.
    pub fn init(env: Env, owner: Address) {
        // require not initialized
        if env.storage().persistent().has(&Symbol::new(&env, "owner")) {
            panic!("already initialized");
        }
        env.storage().persistent().set(&Symbol::new(&env, "owner"), &owner);
    }

    // Set price for an asset. Only updaters allowed.
    pub fn set_price(env: Env, asset: Symbol, price: i128, timestamp: u64) {
        let owner: Address = env.storage().persistent().get(&Symbol::new(&env, "owner")).unwrap();
        
        // Exige que a chamada tenha sido autenticada pelo owner
        owner.require_auth();
        if price <= 0 {
            panic!("price must be positive");
        }

        // store values
        let k_price = (asset.clone(), Symbol::new(&env, "price"));
        let k_ts = (asset.clone(), Symbol::new(&env, "ts"));

        env.storage().persistent().set(&k_price, &price);
        env.storage().persistent().set(&k_ts, &timestamp);

        // Emit PriceUpdate event
        let topic = Symbol::new(&env, "PriceUpdate");
    
        // we emit a simple structured event as bytes: (asset, price, ts, valid_for, nonce)
        // Soroban events can be any serializable values; here we store a blob-like tuple as separate publishes
        env.events().publish((topic.clone(), asset.clone()), (price, timestamp));
    }

    // Get price; returns (price, timestamp, valid_for, nonce)
    pub fn get_price(env: Env, asset: Symbol) -> (i128, u64) {
        let k_price = (asset.clone(), Symbol::new(&env, "price"));
        let k_ts = (asset.clone(), Symbol::new(&env, "ts"));

        let price: Option<i128> = env.storage().persistent().get(&k_price);
        let ts: Option<u64> = env.storage().persistent().get(&k_ts);

        match (price, ts) {
            (Some(p), Some(t)) => (p, t),
            _ => panic!("price not available for asset"),
        }
    }

    // Allow owner to change owner
    pub fn transfer_ownership(env: Env, new_owner: Address) {
        let owner: Address = env.storage().persistent().get(&Symbol::new(&env, "owner")).unwrap();
        owner.require_auth();

        env.storage().persistent().set(&Symbol::new(&env, "owner"), &new_owner);
    }
 
}