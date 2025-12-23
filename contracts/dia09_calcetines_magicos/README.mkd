# Calcetines Mágicos - Medio
## 📖 Historia
Los calcetines navideños cuelgan de la chimenea. Cada niño tiene su calcetín y dentro va acumulando regalos. Sistema para organizar estructuras de datos anidadas: un Map que contiene Vecs.

## 🎯 Concepto: Estructuras de Datos Anidadas
Lo que vas a aprender:

- Estructuras anidadas: Map<Symbol, Vec<Symbol>> - Diccionario de listas
- Combinar múltiples estructuras de datos de Soroban
- Vec::new(&env) - Crear vectores vacíos
- .unwrap_or(Vec::new(&env)) - Manejar valores opcionales con vector vacío por defecto
- mut en variables locales para modificar estructuras anidadas
- .clone() cuando se usa una key múltiples veces
## Aplicación en el mundo real:
Este patrón es común en contratos reales:

- Usuario → Lista de tokens poseídos
- Proyecto → Lista de contribuidores
- Wallet → Lista de transacciones
- DAO → Lista de propuestas por categoría
## 💡 ¿Por qué esto importa para Soroban?
Los contratos reales necesitan organizar datos complejos. Map<K, Vec<V>> es el patrón fundamental para relaciones uno-a-muchos en blockchain.

## ✅ Objetivos
Implementá 4 métodos que gestionan calcetines (Map) con listas de regalos (Vec):

* crear_calcetin(env: Env, mut calcetines: Map<Symbol, Vec<Symbol>>, niño: Symbol) -> Map<Symbol, Vec<Symbol>>
  - Crear un Vec vacío: Vec::new(&env)
  - Asignarlo al niño en el map: calcetines.set(niño, vacio);
  - Retornar el map actualizado

* agregar_regalo(env: Env, mut calcetines: Map<Symbol, Vec<Symbol>>, niño: Symbol, regalo: Symbol) -> Map<Symbol, Vec<Symbol>>
  - Obtener Vec existente o crear uno vacío: calcetines.get(niño.clone()).unwrap_or(Vec::new(&env))
  - Declarar como mut para modificarlo: let mut regalos = ...
  - Agregar el regalo: regalos.push_back(regalo);
  - Actualizar el map: calcetines.set(niño, regalos);
  - Importante: Usar .clone() en niño porque se usa dos veces
  - Retornar el map actualizado

* contar_regalos(env: Env, calcetines: Map<Symbol, Vec<Symbol>>, niño: Symbol) -> u32
  - Obtener Vec y retornar su longitud: calcetines.get(niño).unwrap_or(Vec::new(&env)).len()
  - Encadenar .len() directamente sobre el resultado de .unwrap_or()

* ver_regalos(env: Env, calcetines: Map<Symbol, Vec<Symbol>>, niño: Symbol) -> Vec<Symbol>
  - Retornar el Vec del niño o uno vacío: calcetines.get(niño).unwrap_or(Vec::new(&env))
### Ejemplos
Entrada:
``` Rust
crear_calcetin(map, 'Ana')
```
Salida:
``` Rust
Map con Ana: []
```
Entrada:
``` Rust
agregar_regalo(map, 'Ana', 'bici')
```
Salida:
``` Rust
Map con Ana: ['bici']
```
Entrada:
``` Rust
contar_regalos(map, 'Ana')
```
Salida:
``` Rust
1
```
Entrada:
``` Rust
ver_regalos(map, 'Luis')
```
Salida:
``` Rust
['bici', 'pelota', 'libro']
```