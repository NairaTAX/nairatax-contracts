use soroban_sdk::{contractevent, Address, BytesN, Symbol};

#[contractevent]
pub struct AttestationSubmitted {
    #[topic]
    pub report_hash: BytesN<32>,
    pub account: Address,
    pub period: Symbol,
    pub timestamp: u64,
}
