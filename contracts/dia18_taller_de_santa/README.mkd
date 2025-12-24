# Taller de Santa - Difícil
## 📖 Historia
En el taller de Santa, los elfos fabrican juguetes y los organizan por categorías. Necesitamos un sistema para registrar juguetes, buscarlos por nombre, y llevar estadísticas de producción. Combinemos Storage + Map + Struct!

## 🎯 Concepto: Sistema Completo de Gestión
Lo que vas a aprender:
- Combinar Storage persistente con Map
- Buscar elementos por clave
- Contar elementos que cumplen condiciones
- Actualizar registros existentes
¿Por qué esto importa?
Este patrón es la base de la mayoría de contratos reales:

- Registrar usuarios y sus datos
- Inventario de productos
- Sistema de votación
- Marketplace de NFTs
- Es el día donde juntamos TODO lo aprendido.

## 💡 Importante: Iterar sobre Map
Para iterar sobre un Map en Soroban:

// No podés hacer "for juguete in mapa" directamente
// Necesitás usar .keys() para obtener las claves primero
``` Rust
let keys = juguetes.keys();
for i in 0..keys.len() {
    let key = keys.get(i).unwrap();
    if let Some(juguete) = juguetes.get(key) {
        // trabajar con el juguete
    }
}
```
## ✅ Objetivos
Completá las funciones para gestionar el taller de juguetes:

* registrar_juguete(env: Env, nombre: Symbol, categoria: Symbol, cantidad: u32)
  - Obtener Map del storage (o crear nuevo)
  - Crear struct Juguete
  - Guardar en el Map
  - Persistir en storage
  - 
* obtener_juguete(env: Env, nombre: Symbol) -> Option<Juguete>
  - Obtener Map del storage
  - Buscar juguete por nombre
  - Retornar Option<Juguete>
  - 
* existe_juguete(env: Env, nombre: Symbol) -> bool
  - Obtener Map del storage
  - Usar .contains_key()
  - 
* contar_por_categoria(env: Env, categoria: Symbol) -> u32
  - Obtener Map del storage
  - Iterar sobre las keys
  - Contar los que coinciden con la categoría
  - 
* actualizar_cantidad(env: Env, nombre: Symbol, nueva_cantidad: u32)
  - Obtener Map del storage
  - Buscar juguete
  - Modificar campo cantidad
  - Guardar cambios en storage
### Ejemplos
Entrada:
``` Rust
registrar_juguete(symbol_short!("oso"), symbol_short!("peluche"), 10)
```
Salida:
``` Rust
Juguete registrado en el taller
```
Entrada:
``` Rust
contar_por_categoria(symbol_short!("peluche"))
```
Salida:
``` Rust
2
```