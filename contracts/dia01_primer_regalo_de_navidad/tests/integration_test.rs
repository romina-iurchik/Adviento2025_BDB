use dia01_primer_regalo_de_navidad::*;

#[test]
fn test_contar_regalos() {
    assert_eq!(contar_regalos(), 100);
}

#[test]
fn test_sumar_regalos() {
    assert_eq!(sumar_regalos(10, 5), 15);
}

#[test]
fn test_hay_suficientes() {
    assert!(hay_suficientes(50));
    assert!(!hay_suficientes(20));
}
