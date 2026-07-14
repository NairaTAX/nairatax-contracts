#![cfg(test)]

use super::{AttestationContract, AttestationContractClient, Error};
use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, Symbol};

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

#[test]
fn submit_attestation_stores_record_for_admin() {
    let (env, client, admin) = setup();
    env.mock_all_auths();
    client.init(&admin);

    let report_hash = BytesN::from_array(&env, &[1u8; 32]);
    let account = Address::generate(&env);
    let period = Symbol::new(&env, "2026");

    client.submit_attestation(&admin, &report_hash, &account, &period);

    let stored = client.get_attestation(&report_hash).unwrap();
    assert_eq!(stored.report_hash, report_hash);
    assert_eq!(stored.account, account);
    assert_eq!(stored.period, period);
}
