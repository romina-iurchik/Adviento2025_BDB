#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Vec};

// 🎄 Día 11: Campanas Sonoras
// Ledger y Timestamps en Soroban

#[contract]
pub struct CampanasContract;

#[contractimpl]
impl CampanasContract {
    pub fn sonar(env: Env, mut campanadas: Vec<u64>) -> Vec<u64> {
        let time = env.ledger().timestamp();
        campanadas.push_back(time);
        campanadas
    }
    
    pub fn contar_campanadas(_env: Env, campanadas: Vec<u64>) -> u32 {
        campanadas.len()
    }
    
    pub fn tiempo_transcurrido(_env: Env, campanadas: Vec<u64>) -> u64 {
        if campanadas.len() < 2 { return 0; }
        let primera = campanadas.get(0).unwrap();
        let ultima = campanadas.get(campanadas.len()-1).unwrap();
        ultima-primera
        
    }
    
    pub fn ultima_campanada(_env: Env, campanadas: Vec<u64>) -> u64 {
        if campanadas.is_empty() { return 0; }
        campanadas.get(campanadas.len()-1).unwrap()
        
    }
}