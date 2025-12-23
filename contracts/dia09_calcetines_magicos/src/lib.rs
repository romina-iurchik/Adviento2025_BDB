#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Map, Symbol, Vec};

// 🎄 Día 9: Calcetines Mágicos
// Estructuras de Datos Anidadas en Soroban

#[contract]
pub struct CalcetinesContract;

#[contractimpl]
impl CalcetinesContract {
    pub fn crear_calcetin(env: Env, mut calcetines: Map<Symbol, Vec<Symbol>>, niño: Symbol) -> Map<Symbol, Vec<Symbol>> {
        // TODO
        let value = Vec::new(&env);
        calcetines.set(niño, value);
        calcetines
    }
    
    pub fn agregar_regalo(env: Env, mut calcetines: Map<Symbol, Vec<Symbol>>, niño: Symbol, regalo: Symbol) -> Map<Symbol, Vec<Symbol>> {
        // TODO:
        let mut regalos = calcetines.get(niño.clone()).unwrap_or(Vec::new(&env));
        regalos.push_back(regalo);
        calcetines.set(niño, regalos);
        calcetines
    }
    
    pub fn contar_regalos(env: Env, calcetines: Map<Symbol, Vec<Symbol>>, niño: Symbol) -> u32 {
        // TODO
        calcetines.get(niño).unwrap_or(Vec::new(&env)).len()
    }
    
    pub fn ver_regalos(env: Env, calcetines: Map<Symbol, Vec<Symbol>>, niño: Symbol) -> Vec<Symbol> {
        // TODO
        calcetines.get(niño).unwrap_or(Vec::new(&env))
    }
}