#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Map, Symbol};

// 🎄 Día 8: Elfos Trabajadores
// Map (Diccionarios) en Soroban

#[contract]
pub struct ElfosContract;

#[contractimpl]
impl ElfosContract {
    pub fn registrar_elfo(_env: Env, mut elfos: Map<Symbol, u32>, nombre: Symbol) -> Map<Symbol, u32> {
        // TODO
        elfos.set(nombre, 0);
        elfos
    }
    
    pub fn fabricar_juguetes(_env: Env, mut elfos: Map<Symbol, u32>, nombre: Symbol, cantidad: u32) -> Map<Symbol, u32> {
        // TODO
        let actual = elfos.get(nombre.clone()).unwrap_or(0);
        elfos.set(nombre, actual+cantidad);
        elfos
    }
    
    pub fn contar_juguetes(_env: Env, elfos: Map<Symbol, u32>, nombre: Symbol) -> u32 {
        // TODO
        elfos.get(nombre).unwrap_or(0)
    }
    
    pub fn existe_elfo(_env: Env, elfos: Map<Symbol, u32>, nombre: Symbol) -> bool {
        // TODO
        elfos.contains_key(nombre)
    }
}