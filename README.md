# 🎄 Adviento Stellar 2025

Bienvenido al **Calendario de Adviento Stellar 2025**, 25 días de retos de código con **Stellar + Soroban**, diseñados para aprender, practicar y divertirse programando contratos en Rust.
 
Este repositorio complementa la experiencia online del calendario en 👉 https://adviento2025.buendiabuilders.com/

---

## 💡 ¿Qué es Adviento Stellar 2025?

Adviento Stellar 2025 es una serie de **25 desafíos diarios** en torno a conceptos de programación y blockchain, con un enfoque especial en:

- **Stellar** — red blockchain de pagos descentralizados  
- **Soroban** — smart contracts para Stellar  
- **Rust** — lenguaje de implementación de los contratos

Cada día trae un reto diferente, indicado con:
- una descripción temática (por ejemplo: *Árbol Digital*, *Renos Voladores*)
- un nivel de dificultad (`easy`, `medium`, `hard`)  
- código y tests asociados en este repositorio
  
---
# Visita el calendario visual en:  
## ➡️ https://adviento2025.buendiabuilders.com/  

---

## 📁 Estructura del proyecto

```text
adviento2025/
├── Cargo.toml          # Workspace principal
├── Cargo.lock
├── README.mkd          # Este archivo
├── contracts/
│   ├── dia01_primer_regalo_de_navidad/
│   ├── dia02_arbol_digital/
│   ├── dia03_cartas_a_santa/
│   ├── dia04_tokens_de_jengibre/
│   ├── dia05_trineo_automatizado/
│   ├── dia06_luces_navidenas/
│   ├── dia07_estrella_del_norte/
│   ├── dia08_elfos_trabajadores/
│   ├── dia09_calcetines_magicos/
│   ├── dia10_renos_voladores/
│   ├── dia11_campanas_sonoras/
│   ├── dia12_nieve_cristalina/
│   ├── dia13_chocolate_caliente/
│   ├── dia14_velas_brillantes/
│   ├── dia15_corona_de_adviento/
│   ├── dia16_muneco_de_nieve/
│   ├── dia17_baston_de_caramelo/
│   ├── dia18_taller_de_santa/
│   ├── dia19_guirnaldas_festivas/
│   ├── dia20_villancicos_digitales/
│   ├── dia21_chimenea_acogedora/
│   ├── dia22_regalo_sorpresa/
│   ├── dia23_posada_navidena/
│   ├── dia24_cena_de_nochebuena/
│   └── dia25_gran_final_stellar/
└── target/
```

## 📦 Estructura de cada día

 - Cada carpeta diaXX_* contiene:
 - Cargo.toml — crate independiente del día
 - src/lib.rs — implementación del contrato
 - tests/ — tests de integración
 - README.mkd — descripción del reto y consignas

## 🦀 Requisitos
 * Rust estable (instalado vía rustup)
 * Cargo


## Comprobar instalación:
```bash
rustc --version
cargo --version
```

## 🔧 Compilar todo el calendario

Desde la raíz del proyecto:
```bash
cargo build
```
## 🧪 Ejecutar todos los tests
```bash
cargo test
```

Ejecutar un día específico:
```bash
cargo test -p dia22_regalo_sorpresa
```

## 🎁 Objetivo del proyecto


- [x] Practicar Rust aplicado a smart contracts
- [x] Aprender Soroban y el ecosistema Stellar
- [x] Fomentar el aprendizaje diario a través de pequeños retos
- [x] Disfrutar el proceso 🚀🎄
