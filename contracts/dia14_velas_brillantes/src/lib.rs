#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Color {
    Rojo,
    Verde,
    Dorado,
    Blanco,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Vela {
    pub color: Color,
    pub altura_inicial: u32,
    pub consumida: u32,
}

#[contract]
pub struct VelasContract;

#[contractimpl]
impl VelasContract {
    pub fn crear_vela(_env: Env, color: Color, altura: u32) -> Vela {
        // TODO
        Vela {
            color,
            altura_inicial: altura,
            consumida: 0,
        }
    }
    
    pub fn encender(_env: Env, mut vela: Vela, centimetros: u32) -> Vela {
        // TODO
        vela.consumida += centimetros;
        if vela.consumida > vela.altura_inicial { 
            vela.consumida = vela.altura_inicial; 
        }
        vela
    }
    
    pub fn altura_actual(_env: Env, vela: Vela) -> u32 {
        vela.altura_inicial - vela.consumida
    }
    
    pub fn esta_apagada(_env: Env, vela: Vela) -> bool {
        vela.consumida >= vela.altura_inicial
    }
    
    pub fn cambiar_color(_env: Env, mut vela: Vela, nuevo_color: Color) -> Vela {
        vela.color = nuevo_color;
        vela
    }
}