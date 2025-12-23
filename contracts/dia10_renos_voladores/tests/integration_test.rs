#![cfg(test)]

extern crate std;

use soroban_sdk::{Env, Symbol};


use dia10_renos_voladores::{RenosContract};

#[test]
fn test_crear_reno() {
    let env = Env::default();

    let reno = RenosContract::crear_reno(env.clone(), Symbol::new(&env, "Rudolph"), 10);

    assert_eq!(reno.nombre, Symbol::new(&env, "Rudolph"));
    assert_eq!(reno.velocidad, 10);
    assert_eq!(reno.energia, 100);
}

#[test]
fn test_volar_consumo() {
    let env = Env::default();

    let reno = RenosContract::crear_reno(env.clone(), Symbol::new(&env, "Rudolph"), 10);

    // Volar 50 → consumo = 50 / 10 = 5
    let reno_volado = RenosContract::volar(env.clone(), reno.clone(), 50);
    assert_eq!(reno_volado.energia, 95);

    // Volar con distancia que supera la energía → energia se pone a 0
    let reno_bajo = RenosContract::volar(env.clone(), reno_volado.clone(), 2000); // consumo = 200
    assert_eq!(reno_bajo.energia, 0);
}

#[test]
fn test_descansar() {
    let env = Env::default();

    let reno = RenosContract::crear_reno(env.clone(), Symbol::new(&env, "Rudolph"), 10);
    let reno_volado = RenosContract::volar(env.clone(), reno.clone(), 50); // energia 95

    // Descansar 30 → energia = 95 + 30 = 100 (máximo 100)
    let reno_descansado = RenosContract::descansar(env.clone(), reno_volado.clone(), 30);
    assert_eq!(reno_descansado.energia, 100);

    // Descansar 10 → energia = 95 + 10 = 100 (no supera 100)
    let reno_volado2 = RenosContract::volar(env.clone(), reno.clone(), 50); // energia 95
    let reno_descansado2 = RenosContract::descansar(env.clone(), reno_volado2, 10);
    assert_eq!(reno_descansado2.energia, 100);
}

#[test]
fn test_puede_volar() {
    let env = Env::default();

    let reno = RenosContract::crear_reno(env.clone(), Symbol::new(&env, "Rudolph"), 10);

    // energia = 100, distancia 500 → necesario = 500 / 10 = 50 ≤ 100
    let puede = RenosContract::puede_volar(env.clone(), reno.clone(), 500);
    assert!(puede);

    // distancia grande → consumo = 150 / 10 = 15, energia 10 → false
    let reno_bajo = RenosContract::volar(env.clone(), reno.clone(), 950); // consumo 95, energia 5
    let puede_bajo = RenosContract::puede_volar(env.clone(), reno_bajo, 100);
    assert!(!puede_bajo);
}
