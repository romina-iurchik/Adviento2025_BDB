use dia03_cartas_a_santa::*; // cambia al nombre exacto de tu crate Día 3

#[test]
fn test_validar_carta_ok() {
    let resultado = validar_carta("Querido Santa, quiero una bici");
    assert_eq!(resultado, Ok("Querido Santa, quiero una bici".to_string()));
}

#[test]
fn test_validar_carta_muy_corta() {
    let resultado = validar_carta("Hola");
    assert_eq!(resultado, Err(ErrorCarta::MuyCorta));
}

#[test]
fn test_validar_carta_muy_larga() {
    let carta_larga = "A".repeat(101);
    let resultado = validar_carta(&carta_larga);
    assert_eq!(resultado, Err(ErrorCarta::MuyLarga));
}

#[test]
fn test_validar_nombre_ok() {
    let resultado = validar_nombre("Ana");
    assert_eq!(resultado, Ok("Ana".to_string()));
}

#[test]
fn test_validar_nombre_vacio() {
    let resultado = validar_nombre("");
    assert_eq!(resultado, Err(ErrorCarta::SinNombre));
}
