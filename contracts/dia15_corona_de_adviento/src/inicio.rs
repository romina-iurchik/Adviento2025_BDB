#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VelaCorona {
    pub numero: u32,
    pub encendida: bool,
}

#[contract]
pub struct CoronaContract;

#[contractimpl]
impl CoronaContract {
    pub fn crear_corona(env: Env) -> Vec<VelaCorona> {
        // TODO: Crear 4 velas con números del 1 al 4, todas apagadas
        
        Vec::new(&env)
    }
    
    pub fn encender_siguiente(env: Env, velas: Vec<VelaCorona>) -> Vec<VelaCorona> {
        // TODO: Encender la primera vela apagada
        let mut velas_nuevo = Vec::new(&env);
        
        let mut ya_encendida = false;
        // TODO: Iterar sobre las velas
        
        velas_nuevo
    }
    
    pub fn contar_encendidas(_env: Env, velas: Vec<VelaCorona>) -> u32 {
        // TODO: Contar cuántas velas están encendidas
        
        0
    }
    
    pub fn todas_encendidas(env: Env, velas: Vec<VelaCorona>) -> bool {
        // TODO: Verificar si las 4 velas están encendidas
        false
    }
    
    pub fn apagar_todas(env: Env, velas: Vec<VelaCorona>) -> Vec<VelaCorona> {
        // TODO: Apagar todas las velas
        
        Vec::new(&env)
    }
}