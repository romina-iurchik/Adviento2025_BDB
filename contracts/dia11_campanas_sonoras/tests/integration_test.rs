#![cfg(test)]

extern crate std;

use soroban_sdk::{Env, Vec, testutils::Ledger};


use dia11_campanas_sonoras::CampanasContract;

#[test]
fn test_sonar_agrega_timestamp() {
    let env = Env::default();

    // Forzamos un timestamp conocido
    env.ledger().set_timestamp(1000);

    let campanadas = Vec::new(&env);
    let resultado = CampanasContract::sonar(env.clone(), campanadas);

    assert_eq!(resultado.len(), 1);
    assert_eq!(resultado.get(0).unwrap(), 1000);
}

#[test]
fn test_contar_campanadas() {
    let env = Env::default();

    let mut campanadas = Vec::new(&env);
    campanadas.push_back(1000);
    campanadas.push_back(2000);
    campanadas.push_back(3000);

    let cantidad = CampanasContract::contar_campanadas(env, campanadas);
    assert_eq!(cantidad, 3);
}

#[test]
fn test_tiempo_transcurrido() {
    let env = Env::default();

    let mut campanadas = Vec::new(&env);
    campanadas.push_back(1000);
    campanadas.push_back(2000);
    campanadas.push_back(3000);

    let tiempo = CampanasContract::tiempo_transcurrido(env, campanadas);
    assert_eq!(tiempo, 2000); // 3000 - 1000
}

#[test]
fn test_tiempo_transcurrido_con_menos_de_dos() {
    let env = Env::default();

    let mut campanadas = Vec::new(&env);
    campanadas.push_back(1000);

    let tiempo = CampanasContract::tiempo_transcurrido(env, campanadas);
    assert_eq!(tiempo, 0);
}

#[test]
fn test_ultima_campanada() {
    let env = Env::default();

    let mut campanadas = Vec::new(&env);
    campanadas.push_back(1000);
    campanadas.push_back(2000);
    campanadas.push_back(3000);

    let ultima = CampanasContract::ultima_campanada(env, campanadas);
    assert_eq!(ultima, 3000);
}

#[test]
fn test_ultima_campanada_vec_vacio() {
    let env = Env::default();

    let campanadas = Vec::new(&env);
    let ultima = CampanasContract::ultima_campanada(env, campanadas);

    assert_eq!(ultima, 0);
}
