use soroban_sdk::{contractevent, Address, BytesN, Symbol};

/// Published whenever `submit_attestation` anchors a new report hash, so
/// off-chain services can watch for new anchors without polling
/// `get_attestation`.
#[contractevent]
pub struct AttestationSubmitted {
    /// Indexed topic so events can be filtered by report hash.
    #[topic]
    pub report_hash: BytesN<32>,
    pub account: Address,
    pub period: Symbol,
    pub timestamp: u64,
}
