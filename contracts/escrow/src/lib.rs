#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

pub mod errors;

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {}
