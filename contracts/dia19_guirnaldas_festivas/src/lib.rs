
#![no_std]
use soroban_sdk::{Env, Symbol, Vec, contract, contractimpl, contracttype};

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
            let mut luces= Vec::<Luz>::new(&env);

        // TODO: Agregá 'cantidad' luces apagadas
        for i in 0..cantidad {
            let luz = Luz {
                posicion: i,
                color: color.clone(),
                encendida: false,
            };
            luces.push_back(luz);
        }
        luces
    }
    
    /// Enciende una luz en posición específica
    pub fn encender_luz(env: Env, luces: Vec<Luz>, posicion: u32) -> Vec<Luz> {
        // TODO: Creá un nuevo Vec
        let mut luces_nuevo: Vec<Luz> = Vec::new(&env);
        
        // TODO: Recorré las luces
        for i in 0..luces.len() {
            let mut luz = luces.get(i).unwrap();
            if luz.posicion == posicion {
                luz.encendida = true;
            }
            luces_nuevo.push_back(luz);
        }
        
        luces_nuevo
    }
    
    /// Alterna el estado de una luz (on/off)
    pub fn alternar_luz(env: Env, luces: Vec<Luz>, posicion: u32) -> Vec<Luz> {
        // TODO: Similar a encender_luz, pero usá !luz.encendida
        let mut luces_nuevo: Vec<Luz> = Vec::new(&env);
        
        // TODO: Recorré las luces
        for i in 0..luces.len() {
            let mut luz = luces.get(i).unwrap();
            if luz.posicion == posicion {
                luz.encendida =  !luz.encendida;
            }
            luces_nuevo.push_back(luz);
        }
        
        luces_nuevo
        
    }
    
    /// Enciende luces alternadas (pares o impares)
    pub fn encender_alternadas(env: Env, luces: Vec<Luz>, pares: bool) -> Vec<Luz> {
        // TODO: Creá un nuevo Vec
        let mut luces_nuevo: Vec<Luz> = Vec::new(&env);
        
        // TODO: Recorré las luces
        for i in 0..luces.len() {
            let mut luz = luces.get(i).unwrap();
            if (luz.posicion % 2 == 0 && pares) || (luz.posicion % 2 == 1 && !pares) {
                luz.encendida =  true;
            }
            luces_nuevo.push_back(luz);
        }
        
        luces_nuevo
    }
    
    /// Cuenta cuántas luces están encendidas
    pub fn contar_encendidas(_env: Env, luces: Vec<Luz>) -> u32 {
        // TODO: Inicializá un contador
        let mut contador = 0;
        // TODO: Recorré y contá las encendidas
        for i in 0..luces.len() {
            let luz = luces.get(i).unwrap();
            if luz.encendida {
                contador+=1;
            }
            

        }

        contador
    }
}
