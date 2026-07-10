#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, Symbol, symbol_short};

// Poll ke options aur unke votes store karne ke liye
#[contracttype]
pub enum DataKey {
    Options,        // saare options ki list
    Votes,          // option -> vote count ka map
    HasVoted(Address), // check karega ki ye address vote kar chuka hai ya nahi
}

#[contract]
pub struct PollContract;

#[contractimpl]
impl PollContract {
    // Poll shuru karne ke liye (sirf ek baar call hoga)
    pub fn initialize(env: Env, options: soroban_sdk::Vec<Symbol>) {
        let mut votes: Map<Symbol, u32> = Map::new(&env);
        for option in options.iter() {
            votes.set(option.clone(), 0);
        }
        env.storage().instance().set(&DataKey::Votes, &votes);
        env.storage().instance().set(&DataKey::Options, &options);
    }

    // Vote dena
    pub fn vote(env: Env, voter: Address, option: Symbol) -> bool {
        voter.require_auth(); // wallet se signature verify karega

        // check karo ye voter pehle vote kar chuka hai kya
        let has_voted_key = DataKey::HasVoted(voter.clone());
        let already_voted: bool = env.storage().instance().get(&has_voted_key).unwrap_or(false);

        if already_voted {
            // agar pehle se vote kiya hai, to fail return karo
            return false;
        }

        let mut votes: Map<Symbol, u32> = env.storage().instance().get(&DataKey::Votes).unwrap();

        let current: u32 = votes.get(option.clone()).unwrap_or(0);
        votes.set(option.clone(), current + 1);

        env.storage().instance().set(&DataKey::Votes, &votes);
        env.storage().instance().set(&has_voted_key, &true);

        // Event emit karo taaki frontend ko pata chale
        env.events().publish((symbol_short!("voted"),), option);

        true
    }

    // Results dekhne ke liye
    pub fn get_results(env: Env) -> Map<Symbol, u32> {
        env.storage().instance().get(&DataKey::Votes).unwrap()
    }

    // Options ki list dekhne ke liye
    pub fn get_options(env: Env) -> soroban_sdk::Vec<Symbol> {
        env.storage().instance().get(&DataKey::Options).unwrap()
    }
}

mod test;