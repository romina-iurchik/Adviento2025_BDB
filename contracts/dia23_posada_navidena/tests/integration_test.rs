#![cfg(test)]
// Importamos el cliente generado automáticamente por la macro #[contract]
use dia23_posada_navidena::{PosadaContract, PosadaContractClient};
use soroban_sdk::Env;

#[test]
fn test_gestion_posada_completa() {
    let env = Env::default();
    // Registramos el contrato y obtenemos su ID
    let contract_id = env.register(PosadaContract, ());
    let client = PosadaContractClient::new(&env, &contract_id);

    // 1. Crear y guardar una habitación
    let habitacion_nueva = client.crear_habitacion(&1, &5);
    client.guardar_habitacion(&habitacion_nueva);

    // 2. Verificar disponibilidad inicial
    assert_eq!(client.esta_disponible(&1), true);

    // 3. Realizar una reserva parcial
    client.reservar(&1, &3);
    
    // Verificamos ocupación total (debería ser 3)
    assert_eq!(client.ocupacion_total(), 3);

    // 4. Intentar reservar más de la capacidad disponible
    let resultado_excedido = client.try_reservar(&1, &3);
    assert_eq!(resultado_excedido.is_err(), true);

    // 5. Liberar espacio
    client.liberar(&1, &1); 
    assert_eq!(client.ocupacion_total(), 2);

    // 6. Verificar que ahora hay espacio de nuevo
    assert_eq!(client.esta_disponible(&1), true);

    // 7. Llenar la habitación por completo
    client.reservar(&1, &3); 
    assert_eq!(client.esta_disponible(&1), false);
    assert_eq!(client.ocupacion_total(), 5);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")] // Error::HabitacionNoExiste
fn test_habitacion_inexistente() {
    let env = Env::default();
    let contract_id = env.register(PosadaContract, ());
    
    // CORRECCIÓN: Aquí debes usar PosadaContractClient::new
    let client = PosadaContractClient::new(&env, &contract_id);

    // Intentar reservar en una habitación que no ha sido creada
    client.reservar(&99, &1);
}