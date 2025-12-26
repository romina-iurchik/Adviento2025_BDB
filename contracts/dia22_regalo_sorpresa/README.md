# Regalo Sorpresa - Medio
## 📖 Historia
Santa tiene regalos sorpresa envueltos en cajas misteriosas. Cada caja tiene un color y puede contener un regalo o estar vacía. ¡Solo al abrirla descubrís qué hay dentro! Aprendamos a trabajar con Option dentro de structs y revelar sorpresas.

## 🎯 Lo que vas a aprender: Option Anidado y Estado Revelable

- Option dentro de structs: Las cajas pueden tener o no contenido (Some/None)
- Cambiar estado de "envuelto" a "revelado": Controlar si una caja está abierta o cerrada
- Validar que no se pueda abrir dos veces: Usar Result para manejar errores
- Trabajar con datos que pueden o no existir: Option<Symbol> para el contenido
- Storage con estados mutables: Guardar múltiples cajas y actualizar su estado

## 💡 ¿Por qué esto importa?
Este patrón se usa en:

- NFTs revelables (como loot boxes en juegos)
- Apuestas/sorteos con resultados ocultos
- Sistemas de votación sellada
- Contratos con información encriptada hasta cierto momento

## ✅ Objetivos
Implementá las funciones para gestionar las cajas sorpresa:

* crear_caja() - Crea una caja cerrada con o sin contenido
* abrir_caja() - Abre la caja y revela el contenido (solo una vez)
* esta_abierta() - Verifica si la caja está abierta
* obtener_contenido() - Obtiene el contenido solo si está abierta
* guardar_caja() - Guarda una caja en el storage
* contar_cajas_cerradas() - Cuenta cuántas cajas cerradas hay
Ejemplos
Entrada:
``` Rust
crear_caja("rojo", Some("muneca"))
```
Salida:
``` Rust
Caja { color: "rojo", contenido: Some("muneca"), abierta: false }
```
Entrada:
``` Rust
abrir_caja(caja)
```
Salida:
``` Rust
Caja con abierta: true
```
Entrada:
``` Rust
obtener_contenido(caja cerrada)
```
Salida:
``` Rust
None
```
Entrada:
``` Rust
obtener_contenido(caja abierta con regalo)
```
Salida:
``` Rust
Some("muneca")
```