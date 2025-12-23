#![no_std]
use soroban_sdk::{Env};
use dia15_corona_de_adviento::{CoronaContractClient, CoronaContract};

fn setup<'a>() -> (Env, CoronaContractClient<'a>) {
    let env = Env::default();
    let contract_id = env.register(CoronaContract, ()); // registrar sin args de constructor
    let client = CoronaContractClient::new(&env, &contract_id);
    (env, client)
}


#[test]
fn test_crear_corona() {
    let (_env, client) = setup();
    let velas = client.crear_corona();
    assert_eq!(velas.len(), 4);
    for i in 0..4 {
        let vela = velas.get(i).unwrap();
        assert_eq!(vela.numero, i + 1);
        assert!(!vela.encendida);
    }
}

#[test]
fn test_encender_siguiente() {
    let (_env, client) = setup();
    let velas = client.crear_corona();
    let velas = client.encender_siguiente(&velas);
    // Solo la primera vela se enciende
    assert!(velas.get(0).unwrap().encendida);
    for i in 1..4 {
        assert!(!velas.get(i).unwrap().encendida);
    }
}

#[test]
fn test_contar_encendidas() {
    let (_env, client) = setup();
    let mut velas = client.crear_corona();
    velas = client.encender_siguiente(&velas);
    velas = client.encender_siguiente(&velas);
    assert_eq!(client.contar_encendidas(&velas), 2);
}

#[test]
fn test_todas_encendidas() {
    let (_env, client) = setup();
    let mut velas = client.crear_corona();
    for _ in 0..4 {
        velas = client.encender_siguiente(&velas);
    }
    assert!(client.todas_encendidas(&velas));
}

#[test]
fn test_apagar_todas() {
    let (_env, client) = setup();
    let mut velas = client.crear_corona();
    for _ in 0..4 {
        velas = client.encender_siguiente(&velas);
    }
    let velas_apagadas = client.apagar_todas(&velas);
    for i in 0..velas_apagadas.len() {
        assert!(!velas_apagadas.get(i).unwrap().encendida);
    }
}
