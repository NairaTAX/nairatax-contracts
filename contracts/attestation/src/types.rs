use soroban_sdk::{contracttype, Address, BytesN, Symbol};

/// An on-chain anchor for a NairaTax report: the report's hash, the account
/// it covers, the tax period, and the ledger timestamp it was submitted at.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attestation {
    /// SHA-256 hash of the canonical report produced by `nairatax-engine`.
    pub report_hash: BytesN<32>,
    /// Stellar account the report covers.
    pub account: Address,
    /// Tax period the report covers, e.g. "2026" or "2026Q1".
    pub period: Symbol,
    /// Ledger timestamp at the moment of submission.
    pub timestamp: u64,
}
