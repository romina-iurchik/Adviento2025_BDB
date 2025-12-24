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
        let total_anterior = chimenea.tamano_promedio * chimenea.cantidad_lenos;
        let nueva_cantidad = chimenea.cantidad_lenos + 1;
        let nuevo_promedio = (total_anterior + tamano) / nueva_cantidad;
        Chimenea {
            encendida: chimenea.encendida,
            cantidad_lenos: nueva_cantidad,
            tamano_promedio: nuevo_promedio,
        }
    }

    pub fn encender(_env: Env, chimenea: Chimenea) -> Result<Chimenea, Error> {
        if chimenea.cantidad_lenos == 0 { return Err(Error::NoHayLenos); }
        
        let nueva_chimenea = Chimenea {
            encendida: true,
            cantidad_lenos: chimenea.cantidad_lenos,
            tamano_promedio: chimenea.tamano_promedio,
        };
        Ok(nueva_chimenea)
    }
    
    pub fn calcular_calor(_env: Env, chimenea: Chimenea) -> u32 {
        // TODO
        if chimenea.encendida && chimenea.cantidad_lenos > 0 {
            chimenea.cantidad_lenos * chimenea.tamano_promedio * 10
        } else {
            0
        }
    }
    
    pub fn quemar_leno(_env: Env, chimenea: Chimenea) -> Result<Chimenea, Error> {
        // TODO
        if chimenea.cantidad_lenos == 0 { return Err(Error::NoHayLenosParaQuemar); }
        let nueva_cantidad = chimenea.cantidad_lenos - 1;
        let nueva_encendida = if nueva_cantidad == 0 { false } else { chimenea.encendida };
        let nueva_chimenea = Chimenea {
            encendida: nueva_encendida,
            cantidad_lenos: nueva_cantidad,
            tamano_promedio: chimenea.tamano_promedio,
        };
        Ok(nueva_chimenea)
    }
}
