// 🎄 Día 2: Árbol Digital

pub fn crear_lista() -> Vec<String> {
    Vec::new()
}

pub fn agregar_adorno(mut lista: Vec<String>, adorno: String) -> Vec<String> {
    lista.push(adorno);
    lista
}

pub fn contar_adornos(lista: &Vec<String>) -> usize {
    lista.len()
}

pub fn primer_adorno(lista: &Vec<String>) -> Option<String> {
    lista.get(0).cloned()
}