// 🎄 Día 3: Cartas a Santa

#[derive(Debug, PartialEq)]
pub enum ErrorCarta {
    MuyCorta,
    MuyLarga,
    SinNombre,
}

pub fn validar_carta(carta: &str) -> Result<String, ErrorCarta> {

    // TODO: Si carta.len() < 5 -> Err(ErrorCarta::MuyCorta)
    if carta.len() < 5 { return Err(ErrorCarta::MuyCorta); }

    // TODO: Si carta.len() > 100 -> Err(ErrorCarta::MuyLarga)
    if carta.len() > 100 { return Err(ErrorCarta::MuyLarga); }

    // TODO: Si no -> Ok(carta.to_string())
    Ok(carta.to_string())
}

pub fn validar_nombre(nombre: &str) -> Result<String, ErrorCarta> {

    // TODO: Si nombre.is_empty() -> Err(ErrorCarta::SinNombre)
    if nombre.is_empty() { return  Err(ErrorCarta::SinNombre); }

    // TODO: Si no -> Ok(nombre.to_string())
    Ok(nombre.to_string())
}