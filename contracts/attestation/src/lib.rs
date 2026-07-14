#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct AttestationContract;

#[contractimpl]
impl AttestationContract {
    /// Placeholder so the crate compiles; replaced by init() in the next commit.
    pub fn ping(_env: Env) -> bool {
        true
    }
}
