#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Env};

#[test]
fn test_add_reward_and_get_points() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RewardsContract);
    let client = RewardsContractClient::new(&env, &contract_id);

    let voter = Address::generate(&env);

    client.add_reward(&voter, &10);

    let points = client.get_voter_points(&voter);
    assert_eq!(points, 10);
}

#[test]
fn test_multiple_rewards_accumulate() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RewardsContract);
    let client = RewardsContractClient::new(&env, &contract_id);

    let voter = Address::generate(&env);

    client.add_reward(&voter, &10);
    client.add_reward(&voter, &5);

    let points = client.get_voter_points(&voter);
    assert_eq!(points, 15);
}

#[test]
fn test_get_rewards_list() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RewardsContract);
    let client = RewardsContractClient::new(&env, &contract_id);

    let voter1 = Address::generate(&env);
    let voter2 = Address::generate(&env);

    client.add_reward(&voter1, &10);
    client.add_reward(&voter2, &20);

    let rewards = client.get_rewards();
    assert_eq!(rewards.len(), 2);
}

#[test]
fn test_voter_with_no_rewards_has_zero_points() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RewardsContract);
    let client = RewardsContractClient::new(&env, &contract_id);

    let voter = Address::generate(&env);

    let points = client.get_voter_points(&voter);
    assert_eq!(points, 0);
}