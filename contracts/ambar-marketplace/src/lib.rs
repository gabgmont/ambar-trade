// SPDX-License-Identifier: MIT
// Compatible with Soroban SDK 22.0.0
#![no_std]

use soroban_sdk::{Address, contract, contractimpl, Env, Symbol, IntoVal, String, token};

pub enum Error {
    SymbolNaoDefinido = 1,
    OracleNaoDefinido = 2,
}

#[contract]
pub struct AmbarMarketplace;

#[contractimpl]
impl AmbarMarketplace {
    
    pub fn __constructor(env: Env, owner: Address) {
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
        user.require_auth();
        
        let xlm_contract_address = Address::from_string(
            &String::from_str(&env, "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC")
        );
        
        // Cria cliente do token
        let token_client = token::Client::new(&env, &xlm_contract_address);
        
        // O spender é este contrato
        let spender = env.current_contract_address();
        
        // Chama transfer_from com os parâmetros na ordem correta
        token_client.transfer_from(
            &spender,  // spender (quem está executando a transferência)
            &user,     // from (de onde vem os tokens)
            &spender,       // to (para onde vão os tokens)
            &usdt_amount    // amount (quantidade em stroops)
        );
                
        let token_contract_addr: Address = env.storage()
            .persistent()
            .get(&symbol)
            .expect("Token nao definido");

        // let usdc_contract_addr: Address = env.storage()
        //     .persistent()
        //     .get(&Symbol::new(&env, "usdt"))
        //     .expect("Contrato USDT nao definido");
        
        // let _: () = env.invoke_contract(
        //     &xlm_addr,
        //     &Symbol::new(&env, "transfer_from"),
        //     soroban_sdk::vec![
        //         &env,
        //         env.current_contract_address().into_val(&env),
        //         user.clone().into_val(&env),
        //         env.current_contract_address().into_val(&env),
        //         usdt_amount.into_val(&env),
        //     ],
        // );

        let oracle_contract_addr: Address = env.storage()
            .persistent()
            .get(&Symbol::new(&env, "oracle"))
            .expect("Contrato USDT nao definido");
        
        let (price, _timestamp): (i128, u64) = env.invoke_contract(
                &oracle_contract_addr,
                &Symbol::new(&env, "get_price"),
                soroban_sdk::vec![&env, symbol.clone().into_val(&env)],
            );

        if price <= 0 {
            panic!("invalid price from oracle");
        }

        let tokens_to_mint = usdt_amount.checked_div(price).expect("Erro ao calcular tokens");

        let _: () = env.invoke_contract(
            &token_contract_addr,
            &Symbol::new(&env, "mint"),
            soroban_sdk::vec![&env, user.clone().into_val(&env), tokens_to_mint.into_val(&env)],
        );

        env.events().publish(
            (Symbol::new(&env, "MintedTokens"), user),
            (usdt_amount, tokens_to_mint),
        );
    }
}
