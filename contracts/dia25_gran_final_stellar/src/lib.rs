// ✨ Día 25: Gran Final Estelar - Versión Final In-Place

#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, contracttype, symbol_short, Env, Symbol, Vec};

const REGALOS: Symbol = symbol_short!("REGALOS");
const CONTADOR: Symbol = symbol_short!("CONTADOR");

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    RegaloNoEncontrado = 1,
    YaEntregado = 2,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Prioridad {
    Alta,
    Media,
    Baja,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Regalo {
    pub id: u32,
    pub destinatario: Symbol,
    pub descripcion: Symbol,
    pub prioridad: Prioridad,
    pub entregado: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Estadisticas {
    pub total_regalos: u32,
    pub entregados: u32,
    pub pendientes: u32,
    pub alta_prioridad: u32,
}

#[contract]
pub struct NavidadContract;

#[contractimpl]
impl NavidadContract {
    pub fn registrar_regalo(
        env: Env,
        destinatario: Symbol,
        descripcion: Symbol,
        prioridad: Prioridad
    ) -> u32 {
        let contador: u32 = env.storage().instance().get(&CONTADOR).unwrap_or(0);
        let nuevo_id = contador + 1;

        let mut regalos: Vec<Regalo> = env.storage().instance().get(&REGALOS).unwrap_or(Vec::new(&env));

        let nuevo_regalo = Regalo {
            id: nuevo_id,
            destinatario,
            descripcion,
            prioridad,
            entregado: false,
        };


        regalos.push_back(nuevo_regalo);
        
        env.storage().instance().set(&REGALOS, &regalos);
        env.storage().instance().set(&CONTADOR, &nuevo_id);

        nuevo_id
    }
    
    pub fn marcar_entregado(env: Env, id: u32) -> Result<(), Error> {
        let regalos: Vec<Regalo> = env.storage().instance().get(&REGALOS).unwrap_or(Vec::new(&env));
        let mut regalos_nuevo = Vec::new(&env);
        let mut encontrado = false;

        for i in 0..regalos.len() {
            let mut regalo = regalos.get(i).unwrap();
            if regalo.id == id {
                if regalo.entregado { return Err(Error::YaEntregado); }
                regalo.entregado = true;
                encontrado = true;
            }
            regalos_nuevo.push_back(regalo);
        }
        
        if !encontrado { return Err(Error::RegaloNoEncontrado); }
        env.storage().instance().set(&REGALOS, &regalos_nuevo);
        Ok(())
    }
    
    pub fn obtener_regalos_pendientes(env: Env) -> Vec<Regalo> {
        let regalos: Vec<Regalo> = env.storage().instance().get(&REGALOS).unwrap_or(Vec::new(&env));
        let mut pendientes = Vec::new(&env);

        for i in 0..regalos.len() {
            let regalo = regalos.get(i).unwrap();
            if !regalo.entregado {
                pendientes.push_back(regalo);
            }
        }
        pendientes
    }
    
    pub fn obtener_por_destinatario(env: Env, destinatario: Symbol) -> Vec<Regalo> {

        let regalos: Vec<Regalo> = env.storage().instance().get(&REGALOS).unwrap_or(Vec::new(&env)); 
        let mut resultado = Vec::new(&env);


        for i in 0..regalos.len() {
            let regalo = regalos.get(i).unwrap();


            if regalo.destinatario == destinatario {

                resultado.push_back(regalo);
            }
        }

        resultado
    } 
    pub fn calcular_estadisticas(env: Env) -> Estadisticas {
        let regalos: Vec<Regalo> = env.storage().instance().get(&REGALOS).unwrap_or(Vec::new(&env));
        let mut stats = Estadisticas { total_regalos: 0, entregados: 0, pendientes: 0, alta_prioridad: 0 };

        for i in 0..regalos.len() {
            let regalo = regalos.get(i).unwrap();
            stats.total_regalos += 1;
            if regalo.entregado { stats.entregados += 1; } else { stats.pendientes += 1; }
            if regalo.prioridad == Prioridad::Alta { stats.alta_prioridad += 1; }
        }

        stats
    }
    
    pub fn eliminar_regalo(env: Env, id: u32) -> Result<(), Error> {
        let regalos: Vec<Regalo> = env.storage().instance().get(&REGALOS).unwrap_or(Vec::new(&env));
        let mut lista_nueva = Vec::new(&env);
        let mut encontrado = false;

        for i in 0..regalos.len() {
        let regalo = regalos.get(i).unwrap();
        if regalo.id == id {
                encontrado = true;
                continue;
        }
        lista_nueva.push_back(regalo);
        }

        if !encontrado { return Err(Error::RegaloNoEncontrado); }
        env.storage().instance().set(&REGALOS, &lista_nueva);
        Ok(())
    }

    pub fn total_por_prioridad(env: Env, prioridad: Prioridad) -> u32 {
        let regalos: Vec<Regalo> = env.storage().instance().get(&REGALOS).unwrap_or(Vec::new(&env));
        let mut contador = 0;
        for i in 0..regalos.len() {
            let regalo = regalos.get(i).unwrap();
            if regalo.prioridad == prioridad.clone() {
                contador += 1;
            }
        }
        contador
    }
    
    pub fn obtener_todos(env: Env) -> Vec<Regalo> {
        env.storage().instance().get(&REGALOS).unwrap_or(Vec::new(&env))
    }
}