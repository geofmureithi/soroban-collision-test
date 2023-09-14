#![no_std]

mod contract_lib {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/contract_lib.wasm"
    );
}

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Address, contracttype};
const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct MyCounterContract;

#[contracttype]
pub enum DataKey {
    InnerCounter
}


#[contractimpl]
impl MyCounterContract {
    pub fn init(env: Env, contract: Address) {
        env.storage().persistent().set(&DataKey::InnerCounter, &contract);
        let client = contract_lib::Client::new(&env, &contract);
        client.init();
    }

    pub fn increment(env: Env, by: u32) -> u32 {
        let contract = env.storage().persistent().get(&DataKey::InnerCounter).unwrap();
        let client = contract_lib::Client::new(&env, &contract);
        client.increment(&by)
    }

    pub fn count_bad(env: Env) -> u32 {
        env.storage().persistent().get::<_, u32>(&COUNTER).unwrap()
    }
    pub fn count_good(env: Env) -> u32 {
        let contract = env.storage().persistent().get(&DataKey::InnerCounter).unwrap();
        let client = contract_lib::Client::new(&env, &contract);
        client.count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn composition_works_well() {
        let env = Env::default();
        let inner_contract = env.register_contract_wasm(None, contract_lib::WASM);
        let my_counter_addr = env.register_contract(None, MyCounterContract);
        let my_counter_client = MyCounterContractClient::new(&env, &my_counter_addr);
        my_counter_client.init(&inner_contract);
        my_counter_client.increment(&1);
        assert_eq!(1, my_counter_client.count_good());
    }

    #[test]
    #[should_panic]
    fn cannot_import_symbol() {
        let env = Env::default();
        let inner_contract = env.register_contract_wasm(None, contract_lib::WASM);
        let my_counter_addr = env.register_contract(None, MyCounterContract);
        let my_counter_client = MyCounterContractClient::new(&env, &my_counter_addr);
        my_counter_client.init(&inner_contract);
        my_counter_client.increment(&1);
        assert_eq!(1, my_counter_client.count_bad());
    }
}
