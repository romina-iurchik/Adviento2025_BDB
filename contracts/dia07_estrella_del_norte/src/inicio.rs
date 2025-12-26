#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, Env};

// 🎄 Día 7: Estrella del Norte
// Manejo de Errores Personalizados

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    EnergiaInsuficiente = 1,
    YaBrillando = 2,
}

#[contract]
pub struct EstrellaContract;

#[contractimpl]
impl EstrellaContract {
    /// Enciende la estrella si tiene al menos 10 de energía
    pub fn encender(_env: Env, energia: u32) -> Result<u32, Error> {
        // TODO: Si energia < 10, retorná Err(Error::EnergiaInsuficiente)
        // Si no, retorná Ok(energia)
        Ok(energia)
    }
    
    /// Verifica si la estrella puede brillar
    pub fn puede_brillar(_env: Env, energia: u32, ya_encendida: bool) -> Result<bool, Error> {
        // TODO: 
        // Si ya_encendida == true, retorná Err(Error::YaBrillando)
        // Si energia < 10, retorná Err(Error::EnergiaInsuficiente)
        // Si no, retorná Ok(true)
        Ok(true)
    }
    
    /// Incrementa energía de forma segura
    pub fn cargar_energia(_env: Env, actual: u32, incremento: u32) -> Result<u32, Error> {
        // TODO: Sumá actual + incremento
        // Si el resultado es < 10, retorná Err(Error::EnergiaInsuficiente)
        // Si no, retorná Ok(resultado)
        Ok(actual + incremento)
    }
}