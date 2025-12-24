#![cfg(test)]

use soroban_sdk::{Env};
use dia21_chimenea_acogedora::{
    ChimeneaContract, ChimeneaContractClient, Error
};

#[test]
fn test_crear_chimenea() {
    let env = Env::default();
    let contract_id = env.register(ChimeneaContract, ());
    let client = ChimeneaContractClient::new(&env, &contract_id);

    let chimenea = client.crear_chimenea();
    assert_eq!(chimenea.encendida, false);
    assert_eq!(chimenea.cantidad_lenos, 0);
    assert_eq!(chimenea.tamano_promedio, 0);
}

#[test]
fn test_agregar_lenos() {
    let env = Env::default();
    let contract_id = env.register(ChimeneaContract, ());
    let client = ChimeneaContractClient::new(&env, &contract_id);

    let chimenea = client.crear_chimenea();
    let chimenea = client.agregar_leno(&chimenea, &10);
    assert_eq!(chimenea.cantidad_lenos, 1);
    assert_eq!(chimenea.tamano_promedio, 10);

    let chimenea = client.agregar_leno(&chimenea, &20);
    assert_eq!(chimenea.cantidad_lenos, 2);
    assert_eq!(chimenea.tamano_promedio, 15); // (10+20)/2
}

#[test]
fn test_encender() {
    let env = Env::default();
    let contract_id = env.register(ChimeneaContract, ());
    let client = ChimeneaContractClient::new(&env, &contract_id);

    let chimenea = client.crear_chimenea();
    let chimenea = client.agregar_leno(&chimenea, &10);
    let chimenea = client.encender(&chimenea);
    assert_eq!(chimenea.encendida, true);
}

#[test]
fn test_encender_sin_lenos() {
    let env = Env::default();
    let _contract_id = env.register(ChimeneaContract, ());
    
    // Llamamos directo al contrato para capturar el Result
    let chimenea = ChimeneaContract::crear_chimenea(env.clone());
    let result = ChimeneaContract::encender(env.clone(), chimenea);
    assert_eq!(result, Err(Error::NoHayLenos));
}

#[test]
fn test_calcular_calor() {
    let env = Env::default();
    let contract_id = env.register(ChimeneaContract, ());
    let client = ChimeneaContractClient::new(&env, &contract_id);

    let chimenea = client.crear_chimenea();
    let chimenea = client.agregar_leno(&chimenea, &10);
    let chimenea = client.agregar_leno(&chimenea, &10);
    let chimenea = client.encender(&chimenea);

    let calor = client.calcular_calor(&chimenea);
    assert_eq!(calor, 2 * 10 * 10); // 200
}

#[test]
fn test_quemar_leno() {
    let env = Env::default();
    let contract_id = env.register(ChimeneaContract, ());
    let client = ChimeneaContractClient::new(&env, &contract_id);

    let chimenea = client.crear_chimenea();
    let chimenea = client.agregar_leno(&chimenea, &10);
    let chimenea = client.agregar_leno(&chimenea, &20);
    let chimenea = client.encender(&chimenea);

    let chimenea = client.quemar_leno(&chimenea);
    assert_eq!(chimenea.cantidad_lenos, 1);
    assert_eq!(chimenea.encendida, true);

    let chimenea = client.quemar_leno(&chimenea);
    assert_eq!(chimenea.cantidad_lenos, 0);
    assert_eq!(chimenea.encendida, false);
}

#[test]
fn test_quemar_sin_lenos() {
    let env = Env::default();
    let _contract_id = env.register(ChimeneaContract, ());
    
    let chimenea = ChimeneaContract::crear_chimenea(env.clone());
    let result = ChimeneaContract::quemar_leno(env.clone(), chimenea);
    assert_eq!(result, Err(Error::NoHayLenosParaQuemar));
}

#[test]
fn test_ciclo_completo() {
    let env = Env::default();
    let contract_id = env.register(ChimeneaContract, ());
    let client = ChimeneaContractClient::new(&env, &contract_id);

    let mut chimenea = client.crear_chimenea();
    chimenea = client.agregar_leno(&chimenea, &10);
    chimenea = client.agregar_leno(&chimenea, &20);
    chimenea = client.agregar_leno(&chimenea, &30);

    chimenea = client.encender(&chimenea);
    assert_eq!(chimenea.encendida, true);
    assert_eq!(chimenea.tamano_promedio, 20);
    assert_eq!(chimenea.cantidad_lenos, 3);

    let calor = client.calcular_calor(&chimenea);
    assert_eq!(calor, 3 * 20 * 10); // 600

    chimenea = client.quemar_leno(&chimenea);
    assert_eq!(chimenea.cantidad_lenos, 2);
    assert_eq!(chimenea.encendida, true);

    chimenea = client.quemar_leno(&chimenea);
    chimenea = client.quemar_leno(&chimenea);
    assert_eq!(chimenea.cantidad_lenos, 0);
    assert_eq!(chimenea.encendida, false);
}
