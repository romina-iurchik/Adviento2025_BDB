#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol};

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
    pub fn crear_reno(_env: Env, nombre: Symbol, velocidad: u32) -> Reno {
        // TODO
        Reno {
            nombre,
            velocidad,
            energia: 100,
        }
        

    }
    
    pub fn volar(_env: Env, mut reno: Reno, distancia: u32) -> Reno {
        let consumo = distancia / reno.velocidad;
        if consumo > reno.energia {
            reno.energia = 0;
        } else {
            reno.energia -= consumo;
        }        
        reno
    }
    
    pub fn descansar(_env: Env, mut reno: Reno, cantidad: u32) -> Reno {
        let nueva_energia = reno.energia.saturating_add(cantidad);
        if nueva_energia > 100 {
            reno.energia = 100;
        } else {
            reno.energia = nueva_energia ;
        }
        reno
    }
    
    pub fn puede_volar(_env: Env, reno: Reno, distancia: u32) -> bool {
        let necesario = distancia / reno.velocidad;
        necesario <= reno.energia
    }
}