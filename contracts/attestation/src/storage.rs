use soroban_sdk::{contracttype, BytesN};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Attestation(BytesN<32>),
}
