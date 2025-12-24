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
        let mut menu: Vec<Platillo> = env.storage().instance().get(&MENU).unwrap_or(Vec::new(&env));
        let platillo = Platillo { nombre, porciones, listo: false }; 
        menu.push_back(platillo);
        env.storage().instance().set(&MENU, &menu);
    }

    /// Marca un platillo como listo
    pub fn marcar_listo(env: Env, nombre: Symbol) -> Result<(), Error> {
        let menu: Vec<Platillo> = env.storage().instance().get(&MENU).unwrap_or(Vec::new(&env));
        let mut menu_nuevo = Vec::new(&env);
        let mut encontrado = false;

        for i in 0..menu.len() { 
            let mut platillo = menu.get(i).unwrap();
            if platillo.nombre == nombre { 
                platillo.listo = true; 
                encontrado = true;
            }
            menu_nuevo.push_back(platillo); 
        }

        if !encontrado {
            return Err(Error::PlatilloNoEncontrado);
        }

        env.storage().instance().set(&MENU, &menu_nuevo);
        Ok(())
    }

    /// Verifica si todos los platillos están listos
    pub fn todos_listos(env: Env) -> bool {
        let menu: Vec<Platillo> = env.storage().instance().get(&MENU).unwrap_or(Vec::new(&env));
        
        if menu.is_empty() { return false; }

        for i in 0..menu.len() {
            if !menu.get(i).unwrap().listo { return false; } 
        } 
        true
    }

    /// Calcula el total de porciones
    pub fn porciones_totales(env: Env) -> u32 {
        let menu: Vec<Platillo> = env.storage().instance().get(&MENU).unwrap_or(Vec::new(&env));
        
        let mut total: u32 = 0; 
        for i in 0..menu.len() { 
            let platillo = menu.get(i).unwrap();
            total += platillo.porciones; 
        }
        total
    }

    /// Intenta servir la cena (solo si todos están listos)
    pub fn servir_cena(env: Env) -> Result<(), Error> {
        if !Self::todos_listos(env) { 
            return Err(Error::PlatillosNoListos); 
        } 
        Ok(())
    }
}