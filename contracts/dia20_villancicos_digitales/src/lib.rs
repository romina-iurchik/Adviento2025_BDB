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
        let mut villancicos: Vec<Villancico> =
                env.storage()
                .instance()
                .get(&PLAYLIST)
                .unwrap_or(Vec::new(&env));

        // TODO: Creá el villancico
        let nuevo_villancico = Villancico {
            titulo, artista, duracion_segundos: duracion
        };

        // TODO: Agregalo al Vec
        villancicos.push_back(nuevo_villancico);

        // TODO: Guardá el Vec en storage
        env.storage().instance().set(&PLAYLIST, &villancicos);
    }
    
    /// Obtiene toda la playlist
    pub fn obtener_playlist(env: Env) -> Vec<Villancico> {
        // TODO: Obtené y retorná el Vec del storage
                let villancicos: Vec<Villancico> =
                env.storage()
                .instance()
                .get(&PLAYLIST)
                .unwrap_or(Vec::new(&env));
        villancicos
    }
    
    /// Calcula la duración total de la playlist en segundos
    pub fn duracion_total(env: Env) -> u32 {
        // TODO: Obtené el Vec del storage
        let villancicos: Vec<Villancico> =
                env.storage()
                .instance()
                .get(&PLAYLIST)
                .unwrap_or(Vec::new(&env));

        // TODO: Sumá todas las duraciones
        let mut total = 0;
        for i in 0..villancicos.len() {
            let v = villancicos.get(i).unwrap();
            total += v.duracion_segundos;
        }
        total
        
    }
    
    /// Busca villancicos por artista
    pub fn buscar_por_artista(env: Env, artista: Symbol) -> Vec<Villancico> {
        // TODO: Obtené el Vec del storage
        let villancicos: Vec<Villancico> =
                env.storage()
                .instance()
                .get(&PLAYLIST)
                .unwrap_or(Vec::new(&env));

        // TODO: Creá un nuevo Vec para los resultados
        let mut resultados: Vec<Villancico> = Vec::new(&env);
        
        // TODO: Agregá los que coincidan con el artista
        for i in 0..villancicos.len() {
            let v = villancicos.get(i).unwrap();
            if v.artista == artista {
                resultados.push_back(v);
            }
        }
        resultados
    }
    
    /// Encuentra el villancico más largo
pub fn villancico_mas_largo(env: Env) -> Option<Villancico> {
    let villancicos: Vec<Villancico> =
        env.storage()
        .instance()
        .get(&PLAYLIST)
        .unwrap_or(Vec::new(&env));

    if villancicos.is_empty() {
        return None;
    }

    let mut mas_largo = villancicos.get(0).unwrap().clone();

    for i in 1..villancicos.len() {
        let v = villancicos.get(i).unwrap();
        if v.duracion_segundos > mas_largo.duracion_segundos {
            mas_largo = v.clone();
        }
    }

    Some(mas_largo)
}
}