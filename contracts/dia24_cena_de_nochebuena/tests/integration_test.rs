#[cfg(test)]
mod test {
    use dia24_cena_de_nochebuena::{CenaContract, CenaContractClient};
    use soroban_sdk::{symbol_short, Env};

#[test]
    fn test_agregar_platillo() {
        let env = Env::default();
        let contract_id = env.register(CenaContract, ());
        let client = CenaContractClient::new(&env, &contract_id);

        client.agregar_platillo(&symbol_short!("pavo"), &8);
        // Si este pasa, es que el storage está funcionando
        assert_eq!(client.porciones_totales(), 8);
    }

    #[test]
    fn test_marcar_listo() {
        let env = Env::default();
        let contract_id = env.register(CenaContract, ());
        let client = CenaContractClient::new(&env, &contract_id);

        let nombre = symbol_short!("pavo");
        client.agregar_platillo(&nombre, &8);
        
        // Verificamos el cambio de estado
        client.marcar_listo(&nombre);
        
        // Si esto falla, el error está en la creación del menu_nuevo
        assert_eq!(client.todos_listos(), true);
    }

    #[test]
    fn test_todos_listos() {
        let env = Env::default();
        let contract_id = env.register(CenaContract, ());
        let client = CenaContractClient::new(&env, &contract_id);

        // Caso 1: Vacío debe ser false
        assert_eq!(client.todos_listos(), false);

        client.agregar_platillo(&symbol_short!("pavo"), &8);
        client.agregar_platillo(&symbol_short!("sopa"), &4);

        // Caso 2: Incompletos debe ser false
        client.marcar_listo(&symbol_short!("pavo"));
        assert_eq!(client.todos_listos(), false);

        // Caso 3: Todos listos debe ser true
        client.marcar_listo(&symbol_short!("sopa"));
        assert_eq!(client.todos_listos(), true);
    }

    #[test]
    fn test_porciones_totales() {
        let env = Env::default();
        let contract_id = env.register(CenaContract, ());
        let client = CenaContractClient::new(&env, &contract_id);

        client.agregar_platillo(&symbol_short!("A"), &10);
        client.agregar_platillo(&symbol_short!("B"), &5);
        client.agregar_platillo(&symbol_short!("C"), &5);

        // Si esto no da 20, hay un problema en el acumulador 'total'
        assert_eq!(client.porciones_totales(), 20);
    }

    #[test]
    fn test_servir_cena_exitosa() {
        let env = Env::default();
        let contract_id = env.register(CenaContract, ());
        let client = CenaContractClient::new(&env, &contract_id);

        client.agregar_platillo(&symbol_short!("pavo"), &8);
        
        // No debe dejar servir si no está listo (devuelve error)
        let resultado_error = client.try_servir_cena();
        assert!(resultado_error.is_err());

        client.marcar_listo(&symbol_short!("pavo"));
        
        // Ahora debe ser Ok
        let resultado_ok = client.try_servir_cena();
        assert!(resultado_ok.is_ok());
    }
}