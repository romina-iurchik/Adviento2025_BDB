#![cfg(test)]

extern crate std;

use soroban_sdk::{Env, Symbol, Map};


use dia08_elfos_trabajadores::ElfosContract;

#[test]
fn test_registrar_y_contar_elfos() {
    let env = Env::default();

    let mut elfos: Map<Symbol, u32> = Map::new(&env);

    // Registrar un elfo nuevo
    elfos = ElfosContract::registrar_elfo(env.clone(), elfos, Symbol::new(&env, "Pepe"));
    assert_eq!(ElfosContract::contar_juguetes(env.clone(), elfos.clone(), Symbol::new(&env, "Pepe")), 0);

    // Verificar existencia
    assert!(ElfosContract::existe_elfo(env.clone(), elfos.clone(), Symbol::new(&env, "Pepe")));
    assert!(!ElfosContract::existe_elfo(env.clone(), elfos.clone(), Symbol::new(&env, "NoExiste")));
}

#[test]
fn test_fabricar_juguetes() {
    let env = Env::default();

    let mut elfos: Map<Symbol, u32> = Map::new(&env);

    // Registrar elfo
    elfos = ElfosContract::registrar_elfo(env.clone(), elfos, Symbol::new(&env, "Pepe"));

    // Fabricar 10 juguetes
    elfos = ElfosContract::fabricar_juguetes(env.clone(), elfos, Symbol::new(&env, "Pepe"), 10);
    assert_eq!(ElfosContract::contar_juguetes(env.clone(), elfos.clone(), Symbol::new(&env, "Pepe")), 10);

    // Fabricar 5 más
    elfos = ElfosContract::fabricar_juguetes(env.clone(), elfos, Symbol::new(&env, "Pepe"), 5);
    assert_eq!(ElfosContract::contar_juguetes(env.clone(), elfos.clone(), Symbol::new(&env, "Pepe")), 15);

    // Contar juguetes de un elfo que no existe
    assert_eq!(ElfosContract::contar_juguetes(env.clone(), elfos.clone(), Symbol::new(&env, "NoExiste")), 0);
}

#[test]
fn test_existe_elfo() {
    let env = Env::default();

    let mut elfos: Map<Symbol, u32> = Map::new(&env);

    // Antes de registrar, no existe
    assert!(!ElfosContract::existe_elfo(env.clone(), elfos.clone(), Symbol::new(&env, "Pepe")));

    // Registrar elfo
    elfos = ElfosContract::registrar_elfo(env.clone(), elfos, Symbol::new(&env, "Pepe"));

    // Ahora existe
    assert!(ElfosContract::existe_elfo(env.clone(), elfos.clone(), Symbol::new(&env, "Pepe")));
}
