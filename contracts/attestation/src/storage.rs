use soroban_sdk::{contracttype, BytesN};

/// Storage keys used by this contract.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Instance key holding the sole account authorised to submit attestations.
    Admin,
    /// Persistent key, keyed by report hash, holding the stored `Attestation`.
    Attestation(BytesN<32>),
}
