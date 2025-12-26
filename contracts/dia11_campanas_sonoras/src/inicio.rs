#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Vec};

// 🎄 Día 11: Campanas Sonoras
// Ledger y Timestamps en Soroban

#[contract]
pub struct CampanasContract;

#[contractimpl]
impl CampanasContract {
    pub fn sonar(env: Env, mut campanadas: Vec<u64>) -> Vec<u64> {
        // TODO:
        campanadas
    }
    
    pub fn contar_campanadas(_env: Env, campanadas: Vec<u64>) -> u32 {
        // TODO
        0
    }
    
    pub fn tiempo_transcurrido(_env: Env, campanadas: Vec<u64>) -> u64 {
        // TODO
        0
    }
    
    pub fn ultima_campanada(_env: Env, campanadas: Vec<u64>) -> u64 {
        // TODO
        0
    }
}