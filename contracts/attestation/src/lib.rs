#![no_std]
//! Soroban contract that anchors NairaTax report hashes on-chain.
//!
//! `nairatax-engine` computes a tax report and its hash off-chain; this
//! contract's only job is to record that hash so it can later be
//! independently verified by a filer, an accountant, or the NRS. It never
//! stores report contents, PII, or funds — only a hash, an account, a
//! period, and a timestamp.

mod errors;
mod events;
mod storage;
#[cfg(test)]
mod test;
mod types;

use errors::Error;
use events::AttestationSubmitted;
use storage::DataKey;
use types::Attestation;

use soroban_sdk::{contract, contractimpl, contractmeta, Address, BytesN, Env, Symbol};

contractmeta!(
    key = "Description",
    val = "Anchors NairaTax report hashes on-chain for later verification"
);

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

        let timestamp = env.ledger().timestamp();
        let attestation = Attestation {
            report_hash: report_hash.clone(),
            account: account.clone(),
            period: period.clone(),
            timestamp,
        };
        env.storage().persistent().set(&key, &attestation);

        AttestationSubmitted {
            report_hash,
            account,
            period,
            timestamp,
        }
        .publish(&env);

        Ok(())
    }

    /// Read-only lookup; callable by anyone, no authentication required.
    pub fn get_attestation(env: Env, report_hash: BytesN<32>) -> Option<Attestation> {
        env.storage()
            .persistent()
            .get(&DataKey::Attestation(report_hash))
    }
}
