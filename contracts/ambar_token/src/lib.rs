// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Stellar Soroban Contracts ^0.4.1
#![no_std]

use soroban_sdk::{Address, contract, contractimpl, Env, String};
use stellar_access::ownable::{self as ownable, Ownable};
use stellar_macros::{default_impl, only_owner};
use stellar_tokens::fungible::{Base, burnable::FungibleBurnable, FungibleToken};

#[contract]
pub struct EnerTrade;

#[contractimpl]
impl EnerTrade {
    pub fn __constructor(e: &Env, owner: Address) {
        Base::set_metadata(e, 18, String::from_str(e, "EnerTrade"), String::from_str(e, "Ener"));
        ownable::set_owner(e, &owner);
    }

    #[only_owner]
    pub fn mint(e: &Env, account: Address, amount: i128) {
        Base::mint(e, &account, amount);
    }
}

#[default_impl]
#[contractimpl]
impl FungibleToken for EnerTrade {
    type ContractType = Base;

}

//
// Extensions
//

#[default_impl]
#[contractimpl]
impl FungibleBurnable for EnerTrade {}

//
// Utils
//

#[default_impl]
#[contractimpl]
impl Ownable for EnerTrade {}