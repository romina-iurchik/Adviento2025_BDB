// 🎁 Día 22: Regalo Sorpresa

#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, contracttype, symbol_short, Env, Symbol, Vec};

const CAJAS: Symbol = symbol_short!("CAJAS");

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    CajaYaAbierta = 1,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Caja {
    pub color: Symbol,
    pub contenido: Option<Symbol>,
    pub abierta: bool,
}

#[contract]
pub struct RegaloContract;

#[contractimpl]
impl RegaloContract {
    pub fn crear_caja(_env: Env, color: Symbol, contenido: Option<Symbol>) -> Caja {
        // TODO
        Caja {
            color,
            contenido,
            abierta: false,
        }
    }
    
    pub fn abrir_caja(_env: Env, caja: Caja) -> Result<Caja, Error> {
        // TODO
        
        Ok(caja)
    }
    
    pub fn esta_abierta(_env: Env, caja: Caja) -> bool {
        // TODO
        false
    }
    
    pub fn obtener_contenido(_env: Env, caja: Caja) -> Option<Symbol> {
        // TODO
        None
    }
    
    pub fn guardar_caja(env: Env, caja: Caja) {
        // TODO
    }
    
    pub fn contar_cajas_cerradas(env: Env) -> u32 {
        // TODO
        0
    }
}