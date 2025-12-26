#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

// 🎄 Día 6: Luces Navideñas
// Storage Persistente en Soroban

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct LucesContract;

#[contractimpl]
impl LucesContract {
    /// Inicializa el contador en 0
    pub fn inicializar(env: Env) {
        // TODO: Guardá 0 en el storage
        // env.storage().instance().set(&COUNTER, &0_u32);
    }
    
    /// Incrementa el contador de luces encendidas
    pub fn encender_luz(env: Env) -> u32 {
        // TODO: 
        // 1. Obtené el valor actual: env.storage().instance().get(&COUNTER).unwrap_or(0)
        // 2. Incrementalo en 1
        // 3. Guardalo de nuevo en el storage
        // 4. Retorná el nuevo valor
        0
    }
    
    /// Obtiene cuántas luces están encendidas
    pub fn contar_luces(env: Env) -> u32 {
        // TODO: Retorná el valor del storage
        // env.storage().instance().get(&COUNTER).unwrap_or(0)
        0
    }
    
    /// Apaga todas las luces (resetea a 0)
    pub fn apagar_todo(env: Env) {
        // TODO: Guardá 0 en el storage
    }
}