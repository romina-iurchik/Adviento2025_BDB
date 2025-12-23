#![no_std]
use soroban_sdk::{Env};
use dia16_muneco_de_nieve::{MunecoContract, MunecoContractClient, Muneco};

fn setup<'a>() -> (Env, MunecoContractClient<'a>) {
    let env = Env::default();
    // Registrar el contrato sin constructor
    let contract_id = env.register(MunecoContract, ());
    let client = MunecoContractClient::new(&env, &contract_id);
    (env, client)
}

#[test]
fn test_crear_muneco() {
    let (_env, client) = setup();
    let muneco: Muneco = client.crear_muneco(&50, &35, &20);
    assert_eq!(muneco.bola_inferior, 50);
    assert_eq!(muneco.bola_media, 35);
    assert_eq!(muneco.bola_superior, 20);
}

#[test]
fn test_altura_total() {
    let (_env, client) = setup();
    let muneco: Muneco = client.crear_muneco(&50, &35, &20);
    let altura = client.altura_total(&muneco);
    assert_eq!(altura, (50 + 35 + 20) * 2);
}

#[test]
fn test_esta_completo() {
    let (_env, client) = setup();
    let completo: Muneco = client.crear_muneco(&50, &35, &20);
    let incompleto: Muneco = client.crear_muneco(&50, &0, &20);
    assert!(client.esta_completo(&completo));
    assert!(!client.esta_completo(&incompleto));
}

#[test]
fn test_derretir() {
    let (_env, client) = setup();
    let muneco: Muneco = client.crear_muneco(&50, &35, &20);
    let derretido = client.derretir(&muneco, &10);
    assert_eq!(derretido.bola_inferior, 40);
    assert_eq!(derretido.bola_media, 25);
    assert_eq!(derretido.bola_superior, 10);

    // Si derretimos más de lo que hay, no debe ser negativo
    let casi_nada = client.derretir(&muneco, &100);
    assert_eq!(casi_nada.bola_inferior, 0);
    assert_eq!(casi_nada.bola_media, 0);
    assert_eq!(casi_nada.bola_superior, 0);
}

#[test]
fn test_agregar_nieve() {
    let (_env, client) = setup();
    let muneco: Muneco = client.crear_muneco(&50, &35, &20);
    let nuevo1 = client.agregar_nieve(&muneco, &1, &10);
    assert_eq!(nuevo1.bola_inferior, 60);
    let nuevo2 = client.agregar_nieve(&nuevo1, &2, &5);
    assert_eq!(nuevo2.bola_media, 40);
    let nuevo3 = client.agregar_nieve(&nuevo2, &3, &2);
    assert_eq!(nuevo3.bola_superior, 22);
}
