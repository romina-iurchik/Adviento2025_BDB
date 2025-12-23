#![cfg(test)]

extern crate std;

use soroban_sdk::Env;


use dia06_lucces_navidenas::{
    LucesContract,
    LucesContractClient,
};

#[test]
fn test_inicializar_empieza_en_cero() {
    let env = Env::default();
    let contract_id = env.register(LucesContract, ());

    let client = LucesContractClient::new(&env, &contract_id);

    client.inicializar();

    assert_eq!(client.contar_luces(), 0);
}

#[test]
fn test_encender_luz_incrementa_correctamente() {
    let env = Env::default();
    let contract_id = env.register(LucesContract, ());

    let client = LucesContractClient::new(&env, &contract_id);

    client.inicializar();

    assert_eq!(client.encender_luz(), 1);
    assert_eq!(client.encender_luz(), 2);
    assert_eq!(client.encender_luz(), 3);
}

#[test]
fn test_persistencia_entre_llamadas() {
    let env = Env::default();
    let contract_id =env.register(LucesContract, ());

    let client = LucesContractClient::new(&env, &contract_id);

    client.inicializar();
    client.encender_luz();
    client.encender_luz();

    assert_eq!(client.contar_luces(), 2);
}

#[test]
fn test_apagar_todo_resetea_a_cero() {
    let env = Env::default();
    let contract_id = env.register(LucesContract, ());

    let client = LucesContractClient::new(&env, &contract_id);

    client.inicializar();
    client.encender_luz();
    client.encender_luz();

    client.apagar_todo();

    assert_eq!(client.contar_luces(), 0);
}


