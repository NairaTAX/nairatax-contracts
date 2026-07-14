use soroban_sdk::contracterror;

/// Errors returned by the attestation contract's public functions.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// `submit_attestation` was called before `init` set an admin.
    NotInitialized = 1,
    /// `init` was called a second time; the admin can only be set once.
    AlreadyInitialized = 2,
    /// The submitter does not match the configured admin.
    NotAuthorized = 3,
    /// The report hash was already anchored; attestations are append-only.
    AttestationExists = 4,
}
