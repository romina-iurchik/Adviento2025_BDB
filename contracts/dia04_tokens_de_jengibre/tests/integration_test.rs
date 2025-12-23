use dia04_tokens_de_jengibre::*; // Cambiar al nombre exacto de tu crate Día 4

#[test]
fn test_galleta_nueva() {
    let g = Galleta::nueva("Estrella".to_string(), 30);
    assert_eq!(g.nombre, "Estrella");
    assert_eq!(g.peso_gramos, 30);
}

#[test]
fn test_es_grande_true() {
    let g = Galleta::nueva("Galletón".to_string(), 80);
    assert!(g.es_grande());
}

#[test]
fn test_es_grande_false() {
    let g = Galleta::nueva("Mini".to_string(), 20);
    assert!(!g.es_grande());
}

#[test]
fn test_describir() {
    let g = Galleta::nueva("Árbol".to_string(), 45);
    let descripcion = g.describir();
    assert_eq!(descripcion, "Galleta Árbol de 45g");
}

#[test]
fn test_peso_total() {
    let g1 = Galleta::nueva("Estrella".to_string(), 30);
    let g2 = Galleta::nueva("Árbol".to_string(), 45);
    let g3 = Galleta::nueva("Galletón".to_string(), 80);
    let galletas = vec![g1, g2, g3];
    let total = peso_total(&galletas);
    assert_eq!(total, 155);
}
