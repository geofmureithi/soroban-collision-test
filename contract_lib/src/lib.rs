#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[soroban_sdk::contracttype]
pub enum DataKey {
    InnerCounter,
}

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    pub fn init(env: Env) {
        env.storage().persistent().set(&COUNTER, &0u32);
        env.storage()
            .persistent()
            .set(&DataKey::InnerCounter, &0u32)
    }

    pub fn increment(env: Env, by: u32) -> u32 {
        let mut count = env.storage().persistent().get::<_, u32>(&COUNTER).unwrap();
        count += by;
        env.storage().persistent().set(&COUNTER, &count);
        env.storage()
            .persistent()
            .set(&DataKey::InnerCounter, &count);
        count
    }

    pub fn count(env: Env) -> u32 {
        env.storage().persistent().get::<_, u32>(&COUNTER).unwrap()
    }
}
