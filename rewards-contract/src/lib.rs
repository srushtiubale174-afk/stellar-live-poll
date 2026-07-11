#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol, Vec};

#[contracttype]
#[derive(Clone)]
pub struct RewardRecord {
    pub voter: Address,
    pub points: u32,
}

const REWARDS_KEY: Symbol = symbol_short!("REWARDS");

#[contract]
pub struct RewardsContract;

#[contractimpl]
impl RewardsContract {
    pub fn add_reward(env: Env, voter: Address, points: u32) {
        voter.require_auth();

        let mut rewards: Vec<RewardRecord> = env
            .storage()
            .instance()
            .get(&REWARDS_KEY)
            .unwrap_or(Vec::new(&env));

        let record = RewardRecord {
            voter: voter.clone(),
            points,
        };

        rewards.push_back(record);

        env.storage().instance().set(&REWARDS_KEY, &rewards);

        env.events()
            .publish((symbol_short!("reward"), voter), points);
    }

    pub fn get_rewards(env: Env) -> Vec<RewardRecord> {
        env.storage()
            .instance()
            .get(&REWARDS_KEY)
            .unwrap_or(Vec::new(&env))
    }

    pub fn get_voter_points(env: Env, voter: Address) -> u32 {
        let rewards: Vec<RewardRecord> = env
            .storage()
            .instance()
            .get(&REWARDS_KEY)
            .unwrap_or(Vec::new(&env));

        let mut total: u32 = 0;
        for record in rewards.iter() {
            if record.voter == voter {
                total += record.points;
            }
        }
        total
    }
}
mod test;