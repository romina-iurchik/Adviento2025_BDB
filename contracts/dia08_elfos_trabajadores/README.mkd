# Elfos Trabajadores - Difícil
## 📖 Historia
En el taller de Santa, cada elfo fabrica juguetes y necesitamos llevar registro de cuántos juguetes fabricó cada uno. Introducción a Map en Soroban (equivalente a HashMap/diccionarios).

## 🎯 Concepto: Map (Diccionarios)
Lo que vas a aprender:

- Map<K, V> - Estructura de datos clave-valor en Soroban
- Map<Symbol, u32> - Mapeo de nombres de elfos a cantidad de juguetes
- .set(key, value) - Insertar o actualizar valores
- .get(key) - Obtener valores por clave
- .unwrap_or(default) - Manejar claves inexistentes con valor por defecto
- .contains_key(key) - Verificar si una clave existe
- .clone() - Clonar Symbol cuando se usa múltiples veces
## Equivalencia con Rust normal:
Rust puro: HashMap<String, u32>
Soroban: Map<Symbol, u32>
## 💡 ¿Por qué esto importa para Soroban?
Los Map son fundamentales en contratos inteligentes: balances de tokens, registros de usuarios, configuraciones, etc. A diferencia de HashMap que está en memoria, Map de Soroban persiste en blockchain.

## ✅ Objetivos
Implementá 4 métodos que gestionan un registro de elfos y sus juguetes:

* registrar_elfo(env: Env, mut elfos: Map<Symbol, u32>, nombre: Symbol) -> Map<Symbol, u32>

* Registrar un elfo con contador inicial en 0
  - Usar elfos.set(nombre, 0);
  - Retornar el map actualizado
  - fabricar_juguetes(env: Env, mut elfos: Map<Symbol, u32>, nombre: Symbol,   - cantidad: u32) -> Map<Symbol, u32>

* Obtener juguetes actuales del elfo: elfos.get(nombre.clone()).unwrap_or(0)
  - Incrementar por la cantidad fabricada: actual + cantidad
  - Actualizar el map: elfos.set(nombre, actual + cantidad);
  - Importante: Usar .clone() en nombre porque se usa dos veces (en .get() y en .set())

* Retornar el map actualizado
  - contar_juguetes(env: Env, elfos: Map<Symbol, u32>, nombre: Symbol) -> u32

* Retornar la cantidad de juguetes del elfo
  - Usar elfos.get(nombre).unwrap_or(0) para retornar 0 si no existe
  - existe_elfo(env: Env, elfos: Map<Symbol, u32>, nombre: Symbol) -> bool

* Verificar si el elfo está registrado
Usar elfos.contains_key(nombre)
### Ejemplos
Entrada:
``` Rust
registrar_elfo(map, 'Pepe')
```
Salida:
``` Rust
Map con Pepe: 0
```
Entrada:
``` Rust
fabricar_juguetes(map, 'Pepe', 10)
```
Salida:
``` Rust
Map con Pepe: 10
```
Entrada:
``` Rust
contar_juguetes(map, 'Pepe')
```
Salida:
``` Rust
10
```
Entrada:
``` Rust
existe_elfo(map, 'Pepe')
```
Salida:
``` Rust
true
```
Entrada:
``` Rust
contar_juguetes(map, 'NoExiste')
```
Salida:
``` Rust
0
```