#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, Symbol, symbol_short};

mod rewards_contract {
    soroban_sdk::contractimport!(
        file = "../../../rewards-contract/target/wasm32v1-none/release/rewards_contract.wasm"
    );
}

#[contracttype]
pub enum DataKey {
    Options,
    Votes,
    HasVoted(Address),
    RewardsContract,
}

#[contract]
pub struct PollContract;

#[contractimpl]
impl PollContract {
    pub fn initialize(env: Env, options: soroban_sdk::Vec<Symbol>, rewards_contract_address: Address) {
        let mut votes: Map<Symbol, u32> = Map::new(&env);
        for option in options.iter() {
            votes.set(option.clone(), 0);
        }
        env.storage().instance().set(&DataKey::Votes, &votes);
        env.storage().instance().set(&DataKey::Options, &options);
        env.storage().instance().set(&DataKey::RewardsContract, &rewards_contract_address);
    }

    pub fn vote(env: Env, voter: Address, option: Symbol) -> bool {
        voter.require_auth();

        let has_voted_key = DataKey::HasVoted(voter.clone());
        let already_voted: bool = env.storage().instance().get(&has_voted_key).unwrap_or(false);

        if already_voted {
            return false;
        }

        let mut votes: Map<Symbol, u32> = env.storage().instance().get(&DataKey::Votes).unwrap();

        let current: u32 = votes.get(option.clone()).unwrap_or(0);
        votes.set(option.clone(), current + 1);

        env.storage().instance().set(&DataKey::Votes, &votes);
        env.storage().instance().set(&has_voted_key, &true);

        let rewards_address: Address = env.storage().instance().get(&DataKey::RewardsContract).unwrap();
        let rewards_client = rewards_contract::Client::new(&env, &rewards_address);
        rewards_client.add_reward(&voter, &10);

        env.events().publish((symbol_short!("voted"),), option);

        true
    }

    pub fn get_results(env: Env) -> Map<Symbol, u32> {
        env.storage().instance().get(&DataKey::Votes).unwrap()
    }

    pub fn get_options(env: Env) -> soroban_sdk::Vec<Symbol> {
        env.storage().instance().get(&DataKey::Options).unwrap()
    }
}

mod test;