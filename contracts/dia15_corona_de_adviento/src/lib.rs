#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VelaCorona {
    pub numero: u32,
    pub encendida: bool,
}

#[contract]
pub struct CoronaContract;

#[contractimpl]
impl CoronaContract {
    pub fn crear_corona(env: Env) -> Vec<VelaCorona> {
        let mut velas = Vec::new(&env);
        for i in 1..=4 {
            let vela = VelaCorona { numero: i, encendida: false };
            velas.push_back(vela);
        }
        velas
    }

    pub fn encender_siguiente(env: Env, velas: Vec<VelaCorona>) -> Vec<VelaCorona> {
        let mut velas_nuevo = Vec::new(&env);
        let mut ya_encendida = false;

        for i in 0..velas.len() {
            let mut vela = velas.get(i).unwrap();
            if !vela.encendida && !ya_encendida {
                vela.encendida = true;
                ya_encendida = true;
            }
            velas_nuevo.push_back(vela);
        }

        velas_nuevo
    }

    pub fn contar_encendidas(_env: Env, velas: Vec<VelaCorona>) -> u32 {
        let mut contador = 0;
        for i in 0..velas.len() {
            if velas.get(i).unwrap().encendida {
                contador += 1;
            }
        }
        contador
    }

    pub fn todas_encendidas(env: Env, velas: Vec<VelaCorona>) -> bool {
        Self::contar_encendidas(env, velas) == 4
    }

    pub fn apagar_todas(env: Env, velas: Vec<VelaCorona>) -> Vec<VelaCorona> {
        let mut velas_nuevo = Vec::new(&env);

        for i in 0..velas.len() {
            let mut vela = velas.get(i).unwrap();
            vela.encendida = false;
            velas_nuevo.push_back(vela);
        }

        velas_nuevo
    }
}
