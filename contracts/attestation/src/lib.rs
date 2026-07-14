#![no_std]

mod errors;
mod storage;
mod types;

use errors::Error;
use storage::DataKey;

use soroban_sdk::{contract, contractimpl, Address, Env};

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
}
