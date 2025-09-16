// SPDX-License-Identifier: MIT
// Compatible with Soroban SDK 22.0.0
#![no_std]

use soroban_sdk::{Address, contract, contractimpl, Env, Symbol, IntoVal};

pub enum Error {
    SymbolNaoDefinido = 1,
    OracleNaoDefinido = 2,
}

#[contract]
pub struct AmbarMarketplace;

#[contractimpl]
impl AmbarMarketplace {
    
    pub fn initialize(env: Env, owner: Address) {
        if env.storage().persistent().has(&Symbol::new(&env, "owner")) {
            panic!("already initialized");
        }
        
        env.storage().persistent().set(&Symbol::new(&env, "owner"), &owner);
    }

    pub fn set_oracle_contract(env: Env, oracle: Address) {
        let owner: Address = env.storage().persistent().get(&Symbol::new(&env, "owner")).unwrap();
        owner.require_auth();

        env.storage().persistent().set(&Symbol::new(&env, "oracle"), &oracle);
    }

    pub fn set_usdt_contract(env: Env, usdt: Address) {
        let owner: Address = env.storage().persistent().get(&Symbol::new(&env, "owner")).unwrap();
        owner.require_auth();

        env.storage().persistent().set(&Symbol::new(&env, "usdt"), &usdt);
    }

    pub fn set_contract(env: Env, symbol: Symbol, token: Address) {
        let owner: Address = env.storage().persistent().get(&Symbol::new(&env, "owner")).unwrap();
        owner.require_auth();

        env.storage().persistent().set(&symbol, &token);
    }

    pub fn mint_with_usdt(env: Env, user: Address, symbol: Symbol, usdt_amount: i128) {
        let token_contract: Address = env.storage()
            .persistent()
            .get(&symbol)
            .expect("Token nao definido");

        let usdt_contract: Address = env.storage()
            .persistent()
            .get(&Symbol::new(&env, "usdt"))
            .expect("Contrato USDT nao definido");
        
        env.current_contract_address().require_auth();

        let _: () = env.invoke_contract(
            &usdt_contract,
            &Symbol::new(&env, "transfer_from"),
            soroban_sdk::vec![
                &env,
                user.clone().into_val(&env),
                env.current_contract_address().into_val(&env),
                env.current_contract_address().into_val(&env),
                usdt_amount.into_val(&env),
            ],
        );

        let oracle_contract: Address = env.storage()
            .persistent()
            .get(&Symbol::new(&env, "oracle"))
            .expect("Contrato USDT nao definido");
        
        let (price, _timestamp): (i128, u64) = env.invoke_contract(
                &oracle_contract,
                &Symbol::new(&env, "get_price"),
                soroban_sdk::vec![&env, symbol.clone().into_val(&env)],
            );

        if price <= 0 {
            panic!("invalid price from oracle");
        }

        let tokens_to_mint = usdt_amount / price;

        let _: () = env.invoke_contract(
            &token_contract,
            &Symbol::new(&env, "mint"),
            soroban_sdk::vec![&env, user.clone().into_val(&env), tokens_to_mint.into_val(&env)],
        );

        env.events().publish(
            (Symbol::new(&env, "MintedTokens"), user),
            (usdt_amount, tokens_to_mint),
        );
    }
}
