use soroban_sdk::{Env};
use dia14_velas_brillantes::{VelasContractClient, VelasContract, Color};

fn setup() -> (Env, VelasContractClient<'static>) {
    let env = Env::default();
    // registrar usando el método actualizado `register`
    let contract_id = env.register(VelasContract, ());
    let client = VelasContractClient::new(&env, &contract_id);
    (env, client)
}


#[test]
fn test_crear_vela() {
    let (_env, client) = setup();
    let vela = client.crear_vela(&Color::Rojo, &20);
    assert_eq!(vela.color, Color::Rojo);
    assert_eq!(vela.altura_inicial, 20);
    assert_eq!(vela.consumida, 0);
}

#[test]
fn test_encender() {
    let (_env, client) = setup();
    let vela = client.crear_vela(&Color::Verde, &15);
    let vela = client.encender(&vela, &5);
    assert_eq!(vela.consumida, 5);
    let altura_actual = client.altura_actual(&vela);
    assert_eq!(altura_actual, 10);
}

#[test]
fn test_encender_limite() {
    let (_env, client) = setup();
    let vela = client.crear_vela(&Color::Dorado, &10);
    let vela = client.encender(&vela, &15); // más que altura_inicial
    assert_eq!(vela.consumida, 10); // no debe superar altura_inicial
    let altura_actual = client.altura_actual(&vela);
    assert_eq!(altura_actual, 0);
}

#[test]
fn test_esta_apagada() {
    let (_env, client) = setup();
    let vela = client.crear_vela(&Color::Blanco, &8);
    let vela = client.encender(&vela, &8);
    assert!(client.esta_apagada(&vela));
    let vela2 = client.crear_vela(&Color::Rojo, &12);
    let vela2 = client.encender(&vela2, &5);
    assert!(!client.esta_apagada(&vela2));
}

#[test]
fn test_cambiar_color() {
    let (_env, client) = setup();
    let vela = client.crear_vela(&Color::Verde, &10);
    let vela = client.cambiar_color(&vela, &Color::Rojo);
    assert_eq!(vela.color, Color::Rojo);
}
