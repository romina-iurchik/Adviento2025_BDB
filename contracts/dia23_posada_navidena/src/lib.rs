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
            capacidad,
            ocupados: 0,
        }
    }
    
    /// Reserva espacio en una habitación
    pub fn reservar(env: Env, numero: u32, cantidad: u32) -> Result<(), Error> {
        // TODO: Obtené el Map del storage
        let mut habitaciones:Map<u32,Habitacion> =
            env.storage()
            .persistent()
            .get(&HABITACIONES)
            .unwrap_or(Map::new(&env));

        // TODO: Obtené la habitación
        let mut habitacion = habitaciones.get(numero).ok_or(Error::HabitacionNoExiste)?;

        // TODO: Verificá que haya espacio
        if habitacion.ocupados + cantidad > habitacion.capacidad {
            return Err(Error::SinEspacio);
        }

        // TODO: Sumá a ocupados
        habitacion.ocupados += cantidad;

        // TODO: Guardá la habitación actualizada
        habitaciones.set(numero, habitacion);

        env.storage().persistent().set(&HABITACIONES, &habitaciones);
        
        Ok(())
    }
    
    /// Libera espacio ocupado
    pub fn liberar(env: Env, numero: u32, cantidad: u32) -> Result<(), Error> {
        // TODO: Similar a reservar, pero restando
        // Cuidado: ocupados no puede ser negativo

        let mut habitaciones:Map<u32,Habitacion> =
            env.storage()
            .persistent()
            .get(&HABITACIONES)
            .unwrap_or(Map::new(&env));
        
        let mut habitacion = habitaciones.get(numero).ok_or(Error::HabitacionNoExiste)?;
    
        if habitacion.ocupados < cantidad {
            return Err(Error::SinEspacio);
        }
        
        habitacion.ocupados -= cantidad;
        habitaciones.set(numero, habitacion);
        env.storage().persistent().set(&HABITACIONES, &habitaciones);
        
        Ok(())
    }
    
    /// Verifica si hay espacio disponible
    pub fn esta_disponible(env: Env, numero: u32) -> Result<bool, Error> {
        // TODO: Obtené la habitación
        // Retorná true si ocupados < capacidad

        let habitaciones:Map<u32,Habitacion> =
            env.storage()
            .persistent()
            .get(&HABITACIONES)
            .unwrap_or(Map::new(&env));
        
        let habitacion = habitaciones.get(numero).ok_or(Error::HabitacionNoExiste)?;
    
        if habitacion.ocupados < habitacion.capacidad {
            Ok(true)
        } else { Ok(false) }
        
        
    }
    
/// Calcula la ocupación total de la posada
pub fn ocupacion_total(env: Env) -> u32 {
    // Obtén el Map con las habitaciones
    let habitaciones: Map<u32, Habitacion> =
        env.storage()
            .persistent()
            .get(&HABITACIONES)
            .unwrap_or(Map::new(&env));

    // Inicializamos el contador total
    let mut total = 0;

    // Obtenemos las claves del mapa
    let keys = habitaciones.keys();

    // Iteramos sobre las claves
    for i in 0..keys.len() {
        let key = keys.get(i).unwrap(); // Obtenemos la clave
        if let Some(habitacion) = habitaciones.get(key) {
            total += habitacion.ocupados; // Sumamos los ocupados de esa habitación
        }
    }

    total // Devolvemos el total de ocupados
}


    /// Guarda una habitación en storage
    pub fn guardar_habitacion(env: Env, habitacion: Habitacion) {
        // TODO: Obtené el Map del storage
        let mut habitaciones:Map<u32,Habitacion> =
            env.storage()
            .persistent()
            .get(&HABITACIONES)
            .unwrap_or(Map::new(&env));

        // TODO: Guardá la habitación usando su número como clave
        habitaciones.set(habitacion.numero, habitacion);
        // TODO: Guardá el Map
        env.storage().persistent().set(&HABITACIONES, &habitaciones);
    }
}

