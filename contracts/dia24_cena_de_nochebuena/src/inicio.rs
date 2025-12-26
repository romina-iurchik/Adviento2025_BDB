// 🍽️ Día 24: Cena de Nochebuena

#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, contracttype, symbol_short, Env, Symbol, Vec};

const MENU: Symbol = symbol_short!("MENU");

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    PlatilloNoEncontrado = 1,
    PlatillosNoListos = 2,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Platillo {
    pub nombre: Symbol,
    pub porciones: u32,
    pub listo: bool,
}

#[contract]
pub struct CenaContract;

#[contractimpl]
impl CenaContract {
    /// Agrega un platillo al menú
    pub fn agregar_platillo(env: Env, nombre: Symbol, porciones: u32) {
        // TODO: Obtené el Vec del storage
        // TODO: Creá un nuevo Platillo con listo: false
        // TODO: Agregá el platillo al Vec
        // TODO: Guardá el Vec
    }
    
    /// Marca un platillo como listo
    pub fn marcar_listo(env: Env, nombre: Symbol) -> Result<(), Error> {
        // TODO: Obtené el Vec del storage
        // TODO: Creá un nuevo Vec
        // TODO: Iterá y cuando encuentres el platillo, cambiá listo a true
        // TODO: Si no lo encontrás, retorná Error::PlatilloNoEncontrado
        // TODO: Guardá el nuevo Vec
        
        Ok(())
    }
    
    /// Verifica si todos los platillos están listos
    pub fn todos_listos(env: Env) -> bool {
        // TODO: Obtené el Vec
        // TODO: Si está vacío retorná false
        // TODO: Iterá y si encontrás uno no listo, retorná false
        // TODO: Si todos están listos, retorná true
        
        false
    }
    
    /// Calcula el total de porciones
    pub fn porciones_totales(env: Env) -> u32 {
        // TODO: Obtené el Vec
        // TODO: Sumá todas las porciones
        
        0
    }
    
    /// Intenta servir la cena (solo si todos están listos)
    pub fn servir_cena(env: Env) -> Result<(), Error> {
        // TODO: Verificá que todos los platillos estén listos
        // TODO: Si no, retorná Error::PlatillosNoListos
        
        Ok(())
    }
}