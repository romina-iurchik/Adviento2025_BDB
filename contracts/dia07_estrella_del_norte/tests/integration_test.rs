#![cfg(test)]

extern crate std;

use soroban_sdk::Env;


use dia07_estrella_del_norte::{
    EstrellaContract,
    Error,
};

#[test]
fn test_encender() {
    let env = Env::default();

    // Energia suficiente
    let result = EstrellaContract::encender(env.clone(), 15);
    assert_eq!(result, Ok(15));

    // Energia insuficiente
    let result = EstrellaContract::encender(env.clone(), 5);
    assert_eq!(result, Err(Error::EnergiaInsuficiente));
}

#[test]
fn test_puede_brillar() {
    let env = Env::default();

    // Estrella ya encendida
    let result = EstrellaContract::puede_brillar(env.clone(), 15, true);
    assert_eq!(result, Err(Error::YaBrillando));

    // Energia suficiente y no encendida
    let result = EstrellaContract::puede_brillar(env.clone(), 15, false);
    assert_eq!(result, Ok(true));

    // Energia insuficiente
    let result = EstrellaContract::puede_brillar(env.clone(), 5, false);
    assert_eq!(result, Err(Error::EnergiaInsuficiente));
}

#[test]
fn test_cargar_energia() {
    let env = Env::default();

    // Incremento que deja energia suficiente
    let result = EstrellaContract::cargar_energia(env.clone(), 5, 10);
    assert_eq!(result, Ok(15));

    // Incremento que sigue siendo insuficiente
    let result = EstrellaContract::cargar_energia(env.clone(), 3, 5);
    assert_eq!(result, Err(Error::EnergiaInsuficiente));
}
