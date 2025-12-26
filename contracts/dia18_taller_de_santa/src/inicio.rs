#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Map, Symbol};

const JUGUETES: Symbol = symbol_short!("JUGUETES");

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Juguete {
    pub nombre: Symbol,
    pub categoria: Symbol,  // "muneca", "auto", "peluche", etc.
    pub cantidad: u32,
}

#[contract]
pub struct TallerContract;

#[contractimpl]
impl TallerContract {
    /// Registra un juguete en el taller
    pub fn registrar_juguete(env: Env, nombre: Symbol, categoria: Symbol, cantidad: u32) {
        // TODO: 1. Obtené el Map del storage (o creá uno nuevo)
        // TODO: 2. Creá el juguete
        // TODO: 3. Guardalo en el Map     
        // TODO: 4. Guardá el Map en el storage
    }
    
    /// Obtiene un juguete por nombre
    pub fn obtener_juguete(env: Env, nombre: Symbol) -> Option<Juguete> {
        // TODO: Obtené el Map del storage
        // TODO: Buscá el juguete

        
        None
    }
    
    /// Verifica si existe un juguete
    pub fn existe_juguete(env: Env, nombre: Symbol) -> bool {
        // TODO: Obtené el Map y usá .contains_key()
        false
    }
    
    /// Cuenta juguetes de una categoría específica
    pub fn contar_por_categoria(env: Env, categoria: Symbol) -> u32 {
        // TODO: Obtené el Map del storage
        // TODO: Inicializá contador
        // TODO: Recorré el Map usando .keys()

        
        0
    }
    
    /// Actualiza la cantidad fabricada de un juguete
    pub fn actualizar_cantidad(env: Env, nombre: Symbol, nueva_cantidad: u32) {
        // TODO: Obtené el Map
        // TODO: Obtené el juguete

    }
}