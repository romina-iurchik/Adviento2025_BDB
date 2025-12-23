
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Baston {
    pub sabor: Symbol,
    pub peso_gramos: u32,
    pub envoltorio: Option<Symbol>,  // Color del envoltorio (o None si no tiene)
}

#[contract]
pub struct BastonContract;

#[contractimpl]
impl BastonContract {
    /// Crea un bastón sin envolver
    pub fn crear_baston(_env: Env, sabor: Symbol, peso: u32) -> Baston {
        // TODO: Creá un Baston con envoltorio: None
        Baston {
            sabor,
            peso_gramos: peso,
            envoltorio: None,
        }
    }
}
