// SPDX-License-Identifier: MIT
// Compatible with Soroban SDK 22.0.0
#![no_std]

use soroban_sdk::{Address, contract, contractimpl, Env, String, Symbol};

#[contract]
pub struct EnerTrade;

#[contractimpl]
impl EnerTrade {
    /// Initialize the token contract
    pub fn __constructor(env: Env, owner: Address) {
        if env.storage().instance().has(&Symbol::new(&env, "initialized")) {
            panic!("already initialized");
        }
        
        env.storage().persistent().set(&Symbol::new(&env, "owner"), &owner);
        env.storage().instance().set(&Symbol::new(&env, "name"), &String::from_str(&env, "EnerTrade"));
        env.storage().instance().set(&Symbol::new(&env, "symbol"), &String::from_str(&env, "Ener"));
        env.storage().instance().set(&Symbol::new(&env, "decimals"), &18u32);
        env.storage().instance().set(&Symbol::new(&env, "initialized"), &true);
    }

    pub fn set_minter_addr(env: Env, addr: Address) {
        let owner: Address = env.storage().persistent().get(&Symbol::new(&env, "owner")).unwrap();
        owner.require_auth();

        env.storage().persistent().set(&Symbol::new(&env, "minter"), &addr);
    }

    /// Mint tokens (owner only)
    pub fn mint(env: Env, account: Address, amount: i128) {
        let minter: Address = env.storage().persistent().get(&Symbol::new(&env, "minter")).unwrap();
        minter.require_auth();
        
        let balance_key = (Symbol::new(&env, "balance"), account.clone());
        let current_balance: i128 = env.storage().persistent().get(&balance_key).unwrap_or(0);
        env.storage().persistent().set(&balance_key, &(current_balance + amount));
        
        let total_supply_key = Symbol::new(&env, "total_supply");
        let current_supply: i128 = env.storage().instance().get(&total_supply_key).unwrap_or(0);
        env.storage().instance().set(&total_supply_key, &(current_supply + amount));
    }

    /// Transfer tokens
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        
        let from_balance_key = (Symbol::new(&env, "balance"), from.clone());
        let to_balance_key = (Symbol::new(&env, "balance"), to.clone());
        
        let from_balance: i128 = env.storage().persistent().get(&from_balance_key).unwrap_or(0);
        if from_balance < amount {
            panic!("insufficient balance");
        }
        
        env.storage().persistent().set(&from_balance_key, &(from_balance - amount));
        
        let to_balance: i128 = env.storage().persistent().get(&to_balance_key).unwrap_or(0);
        env.storage().persistent().set(&to_balance_key, &(to_balance + amount));
    }

    /// Burn tokens
    pub fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();
        
        let balance_key = (Symbol::new(&env, "balance"), from.clone());
        let current_balance: i128 = env.storage().persistent().get(&balance_key).unwrap_or(0);
        
        if current_balance < amount {
            panic!("insufficient balance");
        }
        
        env.storage().persistent().set(&balance_key, &(current_balance - amount));
        
        let total_supply_key = Symbol::new(&env, "total_supply");
        let current_supply: i128 = env.storage().instance().get(&total_supply_key).unwrap_or(0);
        env.storage().instance().set(&total_supply_key, &(current_supply - amount));
    }

    /// Get balance
    pub fn balance(env: Env, id: Address) -> i128 {
        let balance_key = (Symbol::new(&env, "balance"), id);
        env.storage().persistent().get(&balance_key).unwrap_or(0)
    }

    /// Get token name
    pub fn name(env: Env) -> String {
        env.storage().instance().get(&Symbol::new(&env, "name")).unwrap()
    }

    /// Get token symbol
    pub fn symbol(env: Env) -> String {
        env.storage().instance().get(&Symbol::new(&env, "symbol")).unwrap()
    }

    /// Get token decimals
    pub fn decimals(env: Env) -> u32 {
        env.storage().instance().get(&Symbol::new(&env, "decimals")).unwrap()
    }

    /// Get total supply
    pub fn total_supply(env: Env) -> i128 {
        env.storage().instance().get(&Symbol::new(&env, "total_supply")).unwrap_or(0)
    }

    /// Transfer ownership (owner only)
    pub fn transfer_ownership(env: Env, new_owner: Address) {
        let owner: Address = env.storage().instance().get(&Symbol::new(&env, "owner")).unwrap();
        owner.require_auth();
        
        env.storage().instance().set(&Symbol::new(&env, "owner"), &new_owner);
    }

    /// Get current owner
    pub fn owner(env: Env) -> Address {
        env.storage().instance().get(&Symbol::new(&env, "owner")).unwrap()
    }
}