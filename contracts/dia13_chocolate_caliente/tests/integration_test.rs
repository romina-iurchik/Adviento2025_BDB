#![cfg(test)]

use soroban_sdk::{Env, symbol_short};
use dia13_chocolate_caliente::{ChocolateContractClient, Taza, ChocolateContract};

fn setup<'a>() -> (Env, ChocolateContractClient<'a>) {
    let env = Env::default();
    let contract_id = env.register(ChocolateContract, ());
    let client = ChocolateContractClient::new(&env, &contract_id);
    (env, client)
}

#[test]
fn test_preparar_taza() {
    let (_env, client) = setup();
    let taza = client.preparar_taza(&10, &200, &5);
    assert_eq!(taza.cacao, 10);
    assert_eq!(taza.leche, 200);
    assert_eq!(taza.azucar, 5);
    assert_eq!(taza.temperatura, 25);
}

#[test]
fn test_calentar() {
    let (_env, client) = setup();
    let taza = Taza { cacao: 10, leche: 200, azucar: 5, temperatura: 25 };
    let taza = client.calentar(&taza, &45);
    assert_eq!(taza.temperatura, 70);
}

#[test]
fn test_enfriar() {
    let (_env, client) = setup();
    let taza = Taza { cacao: 10, leche: 200, azucar: 5, temperatura: 50 };
    let taza = client.enfriar(&taza, &20);
    assert_eq!(taza.temperatura, 30);

    // probar enfriar más de la temperatura
    let taza = Taza { cacao: 10, leche: 200, azucar: 5, temperatura: 15 };
    let taza = client.enfriar(&taza, &30);
    assert_eq!(taza.temperatura, 0);
}

#[test]
fn test_esta_perfecta() {
    let (_env, client) = setup();
    let taza = Taza { cacao: 10, leche: 200, azucar: 5, temperatura: 70 };
    assert!(client.esta_perfecta(&taza));

    let taza = Taza { cacao: 10, leche: 200, azucar: 5, temperatura: 50 };
    assert!(!client.esta_perfecta(&taza));

    let taza = Taza { cacao: 10, leche: 200, azucar: 5, temperatura: 85 };
    assert!(!client.esta_perfecta(&taza));
}

#[test]
fn test_agregar_ingrediente() {
    let (_env, client) = setup();
    let taza = Taza { cacao: 10, leche: 200, azucar: 5, temperatura: 25 };

    let taza = client.agregar_ingrediente(&taza, &symbol_short!("cacao"), &5);
    assert_eq!(taza.cacao, 15);

    let taza = client.agregar_ingrediente(&taza, &symbol_short!("leche"), &50);
    assert_eq!(taza.leche, 250);

    let taza = client.agregar_ingrediente(&taza, &symbol_short!("azucar"), &10);
    assert_eq!(taza.azucar, 15);
}
