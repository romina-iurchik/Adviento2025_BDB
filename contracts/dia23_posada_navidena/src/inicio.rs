// 🏠 Día 23: Posada Navideña

#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, contracttype, symbol_short, Env, Map, Symbol};

const HABITACIONES: Symbol = symbol_short!("HABITA");

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    SinEspacio = 1,
    HabitacionNoExiste = 2,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Habitacion {
    pub numero: u32,
    pub capacidad: u32,
    pub ocupados: u32,
}

#[contract]
pub struct PosadaContract;

#[contractimpl]
impl PosadaContract {
    /// Crea una habitación con capacidad
    pub fn crear_habitacion(_env: Env, numero: u32, capacidad: u32) -> Habitacion {
        // TODO: Creá una habitación con ocupados: 0
        Habitacion {
            numero,
            capacidad: 0,
            ocupados: 0,
        }
    }
    
    /// Reserva espacio en una habitación
    pub fn reservar(env: Env, numero: u32, cantidad: u32) -> Result<(), Error> {
        // TODO: Obtené el Map del storage
        // TODO: Obtené la habitación
        // TODO: Verificá que haya espacio
        // TODO: Sumá a ocupados
        // TODO: Guardá la habitación actualizada
        
        Ok(())
    }
    
    /// Libera espacio ocupado
    pub fn liberar(env: Env, numero: u32, cantidad: u32) -> Result<(), Error> {
        // TODO: Similar a reservar, pero restando
        // Cuidado: ocupados no puede ser negativo
        
        Ok(())
    }
    
    /// Verifica si hay espacio disponible
    pub fn esta_disponible(env: Env, numero: u32) -> Result<bool, Error> {
        // TODO: Obtené la habitación
        // Retorná true si ocupados < capacidad
        
        Ok(false)
    }
    
    /// Calcula la ocupación total de la posada
    pub fn ocupacion_total(env: Env) -> u32 {
        // TODO: Obtené el Map
        // Sumá todos los ocupados de todas las habitaciones
        
        0
    }
    
    /// Guarda una habitación en storage
    pub fn guardar_habitacion(env: Env, habitacion: Habitacion) {
        // TODO: Obtené el Map del storage
        // TODO: Guardá la habitación usando su número como clave
        // TODO: Guardá el Map
    }
}