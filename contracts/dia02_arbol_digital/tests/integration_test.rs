use dia02_arbol_digital::*; // cambia el nombre al crate que corresponda

#[test]
fn test_crear_lista_vacia() {
    let lista = crear_lista();
    assert!(lista.is_empty());
}

#[test]
fn test_agregar_adorno() {
    let lista = crear_lista();
    let lista = agregar_adorno(lista, "Estrella".to_string());
    assert_eq!(lista.len(), 1);
    assert_eq!(lista[0], "Estrella");
}

#[test]
fn test_contar_adornos() {
    let mut lista = crear_lista();
    lista = agregar_adorno(lista, "Bola".to_string());
    lista = agregar_adorno(lista, "Guirnalda".to_string());
    assert_eq!(contar_adornos(&lista), 2);
}

#[test]
fn test_primer_adorno_some() {
    let mut lista = crear_lista();
    lista = agregar_adorno(lista, "Estrella".to_string());
    assert_eq!(primer_adorno(&lista), Some("Estrella".to_string()));
}

#[test]
fn test_primer_adorno_none() {
    let lista = crear_lista();
    assert_eq!(primer_adorno(&lista), None);
}
