#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map};

// ❄️ Día 12: Nieve Cristalina
// Address y Autenticación en Soroban

#[contract]
pub struct NieveContract;

#[contractimpl]
impl NieveContract {
    pub fn recolectar(_env: Env, mut copos: Map<Address, u32>, persona: Address, cantidad: u32) -> Map<Address, u32> {
        // Verificar autenticación
        persona.require_auth();

        let actual = copos.get(persona.clone()).unwrap_or(0);
        copos.set(persona, actual + cantidad);
        copos
    }
    
    /// Obtiene cuántos copos tiene una persona
    pub fn ver_copos(_env: Env, copos: Map<Address, u32>, persona: Address) -> u32 {
        copos.get(persona).unwrap_or(0)
    }
    
    /// Transfiere copos de una persona a otra
    pub fn transferir(_env: Env, mut copos: Map<Address, u32>, de: Address, a: Address, cantidad: u32) -> Map<Address, u32> {
        // Verificar autenticación del emisor
        de.require_auth();
        
        let saldo_de = copos.get(de.clone()).unwrap_or(0);
        if saldo_de < cantidad {
            panic!("Saldo insuficiente");
        }
        let saldo_a = copos.get(a.clone()).unwrap_or(0);
        copos.set(de, saldo_de - cantidad);
        copos.set(a, saldo_a + cantidad);
        copos
    }
}
