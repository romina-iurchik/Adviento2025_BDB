#![cfg(test)]

use soroban_sdk::{Env, Map, Address};
use soroban_sdk::testutils::Address as TestAddressTrait; // trait de test

use dia12_nieve_cristalina::{
    NieveContract,
    NieveContractClient,
};

fn setup<'a>() -> (Env, NieveContractClient<'a>, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(NieveContract, ());
    let client = NieveContractClient::new(&env, &contract_id);

    // Llamada fully-qualified al trait
    let alice = <Address as TestAddressTrait>::generate(&env);
    let bob = <Address as TestAddressTrait>::generate(&env);

    (env, client, alice, bob)
}

#[test]
fn test_recolectar() {
    let (env, client, alice, _) = setup();

    let copos = Map::new(&env);

    let copos = client.recolectar(&copos, &alice, &10);

    assert_eq!(copos.get(alice).unwrap(), 10);
}

#[test]
fn test_ver_copos() {
    let (env, client, alice, _) = setup();

    let mut copos = Map::new(&env);
    copos.set(alice.clone(), 15);

    let total = client.ver_copos(&copos, &alice);

    assert_eq!(total, 15);
}

#[test]
fn test_transferir() {
    let (env, client, alice, bob) = setup();

    let mut copos = Map::new(&env);
    copos.set(alice.clone(), 20);

    let copos = client.transferir(&copos, &alice, &bob, &5);

    assert_eq!(copos.get(alice).unwrap(), 15);
    assert_eq!(copos.get(bob).unwrap(), 5);
}

#[test]
#[should_panic(expected = "Saldo insuficiente")]
fn test_transferir_sin_saldo() {
    let (env, client, alice, bob) = setup();

    let mut copos = Map::new(&env);
    copos.set(alice.clone(), 3);

    client.transferir(&copos, &alice, &bob, &10);
}
