#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Luz {
    pub posicion: u32,
    pub color: Symbol,
    pub encendida: bool,
}

#[contract]
pub struct GuirnaldasContract;

#[contractimpl]
impl GuirnaldasContract {
    /// Crea una guirnalda con N luces apagadas
    pub fn crear_guirnalda(env: Env, cantidad: u32, color: Symbol) -> Vec<Luz> {
        // TODO: Creá un Vec vacío
        // TODO: Agregá 'cantidad' luces apagadas
        
        Vec::new(&env)
    }
    
    /// Enciende una luz en posición específica
    pub fn encender_luz(env: Env, luces: Vec<Luz>, posicion: u32) -> Vec<Luz> {
        // TODO: Creá un nuevo Vec
        let mut luces_nuevo = Vec::new(&env);
        
        // TODO: Recorré las luces

        
        luces_nuevo
    }
    
    /// Alterna el estado de una luz (on/off)
    pub fn alternar_luz(env: Env, luces: Vec<Luz>, posicion: u32) -> Vec<Luz> {
        // TODO: Similar a encender_luz, pero usá !luz.encendida
        
        Vec::new(&env)
    }
    
    /// Enciende luces alternadas (pares o impares)
    pub fn encender_alternadas(env: Env, luces: Vec<Luz>, pares: bool) -> Vec<Luz> {
        // TODO: Creá un nuevo Vec
        let mut luces_nuevo = Vec::new(&env);
        // TODO: Recorré las luces
        
        luces_nuevo
    }
    
    /// Cuenta cuántas luces están encendidas
    pub fn contar_encendidas(_env: Env, luces: Vec<Luz>) -> u32 {
        // TODO: Inicializá un contador
        let mut contador = 0;
        // TODO: Recorré y contá las encendidas

        contador
    }
}