use soroban_sdk::{contracttype, Address, BytesN, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attestation {
    pub report_hash: BytesN<32>,
    pub account: Address,
    pub period: Symbol,
    pub timestamp: u64,
}
