#![cfg(test)]

use soroban_sdk::{

    symbol_short,
    Env,
};

use dia18_taller_de_santa::{TallerContract, TallerContractClient};

#[test]
fn test_registrar_y_obtener_juguete() {
    let env = Env::default();
    let contract_id = env.register(TallerContract, ());
    let client = TallerContractClient::new(&env, &contract_id);

    let nombre = symbol_short!("oso");
    let categoria = symbol_short!("peluche");

    client.registrar_juguete(&nombre, &categoria, &10);

    let juguete = client.obtener_juguete(&nombre).unwrap();

    assert_eq!(juguete.nombre, nombre);
    assert_eq!(juguete.categoria, categoria);
    assert_eq!(juguete.cantidad, 10);
}

#[test]
fn test_existe_juguete() {
    let env = Env::default();
    let contract_id = env.register(TallerContract, ());
    let client = TallerContractClient::new(&env, &contract_id);

    let nombre = symbol_short!("auto");

    assert_eq!(client.existe_juguete(&nombre), false);

    client.registrar_juguete(
        &nombre,
        &symbol_short!("vehiculo"),
        &5,
    );

    assert_eq!(client.existe_juguete(&nombre), true);
}

#[test]
fn test_contar_por_categoria() {
    let env = Env::default();
    let contract_id = env.register(TallerContract, ());
    let client = TallerContractClient::new(&env, &contract_id);

    client.registrar_juguete(
        &symbol_short!("oso"),
        &symbol_short!("peluche"),
        &10,
    );

    client.registrar_juguete(
        &symbol_short!("conejo"),
        &symbol_short!("peluche"),
        &7,
    );

    client.registrar_juguete(
        &symbol_short!("auto"),
        &symbol_short!("vehiculo"),
        &3,
    );

    let cantidad = client.contar_por_categoria(&symbol_short!("peluche"));
    assert_eq!(cantidad, 2);
}

#[test]
fn test_actualizar_cantidad() {
    let env = Env::default();
    let contract_id = env.register(TallerContract, ());
    let client = TallerContractClient::new(&env, &contract_id);

    let nombre = symbol_short!("tren");

    client.registrar_juguete(
        &nombre,
        &symbol_short!("vehiculo"),
        &4,
    );

    client.actualizar_cantidad(&nombre, &20);

    let juguete = client.obtener_juguete(&nombre).unwrap();
    assert_eq!(juguete.cantidad, 20);
}
