#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, Vec};

// 🎄 Día 5: Trineo Automatizado
// ¡Tu PRIMER contrato Soroban!

#[contract]
pub struct TrineoContract;

#[contractimpl]
impl TrineoContract {
    /// Agrega un regalo a la lista del trineo
    pub fn agregar_regalo(env: Env, regalos: Vec<Symbol>, regalo: Symbol) -> Vec<Symbol> {
        // TODO: Agregá el regalo a la lista
        // Pista: En Soroban usamos push_back en vez de push
        // regalos.push_back(regalo);
        regalos
    }
    
    /// Cuenta cuántos regalos lleva el trineo
    pub fn contar_regalos(env: Env, regalos: Vec<Symbol>) -> u32 {
        // TODO: Retorná la cantidad de regalos
        // Pista: igual que en Rust normal, usá .len()
        0
    }
    
    /// Verifica si el trineo puede despegar (mínimo 3 regalos)
    pub fn puede_despegar(env: Env, regalos: Vec<Symbol>) -> bool {
        // TODO: Verificá que haya al menos 3 regalos
        // Pista: regalos.len() >= 3
        false
    }
}