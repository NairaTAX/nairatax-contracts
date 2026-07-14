#![no_std]

mod errors;
mod storage;
mod types;

use errors::Error;
use storage::DataKey;
use types::Attestation;

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, Symbol};

#[contract]
pub struct AttestationContract;

#[contractimpl]
impl AttestationContract {
    /// Sets the sole account authorised to call `submit_attestation`. Callable once.
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        Ok(())
    }

    /// Anchors `report_hash` on-chain. Only the configured admin may call this;
    /// re-submitting an already-anchored hash fails rather than overwriting it,
    /// since attestations are append-only by design.
    pub fn submit_attestation(
        env: Env,
        submitter: Address,
        report_hash: BytesN<32>,
        account: Address,
        period: Symbol,
    ) -> Result<(), Error> {
        submitter.require_auth();

        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;

        if submitter != admin {
            return Err(Error::NotAuthorized);
        }

        let key = DataKey::Attestation(report_hash.clone());
        if env.storage().persistent().has(&key) {
            return Err(Error::AttestationExists);
        }

        let attestation = Attestation {
            report_hash,
            account,
            period,
            timestamp: env.ledger().timestamp(),
        };
        env.storage().persistent().set(&key, &attestation);

        Ok(())
    }

    /// Read-only lookup; callable by anyone, no authentication required.
    pub fn get_attestation(env: Env, report_hash: BytesN<32>) -> Option<Attestation> {
        env.storage()
            .persistent()
            .get(&DataKey::Attestation(report_hash))
    }
}
