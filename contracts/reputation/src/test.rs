#![cfg(test)]

use super::*;
use soroban_sdk::testutils::{Address as _};
use soroban_sdk::Env;

#[test]
fn test_reputation_flow() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ReputationContract);
    let client = ReputationContractClient::new(&env, &contract_id);

    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    client.submit_review(&user1, &user2, &8, &"Good".into());

    let (_, _, _, total, normalized) = client.get_reputation(&user2);

    assert_eq!(total, 1);
    assert!(normalized > 50);
}
