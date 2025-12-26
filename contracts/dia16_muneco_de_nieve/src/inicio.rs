#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Muneco {
    pub bola_inferior: u32,  // Radio en cm
    pub bola_media: u32,
    pub bola_superior: u32,
}

#[contract]
pub struct MunecoContract;

#[contractimpl]
impl MunecoContract {
    pub fn crear_muneco(_env: Env, inferior: u32, media: u32, superior: u32) -> Muneco {
        // TODO
        Muneco {
            bola_inferior: 0,
            bola_media: 0,
            bola_superior: 0,
        }
    }
    
    pub fn altura_total(_env: Env, muneco: Muneco) -> u32 {
        // TODO
        0
    }
    

    pub fn esta_completo(_env: Env, muneco: Muneco) -> bool {
        // TODO
        false
    }
    

    pub fn derretir(_env: Env, mut muneco: Muneco, cantidad: u32) -> Muneco {
        // TODO
        muneco
    }
    
    pub fn agregar_nieve(_env: Env, mut muneco: Muneco, bola: u32, cantidad: u32) -> Muneco {
        // TODO
        muneco
    }
}