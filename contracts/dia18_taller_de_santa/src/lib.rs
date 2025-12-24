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
        let mut juguetes: Map<Symbol, Juguete> =
            env.storage()
            .persistent()
            .get(&JUGUETES)
            .unwrap_or(Map::new(&env));

        // TODO: 2. Creá el juguete
        let juguete =Juguete {
            nombre: nombre.clone(),
            categoria,
            cantidad,
        };

        // TODO: 3. Guardalo en el Map
        juguetes.set(nombre, juguete);

        // TODO: 4. Guardá el Map en el storage
        env.storage()
        .persistent()
        .set(&JUGUETES, &juguetes);
    }
    
    /// Obtiene un juguete por nombre
    pub fn obtener_juguete(env: Env, nombre: Symbol) -> Option<Juguete> {
        // TODO: Obtené el Map del storage
        let juguetes: Map<Symbol, Juguete> =
            env.storage()
            .persistent()
            .get(&JUGUETES)
            .unwrap_or(Map::new(&env));

        // TODO: Buscá el juguete
        juguetes.get(nombre)
    }
    
    /// Verifica si existe un juguete
    pub fn existe_juguete(env: Env, nombre: Symbol) -> bool {
        // TODO: Obtené el Map y usá .contains_key()
        let juguetes: Map<Symbol, Juguete> =
            env.storage()
            .persistent()
            .get(&JUGUETES)
            .unwrap_or(Map::new(&env));

        juguetes.contains_key(nombre)
    }
    
    /// Cuenta juguetes de una categoría específica
    pub fn contar_por_categoria(env: Env, categoria: Symbol) -> u32 {
        // TODO: Obtené el Map del storage
        let juguetes: Map<Symbol, Juguete> =
            env.storage()
            .persistent()
            .get(&JUGUETES)
            .unwrap_or(Map::new(&env));

        // TODO: Inicializá contador
        let mut contador:u32 = 0;

        // TODO: Recorré el Map usando .keys()
        let keys = juguetes.keys();
        for i in 0..keys.len(){
            let key = keys.get(i).unwrap();
            if let Some(juguete) = juguetes.get(key){
                if juguete.categoria == categoria {
                    contador += 1;
                }
            }
        }
        contador
    }
    
    /// Actualiza la cantidad fabricada de un juguete
    pub fn actualizar_cantidad(env: Env, nombre: Symbol, nueva_cantidad: u32) {
        // TODO: Obtené el Map
        let mut juguetes: Map<Symbol, Juguete> =
            env.storage()
            .persistent()
            .get(&JUGUETES)
            .unwrap_or(Map::new(&env));

        // TODO: Obtené el juguete
        if let Some(mut juguete) = juguetes.get(nombre.clone()){

            //Modifica el campo cantidad
            juguete.cantidad = nueva_cantidad;

            //Guarda el juguete actualizado en el Map
            juguetes.set(nombre, juguete);

            //Guarda el map completo en el storage
            env.storage()
                .persistent()
                .set(&JUGUETES, &juguetes);
        }
    }
}