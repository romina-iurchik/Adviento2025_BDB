#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map};

// ❄️ Día 12: Nieve Cristalina
// Address y Autenticación en Soroban

#[contract]
pub struct NieveContract;

#[contractimpl]
impl NieveContract {
    pub fn recolectar(env: Env, mut copos: Map<Address, u32>, persona: Address, cantidad: u32) -> Map<Address, u32> {
        // TODO
        copos
    }
    
    /// Obtiene cuántos copos tiene una persona
    pub fn ver_copos(_env: Env, copos: Map<Address, u32>, persona: Address) -> u32 {
        // TODO
        0
    }
    
    /// Transfiere copos de una persona a otra
    pub fn transferir(env: Env, mut copos: Map<Address, u32>, de: Address, a: Address, cantidad: u32) -> Map<Address, u32> {
        // TODO
        copos
    }
}