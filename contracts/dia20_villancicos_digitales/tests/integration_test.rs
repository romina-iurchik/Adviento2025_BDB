#![cfg(test)]

use soroban_sdk::{
    symbol_short,
    Env,
};

use dia20_villancicos_digitales::{
    VillancicoContract,
    VillancicoContractClient,
};

#[test]
fn test_agregar_y_obtener() {
    let env = Env::default();
    let contract_id = env.register(VillancicoContract, ());
    let client = VillancicoContractClient::new(&env, &contract_id);

    client.agregar_villancico(
        &symbol_short!("JingleB"),
        &symbol_short!("Clasico"),
        &180,
    );

    let playlist = client.obtener_playlist();
    assert_eq!(playlist.len(), 1);

    let v = playlist.get(0).unwrap();
    assert_eq!(v.titulo, symbol_short!("JingleB"));
    assert_eq!(v.artista, symbol_short!("Clasico"));
    assert_eq!(v.duracion_segundos, 180);
}

#[test]
fn test_duracion_total() {
    let env = Env::default();
    let contract_id = env.register(VillancicoContract, ());
    let client = VillancicoContractClient::new(&env, &contract_id);

    client.agregar_villancico(
        &symbol_short!("JingleB"),
        &symbol_short!("Clasico"),
        &180,
    );
    client.agregar_villancico(
        &symbol_short!("NocheP"),
        &symbol_short!("Moderno"),
        &240,
    );

    let total = client.duracion_total();
    assert_eq!(total, 420);
}

#[test]
fn test_buscar_por_artista() {
    let env = Env::default();
    let contract_id = env.register(VillancicoContract, ());
    let client = VillancicoContractClient::new(&env, &contract_id);

    client.agregar_villancico(
        &symbol_short!("JingleB"),
        &symbol_short!("Clasico"),
        &180,
    );
    client.agregar_villancico(
        &symbol_short!("SilentN"),
        &symbol_short!("Clasico"),
        &210,
    );
    client.agregar_villancico(
        &symbol_short!("NavidadP"),
        &symbol_short!("Moderno"),
        &200,
    );

    let clasicos = client.buscar_por_artista(&symbol_short!("Clasico"));
    assert_eq!(clasicos.len(), 2);

    for i in 0..clasicos.len() {
        let v = clasicos.get(i).unwrap();
        assert_eq!(v.artista, symbol_short!("Clasico"));
    }
}

#[test]
fn test_villancico_mas_largo() {
    let env = Env::default();
    let contract_id = env.register(VillancicoContract, ());
    let client = VillancicoContractClient::new(&env, &contract_id);

    assert!(client.villancico_mas_largo().is_none());

    client.agregar_villancico(
        &symbol_short!("Corto"),
        &symbol_short!("Clasico"),
        &120,
    );
    client.agregar_villancico(
        &symbol_short!("Largo"),
        &symbol_short!("Clasico"),
        &300,
    );
    client.agregar_villancico(
        &symbol_short!("Medio"),
        &symbol_short!("Moderno"),
        &200,
    );

    let mas_largo = client.villancico_mas_largo().unwrap();
    assert_eq!(mas_largo.titulo, symbol_short!("Largo"));
    assert_eq!(mas_largo.duracion_segundos, 300);
}

#[test]
fn test_playlist_completa() {
    let env = Env::default();
    let contract_id = env.register(VillancicoContract, ());
    let client = VillancicoContractClient::new(&env, &contract_id);

    client.agregar_villancico(
        &symbol_short!("A"),
        &symbol_short!("Clasico"),
        &100,
    );
    client.agregar_villancico(
        &symbol_short!("B"),
        &symbol_short!("Clasico"),
        &150,
    );
    client.agregar_villancico(
        &symbol_short!("C"),
        &symbol_short!("Moderno"),
        &200,
    );

    let playlist = client.obtener_playlist();
    let total = client.duracion_total();
    let clasicos = client.buscar_por_artista(&symbol_short!("Clasico"));
    let mas_largo = client.villancico_mas_largo().unwrap();

    assert_eq!(playlist.len(), 3);
    assert_eq!(total, 450);
    assert_eq!(clasicos.len(), 2);
    assert_eq!(mas_largo.titulo, symbol_short!("C"));
}
