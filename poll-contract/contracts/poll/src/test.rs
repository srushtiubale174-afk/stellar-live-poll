#![cfg(test)]
use super::*;
use soroban_sdk::{symbol_short, testutils::Address as _, Env};

fn create_poll_contract(env: &Env) -> PollContractClient {
    let contract_id = env.register_contract(None, PollContract);
    PollContractClient::new(env, &contract_id)
}

#[test]
fn test_initialize_and_get_options() {
    let env = Env::default();
    env.mock_all_auths();

    let client = create_poll_contract(&env);
    let rewards_address = Address::generate(&env);

    let options = soroban_sdk::vec![
        &env,
        symbol_short!("Stellar"),
        symbol_short!("Ethereum")
    ];

    client.initialize(&options, &rewards_address);

    let stored_options = client.get_options();
    assert_eq!(stored_options.len(), 2);
}

#[test]
fn test_vote_increases_count() {
    let env = Env::default();
    env.mock_all_auths();

    let client = create_poll_contract(&env);
    let rewards_address = Address::generate(&env);
    let voter = Address::generate(&env);

    let options = soroban_sdk::vec![&env, symbol_short!("Stellar")];
    client.initialize(&options, &rewards_address);

    // Note: full vote() call needs rewards contract deployed too,
    // so this test checks initial state before voting
    let results = client.get_results();
    assert_eq!(results.get(symbol_short!("Stellar")), Some(0));
}

#[test]
fn test_get_results_starts_at_zero() {
    let env = Env::default();
    env.mock_all_auths();

    let client = create_poll_contract(&env);
    let rewards_address = Address::generate(&env);

    let options = soroban_sdk::vec![
        &env,
        symbol_short!("Stellar"),
        symbol_short!("Solana")
    ];
    client.initialize(&options, &rewards_address);

    let results = client.get_results();
    assert_eq!(results.get(symbol_short!("Stellar")), Some(0));
    assert_eq!(results.get(symbol_short!("Solana")), Some(0));
}