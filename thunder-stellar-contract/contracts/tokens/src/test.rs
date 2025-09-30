#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, String,
};
use soroban_sdk::testutils::{Ledger,LedgerInfo};

fn s(e: &Env, val: &str) -> String {
    String::from_str(e, val)
}


fn setup<'a>() -> (Env, Address, Address, Address, TokenClient<'a>) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    env.ledger().set(LedgerInfo {
        protocol_version: 22,
        sequence_number: 100,
        timestamp: 0,
        base_reserve: 5_000_000,
        min_temp_entry_ttl: 1,
        min_persistent_entry_ttl: 1,
        max_entry_ttl: 631_152_000,
           network_id: [0; 32],
    });

    let contract_id = env.register_contract(None, Token);
    let client = TokenClient::new(&env, &contract_id);

    (env, admin, user1, user2, client)
}

#[test]
fn test_initialize() {
    let (env, admin, _, _, client) = setup();

    client.initialize(&admin, &18, &s(&env, "TestToken"), &s(&env, "TT"));
    assert_eq!(client.decimals(), 18);
    assert_eq!(client.name(), s(&env, "TestToken"));
    assert_eq!(client.symbol(), s(&env, "TT"));
}

#[test]
fn test_mint_and_balance() {
    let (env, admin, user1, _, client) = setup();

    client.initialize(&admin, &6, &s(&env, "T"), &s(&env, "T"));
    client.mint(&user1, &1_000_000);
    assert_eq!(client.balance(&user1), 1_000_000);
}

#[test]
fn test_transfer() {
    let (env, admin, user1, user2, client) = setup();

    client.initialize(&admin, &6, &s(&env, "T"), &s(&env, "T"));
    client.mint(&user1, &500_000);
    client.transfer(&user1, &user2, &200_000);

    assert_eq!(client.balance(&user1), 300_000);
    assert_eq!(client.balance(&user2), 200_000);
}

#[test]
fn test_approve_and_transfer_from() {
    let (env, admin, user1, user2, client) = setup();
    let spender = Address::generate(&env);

    client.initialize(&admin, &6, &s(&env, "T"), &s(&env, "T"));
    client.mint(&user1, &500_000);

    let expiration = env.ledger().sequence() + 100;
    client.approve(&user1, &spender, &100_000, &expiration);
    assert_eq!(client.allowance(&user1, &spender), 100_000);

    client.transfer_from(&spender, &user1, &user2, &60_000);
    assert_eq!(client.balance(&user1), 440_000);
    assert_eq!(client.balance(&user2), 60_000);
    assert_eq!(client.allowance(&user1, &spender), 40_000);
}

#[test]
fn test_burn_and_burn_from() {
    let (env, admin, user1, _, client) = setup();
    let spender = Address::generate(&env);

    client.initialize(&admin, &6, &s(&env, "T"), &s(&env, "T"));
    client.mint(&user1, &300_000);

    client.burn(&user1, &100_000);
    assert_eq!(client.balance(&user1), 200_000);

    let expiration = env.ledger().sequence() + 100;
    client.approve(&user1, &spender, &50_000, &expiration);
    client.burn_from(&spender, &user1, &50_000);
    assert_eq!(client.balance(&user1), 150_000);
}

#[test]
#[should_panic(expected = "already initialized")]
fn test_initialize_twice_should_fail() {
    let (env, admin, _, _, client) = setup();
    client.initialize(&admin, &6, &s(&env, "T"), &s(&env, "T"));
    client.initialize(&admin, &6, &s(&env, "T"), &s(&env, "T")); // panic
}

#[test]
#[should_panic(expected = "insufficient balance")]
fn test_transfer_more_than_balance_should_fail() {
    let (env, admin, user1, user2, client) = setup();
    client.initialize(&admin, &6, &s(&env, "T"), &s(&env, "T"));
    client.mint(&user1, &100_000);
    client.transfer(&user1, &user2, &200_000); // panic
}