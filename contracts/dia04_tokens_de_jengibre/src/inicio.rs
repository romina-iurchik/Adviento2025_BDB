// 🎄 Día 4: Tokens de Jengibre

#[derive(Debug, Clone, PartialEq)]
pub struct Galleta {
    pub nombre: String,
    pub peso_gramos: u32,
}

impl Galleta {
    /// Crea una nueva galleta
    pub fn nueva(nombre: String, peso_gramos: u32) -> Self {
        // TODO: Retorná Galleta { nombre, peso_gramos }
        Galleta {
            nombre: String::new(),
            peso_gramos: 0,
        }
    }
    
    /// Verifica si la galleta es grande (más de 50g)
    pub fn es_grande(&self) -> bool {
        // TODO: Retorná self.peso_gramos > 50
        false
    }
    
    /// Describe la galleta
    pub fn describir(&self) -> String {
        // TODO: Retorná un String con formato:
        // "Galleta {nombre} de {peso}g"
        String::new()
    }
}

/// Calcula el peso total de varias galletas
pub fn peso_total(galletas: &Vec<Galleta>) -> u32 {
    // TODO: Sumá el peso de todas las galletas
    0
}