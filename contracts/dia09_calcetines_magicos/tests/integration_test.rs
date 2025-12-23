#![cfg(test)]

extern crate std;

use soroban_sdk::{Env, Symbol, Map, Vec};


use dia09_calcetines_magicos::CalcetinesContract;

#[test]
fn test_crear_calcetin_y_ver() {
    let env = Env::default();

    let mut calcetines: Map<Symbol, Vec<Symbol>> = Map::new(&env);

    // Crear calcetín para Ana
    calcetines = CalcetinesContract::crear_calcetin(env.clone(), calcetines, Symbol::new(&env, "Ana"));

    // Verificar que existe pero está vacío
    let regalos = CalcetinesContract::ver_regalos(env.clone(), calcetines.clone(), Symbol::new(&env, "Ana"));
    assert_eq!(regalos.len(), 0);

    // Ver un niño que no existe → vacío
    let regalos_no = CalcetinesContract::ver_regalos(env.clone(), calcetines.clone(), Symbol::new(&env, "Luis"));
    assert_eq!(regalos_no.len(), 0);
}

#[test]
fn test_agregar_regalo() {
    let env = Env::default();

    let mut calcetines: Map<Symbol, Vec<Symbol>> = Map::new(&env);

    // Crear calcetín para Ana
    calcetines = CalcetinesContract::crear_calcetin(env.clone(), calcetines, Symbol::new(&env, "Ana"));

    // Agregar regalo
    calcetines = CalcetinesContract::agregar_regalo(env.clone(), calcetines, Symbol::new(&env, "Ana"), Symbol::new(&env, "bici"));
    let regalos = CalcetinesContract::ver_regalos(env.clone(), calcetines.clone(), Symbol::new(&env, "Ana"));
    assert_eq!(regalos.len(), 1);
    assert_eq!(regalos.get(0).unwrap(), Symbol::new(&env, "bici"));

    // Agregar otro regalo
    calcetines = CalcetinesContract::agregar_regalo(env.clone(), calcetines, Symbol::new(&env, "Ana"), Symbol::new(&env, "pelota"));
    let regalos = CalcetinesContract::ver_regalos(env.clone(), calcetines.clone(), Symbol::new(&env, "Ana"));
    assert_eq!(regalos.len(), 2);
    assert_eq!(regalos.get(1).unwrap(), Symbol::new(&env, "pelota"));
}

#[test]
fn test_contar_regalos() {
    let env = Env::default();

    let mut calcetines: Map<Symbol, Vec<Symbol>> = Map::new(&env);

    // Crear y agregar regalos
    calcetines = CalcetinesContract::crear_calcetin(env.clone(), calcetines, Symbol::new(&env, "Ana"));
    calcetines = CalcetinesContract::agregar_regalo(env.clone(), calcetines, Symbol::new(&env, "Ana"), Symbol::new(&env, "bici"));
    calcetines = CalcetinesContract::agregar_regalo(env.clone(), calcetines, Symbol::new(&env, "Ana"), Symbol::new(&env, "pelota"));

    // Contar regalos
    let cantidad = CalcetinesContract::contar_regalos(env.clone(), calcetines.clone(), Symbol::new(&env, "Ana"));
    assert_eq!(cantidad, 2);

    // Contar regalos de un niño que no existe → 0
    let cantidad_no = CalcetinesContract::contar_regalos(env.clone(), calcetines.clone(), Symbol::new(&env, "Luis"));
    assert_eq!(cantidad_no, 0);
}

#[test]
fn test_ver_regalos_multiple_niños() {
    let env = Env::default();

    let mut calcetines: Map<Symbol, Vec<Symbol>> = Map::new(&env);

    // Agregar regalos a distintos niños
    calcetines = CalcetinesContract::agregar_regalo(env.clone(), calcetines, Symbol::new(&env, "Ana"), Symbol::new(&env, "bici"));
    calcetines = CalcetinesContract::agregar_regalo(env.clone(), calcetines, Symbol::new(&env, "Luis"), Symbol::new(&env, "pelota"));
    calcetines = CalcetinesContract::agregar_regalo(env.clone(), calcetines, Symbol::new(&env, "Luis"), Symbol::new(&env, "libro"));

    // Verificar listas
    let regalos_ana = CalcetinesContract::ver_regalos(env.clone(), calcetines.clone(), Symbol::new(&env, "Ana"));
    assert_eq!(regalos_ana.len(), 1);
    assert_eq!(regalos_ana.get(0).unwrap(), Symbol::new(&env, "bici"));

    let regalos_luis = CalcetinesContract::ver_regalos(env.clone(), calcetines.clone(), Symbol::new(&env, "Luis"));
    assert_eq!(regalos_luis.len(), 2);
    assert_eq!(regalos_luis.get(0).unwrap(), Symbol::new(&env, "pelota"));
    assert_eq!(regalos_luis.get(1).unwrap(), Symbol::new(&env, "libro"));
}
