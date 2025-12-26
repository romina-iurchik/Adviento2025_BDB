// 🔥 Día 21: Chimenea Acogedora

#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, contracttype, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NoHayLenos = 1,
    NoHayLenosParaQuemar = 2,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Chimenea {
    pub encendida: bool,
    pub cantidad_lenos: u32,
    pub tamano_promedio: u32,
}

#[contract]
pub struct ChimeneaContract;

#[contractimpl]
impl ChimeneaContract {
    pub fn crear_chimenea(_env: Env) -> Chimenea {
        // TODO
        Chimenea {
            encendida: false,
            cantidad_lenos: 0,
            tamano_promedio: 0,
        }
    }
    
    pub fn agregar_leno(_env: Env, chimenea: Chimenea, tamano: u32) -> Chimenea {
        // TODO
        
        Chimenea {
            encendida: chimenea.encendida,
            cantidad_lenos: 0,
            tamano_promedio: 0,
        }
    }
    
    pub fn encender(_env: Env, chimenea: Chimenea) -> Result<Chimenea, Error> {
        // TODO
        
        Ok(chimenea)
    }
    
    pub fn calcular_calor(_env: Env, chimenea: Chimenea) -> u32 {
        // TODO
        
        0
    }
    
    pub fn quemar_leno(_env: Env, chimenea: Chimenea) -> Result<Chimenea, Error> {
        // TODO
        
        Ok(chimenea)
    }
}