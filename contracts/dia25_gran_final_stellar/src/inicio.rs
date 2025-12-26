// ✨ Día 25: Gran Final Estelar

#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, contracttype, symbol_short, Env, Symbol, Vec};

const REGALOS: Symbol = symbol_short!("REGALOS");
const CONTADOR: Symbol = symbol_short!("CONTADOR");

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    RegaloNoEncontrado = 1,
    YaEntregado = 2,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Prioridad {
    Alta,
    Media,
    Baja,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Regalo {
    pub id: u32,
    pub destinatario: Symbol,
    pub descripcion: Symbol,
    pub prioridad: Prioridad,
    pub entregado: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Estadisticas {
    pub total_regalos: u32,
    pub entregados: u32,
    pub pendientes: u32,
    pub alta_prioridad: u32,
}

#[contract]
pub struct NavidadContract;

#[contractimpl]
impl NavidadContract {
    pub fn registrar_regalo(
        env: Env,
        destinatario: Symbol,
        descripcion: Symbol,
        prioridad: Prioridad
    ) -> u32 {
        // TODO: Implementar
        0
    }
    
    pub fn marcar_entregado(env: Env, id: u32) -> Result<(), Error> {
        // TODO: Implementar
        Ok(())
    }
    
    pub fn obtener_regalos_pendientes(env: Env) -> Vec<Regalo> {
        // TODO: Implementar
        Vec::new(&env)
    }
    
    pub fn obtener_por_destinatario(env: Env, destinatario: Symbol) -> Vec<Regalo> {
        // TODO: Implementar
        Vec::new(&env)
    }
    
    pub fn calcular_estadisticas(env: Env) -> Estadisticas {
        // TODO: Implementar
        Estadisticas {
            total_regalos: 0,
            entregados: 0,
            pendientes: 0,
            alta_prioridad: 0,
        }
    }
    
    pub fn eliminar_regalo(env: Env, id: u32) -> Result<(), Error> {
        // TODO: Implementar
        Ok(())
    }
    
    pub fn total_por_prioridad(env: Env, prioridad: Prioridad) -> u32 {
        // TODO: Implementar
        0
    }
    
    pub fn obtener_todos(env: Env) -> Vec<Regalo> {
        env.storage()
            .instance()
            .get(&REGALOS)
            .unwrap_or(Vec::new(&env))
    }
}