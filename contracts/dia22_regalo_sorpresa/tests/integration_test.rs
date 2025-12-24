#![cfg(test)]

use soroban_sdk::{symbol_short, Env};

use dia22_regalo_sorpresa::{RegaloContract, RegaloContractClient};

#[test]
fn test_crear_caja() {
    let env = Env::default();
    let contract_id = env.register(RegaloContract, ());
    let client = RegaloContractClient::new(&env, &contract_id);

    let caja = client.crear_caja(&symbol_short!("roja"), &Some(symbol_short!("muneco")));
    assert_eq!(caja.color, symbol_short!("roja"));
    assert_eq!(caja.contenido, Some(symbol_short!("muneco")));
    assert!(!caja.abierta);
}

#[test]
fn test_abrir_caja() {
    let env = Env::default();
    let contract_id = env.register(RegaloContract, ());
    let client = RegaloContractClient::new(&env, &contract_id);

    let caja = client.crear_caja(&symbol_short!("verde"), &Some(symbol_short!("chocolate")));
    let caja_abierta = client.abrir_caja(&caja);

    assert!(caja_abierta.abierta);
    assert_eq!(caja_abierta.contenido, Some(symbol_short!("chocolate")));
}

#[test]
fn test_obtener_contenido() {
    let env = Env::default();
    let contract_id = env.register(RegaloContract, ());
    let client = RegaloContractClient::new(&env, &contract_id);

    let caja = client.crear_caja(&symbol_short!("amarilla"), &Some(symbol_short!("caramelos")));

    // Antes de abrir
    assert_eq!(client.obtener_contenido(&caja), None);

    // Abrimos y verificamos
    let caja_abierta = client.abrir_caja(&caja);
    assert_eq!(client.obtener_contenido(&caja_abierta), Some(symbol_short!("caramelos")));
}

#[test]
fn test_guardar_y_contar() {
    let env = Env::default();
    let contract_id = env.register(RegaloContract, ());
    let client = RegaloContractClient::new(&env, &contract_id);

    let caja1 = client.crear_caja(&symbol_short!("roja"), &Some(symbol_short!("muneco")));
    let caja2 = client.crear_caja(&symbol_short!("verde"), &Some(symbol_short!("chocolate")));
    let caja3 = client.crear_caja(&symbol_short!("azul"), &Some(symbol_short!("golosinas")));

    client.guardar_caja(&caja1);
    client.guardar_caja(&caja2);
    client.guardar_caja(&caja3);

    let contador_inicial = client.contar_cajas_cerradas();
    assert_eq!(contador_inicial, 3);
}
