#![cfg(test)]

use soroban_sdk::{
    symbol_short,
    Env,
};

use dia19_guirnaldas_festivas::{
    GuirnaldasContract,
    GuirnaldasContractClient,
};

#[test]
fn test_crear_guirnalda() {
    let env = Env::default();
    let contract_id = env.register(GuirnaldasContract, ());
    let client = GuirnaldasContractClient::new(&env, &contract_id);

    let luces = client.crear_guirnalda(&5, &symbol_short!("rojo"));

    assert_eq!(luces.len(), 5);

    for i in 0..luces.len() {
        let luz = luces.get(i).unwrap();
        assert_eq!(luz.posicion, i);
        assert_eq!(luz.color, symbol_short!("rojo"));
        assert_eq!(luz.encendida, false);
    }
}

#[test]
fn test_encender_luz() {
    let env = Env::default();
    let contract_id = env.register(GuirnaldasContract, ());
    let client = GuirnaldasContractClient::new(&env, &contract_id);

    let luces = client.crear_guirnalda(&3, &symbol_short!("verde"));
    let luces = client.encender_luz(&luces, &1);

    for i in 0..luces.len() {
        let luz = luces.get(i).unwrap();
        if i == 1 {
            assert!(luz.encendida);
        } else {
            assert!(!luz.encendida);
        }
    }
}

#[test]
fn test_alternar_luz() {
    let env = Env::default();
    let contract_id = env.register(GuirnaldasContract, ());
    let client = GuirnaldasContractClient::new(&env, &contract_id);

    let luces = client.crear_guirnalda(&2, &symbol_short!("azul"));

    let luces = client.alternar_luz(&luces, &0);
    assert!(luces.get(0).unwrap().encendida);

    let luces = client.alternar_luz(&luces, &0);
    assert!(!luces.get(0).unwrap().encendida);
}

#[test]
fn test_encender_alternadas_pares() {
    let env = Env::default();
    let contract_id = env.register(GuirnaldasContract, ());
    let client = GuirnaldasContractClient::new(&env, &contract_id);

    let luces = client.crear_guirnalda(&5, &symbol_short!("blanco"));
    let luces = client.encender_alternadas(&luces, &true);

    for i in 0..luces.len() {
        let luz = luces.get(i).unwrap();
        if i % 2 == 0 {
            assert!(luz.encendida);
        } else {
            assert!(!luz.encendida);
        }
    }
}

#[test]
fn test_encender_alternadas_impares() {
    let env = Env::default();
    let contract_id = env.register(GuirnaldasContract, ());
    let client = GuirnaldasContractClient::new(&env, &contract_id);

    let luces = client.crear_guirnalda(&5, &symbol_short!("blanco"));
    let luces = client.encender_alternadas(&luces, &false);

    for i in 0..luces.len() {
        let luz = luces.get(i).unwrap();
        if i % 2 == 1 {
            assert!(luz.encendida);
        } else {
            assert!(!luz.encendida);
        }
    }
}

#[test]
fn test_patron_complejo() {
    let env = Env::default();
    let contract_id = env.register(GuirnaldasContract, ());
    let client = GuirnaldasContractClient::new(&env, &contract_id);

    // Crear guirnalda
    let luces = client.crear_guirnalda(&6, &symbol_short!("rojo"));

    // Encender pares
    let luces = client.encender_alternadas(&luces, &true);

    // Alternar una luz par (la apaga)
    let luces = client.alternar_luz(&luces, &2);

    // Encender una impar manualmente
    let luces = client.encender_luz(&luces, &3);

    let encendidas = client.contar_encendidas(&luces);
    assert_eq!(encendidas, 3);
}
