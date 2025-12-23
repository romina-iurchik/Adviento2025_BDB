use dia05_trineo_automatizado::TrineoContract;
use soroban_sdk::{Env, Vec, symbol_short};

#[test]
fn test_agregar_regalo() {
    let env = Env::default();
    let regalos: Vec<_> = Vec::new(&env);
    let regalo = symbol_short!("bici");
    let regalos = TrineoContract::agregar_regalo(env.clone(), regalos, regalo.clone());
    assert_eq!(regalos.len(), 1);
    assert_eq!(regalos.get(0).unwrap(), regalo);
}

#[test]
fn test_contar_regalos() {
    let env = Env::default();
    let mut regalos: Vec<_> = Vec::new(&env);
    regalos.push_back(symbol_short!("bici"));
    regalos.push_back(symbol_short!("muneco"));

    let cantidad = TrineoContract::contar_regalos(env.clone(), regalos);
    assert_eq!(cantidad, 2);
}

#[test]
fn test_puede_despegar() {
    let env = Env::default();
    let mut regalos: Vec<_> = Vec::new(&env);
    regalos.push_back(symbol_short!("bici"));
    regalos.push_back(symbol_short!("muneco"));

    assert_eq!(TrineoContract::puede_despegar(env.clone(), regalos.clone()), false);

    regalos.push_back(symbol_short!("tren"));
    assert_eq!(TrineoContract::puede_despegar(env.clone(), regalos), true);
}

