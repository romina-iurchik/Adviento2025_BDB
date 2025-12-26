#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Map, Symbol};

// 🎄 Día 10: Renos Voladores
// Structs Personalizados en Soroban

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reno {
    pub nombre: Symbol,
    pub velocidad: u32,
    pub energia: u32,
}

#[contract]
pub struct RenosContract;

#[contractimpl]
impl RenosContract {
    pub fn crear_reno(env: Env, nombre: Symbol, velocidad: u32) -> Reno {
        // TODO
        Reno {
            nombre,
            velocidad,
            energia: 0,
        }
    }
    
    pub fn volar(env: Env, mut reno: Reno, distancia: u32) -> Reno {
        // TODO        
        reno
    }
    
    pub fn descansar(env: Env, mut reno: Reno, cantidad: u32) -> Reno {
        // TODO
        reno
    }
    
    pub fn puede_volar(env: Env, reno: Reno, distancia: u32) -> bool {
        // TODO
        false
    }
}