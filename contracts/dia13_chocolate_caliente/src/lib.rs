#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, symbol_short};

// ❄️ Día 13: Chocolate Caliente
// Storage + Structs Complejos

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Taza {
    pub cacao: u32,
    pub leche: u32,
    pub azucar: u32,
    pub temperatura: u32,
}

#[contract]
pub struct ChocolateContract;

#[contractimpl]
impl ChocolateContract {
    /// Prepara una taza de chocolate con los ingredientes dados
    pub fn preparar_taza(_env: Env, cacao: u32, leche: u32, azucar: u32) -> Taza {
        // TODO
        Taza {
            cacao,
            leche,
            azucar,
            temperatura: 25,
        }

    }
    
    /// Calienta el chocolate (incrementa temperatura)
    pub fn calentar(_env: Env, mut taza: Taza, grados: u32) -> Taza {
        taza.temperatura += grados;
        taza
    }
    
    pub fn enfriar(_env: Env, mut taza: Taza, grados: u32) -> Taza {
        if grados > taza.temperatura {
            taza.temperatura = 0;
        } else {
            taza.temperatura -= grados;
        }        
        taza
    }
    
    /// Verifica si el chocolate está en temperatura perfecta (60-80°C)
    pub fn esta_perfecta(_env: Env, taza: Taza) -> bool {
        taza.temperatura >= 60 && taza.temperatura <= 80

    }
    
    /// Agrega más de un ingrediente
    pub fn agregar_ingrediente(_env: Env, mut taza: Taza, ingrediente: Symbol, cantidad: u32) -> Taza {
if ingrediente == symbol_short!("cacao") { taza.cacao += cantidad }
if ingrediente == symbol_short!("leche") { taza.leche += cantidad }
if ingrediente == symbol_short!("azucar") { taza.azucar += cantidad }
        taza
    }
}