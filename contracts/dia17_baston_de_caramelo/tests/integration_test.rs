#[cfg(test)]
mod test {

    use dia17_baston_de_caramelo::{Baston, BastonContract, BastonContractClient};
    use soroban_sdk::{Env, symbol_short};

    #[test]
    fn test_crear_baston() {
        let env = Env::default();
        let contract_id = env.register(BastonContract, ());
        let client = BastonContractClient::new(&env, &contract_id);

        let sabor = symbol_short!("menta");
        let peso = 25u32;

        // Pasar referencias exactas como espera el client
        let baston: Baston = client.crear_baston(&sabor, &peso);

        assert_eq!(baston.sabor, sabor);
        assert_eq!(baston.peso_gramos, peso);
        assert!(baston.envoltorio.is_none());
    }

    #[test]
    fn test_envoltorio_inicial_none() {
        let env = Env::default();
        let contract_id = env.register(BastonContract, ());
        let client = BastonContractClient::new(&env, &contract_id);

        let sabor = symbol_short!("chocolate");
        let peso = 40u32;

        let baston: Baston = client.crear_baston(&sabor, &peso);

        // El envoltorio debe ser None
        assert!(baston.envoltorio.is_none());
    }
}
