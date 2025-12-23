
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
            bola_inferior: inferior,
            bola_media: media,
            bola_superior: superior,
        }
    }
    
    pub fn altura_total(_env: Env, muneco: Muneco) -> u32 {
        (muneco.bola_inferior + muneco.bola_media + muneco.bola_superior) * 2
        
    }
    

    pub fn esta_completo(_env: Env, muneco: Muneco) -> bool {
        muneco.bola_inferior > 0 && muneco.bola_media > 0 && muneco.bola_superior > 0 
    }
    

    pub fn derretir(_env: Env, mut muneco: Muneco, cantidad: u32) -> Muneco {
        if cantidad > muneco.bola_inferior {
            muneco.bola_inferior = 0;
        } else {
            muneco.bola_inferior -= cantidad;
        }

        if cantidad > muneco.bola_media {
            muneco.bola_media = 0;
        } else {
            muneco.bola_media -= cantidad;
        }

        if cantidad > muneco.bola_superior {
            muneco.bola_superior = 0;
        } else {
            muneco.bola_superior -= cantidad;
        }

        muneco
    }
    
    pub fn agregar_nieve(_env: Env, mut muneco: Muneco, bola: u32, cantidad: u32) -> Muneco {
        if bola == 1 { muneco.bola_inferior += cantidad; }
        if bola == 2 { muneco.bola_media += cantidad; }
        if bola == 3 { muneco.bola_superior += cantidad; }
        muneco
    }
}
