#![cfg(test)]

use super::{AttestationContract, AttestationContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env};

fn setup<'a>() -> (Env, AttestationContractClient<'a>, Address) {
    let env = Env::default();
    let contract_id = env.register(AttestationContract, ());
    let client = AttestationContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    (env, client, admin)
}

#[test]
fn init_sets_admin_once() {
    let (_env, client, admin) = setup();
    client.init(&admin);
}

#[test]
fn init_twice_fails() {
    let (env, client, admin) = setup();
    client.init(&admin);

    let other = Address::generate(&env);
    let result = client.try_init(&other);
    assert!(result.is_err());
}
