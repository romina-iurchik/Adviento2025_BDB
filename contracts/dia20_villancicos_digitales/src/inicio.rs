#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Symbol, Vec};

const PLAYLIST: Symbol = symbol_short!("PLAYLIST");

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Villancico {
    pub titulo: Symbol,
    pub artista: Symbol,
    pub duracion_segundos: u32,
}

#[contract]
pub struct VillancicoContract;

#[contractimpl]
impl VillancicoContract {
    /// Agrega un villancico a la playlist
    pub fn agregar_villancico(env: Env, titulo: Symbol, artista: Symbol, duracion: u32) {
        // TODO: Obtené el Vec del storage (o creá uno nuevo)
        // TODO: Creá el villancico
        // TODO: Agregalo al Vec
        // TODO: Guardá el Vec en storage
    }
    
    /// Obtiene toda la playlist
    pub fn obtener_playlist(env: Env) -> Vec<Villancico> {
        // TODO: Obtené y retorná el Vec del storage
        Vec::new(&env)
    }
    
    /// Calcula la duración total de la playlist en segundos
    pub fn duracion_total(env: Env) -> u32 {
        // TODO: Obtené el Vec del storage
        // TODO: Sumá todas las duraciones
        0
    }
    
    /// Busca villancicos por artista
    pub fn buscar_por_artista(env: Env, artista: Symbol) -> Vec<Villancico> {
        // TODO: Obtené el Vec del storage
        // TODO: Creá un nuevo Vec para los resultados
        // TODO: Agregá los que coincidan con el artista
        Vec::new(&env)
    }
    
    /// Encuentra el villancico más largo
    pub fn villancico_mas_largo(env: Env) -> Option<Villancico> {
        // TODO: Obtené el Vec del storage
        // TODO: Si está vacío, retorná None
        // TODO: Encontrá el de mayor duración
        None
    }
}