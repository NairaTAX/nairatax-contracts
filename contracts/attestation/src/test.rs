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

#[test]
fn submit_attestation_rejects_non_admin() {
    let (env, client, admin) = setup();
    env.mock_all_auths();
    client.init(&admin);

    let attacker = Address::generate(&env);
    let report_hash = BytesN::from_array(&env, &[2u8; 32]);
    let account = Address::generate(&env);
    let period = Symbol::new(&env, "2026");

    let result = client.try_submit_attestation(&attacker, &report_hash, &account, &period);
    assert_eq!(result, Err(Ok(Error::NotAuthorized)));
}

#[test]
fn submit_attestation_rejects_duplicate_hash() {
    let (env, client, admin) = setup();
    env.mock_all_auths();
    client.init(&admin);

    let report_hash = BytesN::from_array(&env, &[3u8; 32]);
    let account = Address::generate(&env);
    let period = Symbol::new(&env, "2026");

    client.submit_attestation(&admin, &report_hash, &account, &period);
    let result = client.try_submit_attestation(&admin, &report_hash, &account, &period);
    assert_eq!(result, Err(Ok(Error::AttestationExists)));
}

#[test]
fn submit_attestation_before_init_fails() {
    let (env, client, admin) = setup();
    env.mock_all_auths();

    let report_hash = BytesN::from_array(&env, &[4u8; 32]);
    let account = Address::generate(&env);
    let period = Symbol::new(&env, "2026");

    let result = client.try_submit_attestation(&admin, &report_hash, &account, &period);
    assert_eq!(result, Err(Ok(Error::NotInitialized)));
}

#[test]
fn get_attestation_returns_none_for_unknown_hash() {
    let (env, client, _admin) = setup();
    let report_hash = BytesN::from_array(&env, &[5u8; 32]);
    assert_eq!(client.get_attestation(&report_hash), None);
}
