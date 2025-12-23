# Campanas Sonoras - Fácil
## 📖 Historia
Las campanas del pueblo suenan anunciando la Navidad. Cada campanada se registra con su timestamp del blockchain. Introducción al ledger de Soroban para trabajar con tiempo en blockchain.

## 🎯 Concepto: Ledger y Timestamps en Soroban
Lo que vas a aprender:

- env.ledger() - Acceso al estado del ledger (blockchain)
- env.ledger().timestamp() - Obtiene el timestamp actual del bloque (en segundos Unix)
- env.ledger().sequence() - Obtiene el número de bloque (no se usa en este ejercicio, pero es útil saberlo)
- Tiempo en blockchain vs tiempo real del sistema
- Operaciones con vectores: .get(), .len(), .is_empty(), .push_back()
- Manejo de casos edge: vectores vacíos, un solo elemento
¿Por qué esto importa en blockchain?
- En blockchain, el tiempo NO es el reloj del sistema operativo. Es el timestamp del bloque actual. Esto es crítico para:
  - Contratos con plazos: Vesting, locks, préstamos con fecha de vencimiento
  - Subastas temporales: Ofertas con tiempo límite
  - Registros históricos: Marcas de tiempo inmutables y verificables
  - Coordinación distribuida: Todos los nodos usan el mismo tiempo
## 💡 ¿Por qué esto importa para Soroban?
Los timestamps son fundamentales en contratos DeFi, NFTs con tiempo limitado, sistemas de votación con plazos, y cualquier lógica temporal en blockchain.

## ✅ Objetivos
Implementá 4 métodos que gestionan campanadas con timestamps:

* sonar(env: Env, mut campanadas: Vec<u64>) -> Vec<u64>
  - Obtener timestamp del ledger: let timestamp = env.ledger().timestamp();
  - Agregar al vector: campanadas.push_back(timestamp);
  - Retornar el vector actualizado
  - Importante: El vector es mut porque se modifica

* contar_campanadas(_env: Env, campanadas: Vec<u64>) -> u32
  - Retornar la cantidad de elementos: campanadas.len()
  - Nota: El parámetro _env tiene underscore porque no se usa tiempo_transcurrido(_env: Env, campanadas: Vec<u64>) -> u64
  - Si hay menos de 2 campanadas, retornar 0
  - Validación: if campanadas.len() < 2 { return 0; }
  - Obtener primera: let primera = campanadas.get(0).unwrap();
  - Obtener última: let ultima = campanadas.get(campanadas.len() - 1).unwrap();
  - Retornar diferencia: ultima - primera
  - Importante: Orden de validación primero para evitar unwrap en vector vacío

* ultima_campanada(_env: Env, campanadas: Vec<u64>) -> u64
  - Si el vector está vacío, retornar 0
  - Validación: if campanadas.is_empty() { return 0; }
  - Retornar último elemento: campanadas.get(campanadas.len() - 1).unwrap()
### Ejemplos
Entrada:
``` Rust
sonar(vec![], timestamp=1000)
```
Salida:
``` Rust
vec![1000]
```
Entrada:
``` Rust
contar_campanadas(vec![1000, 2000, 3000])
```
Salida:
``` Rust
3
```
Entrada:
``` Rust
tiempo_transcurrido(vec![1000, 2000, 3000])
```
Salida:
``` Rust
2000 (3000 - 1000)
```
Entrada:
``` Rust
ultima_campanada(vec![1000, 2000, 3000])
```
Salida:
``` Rust
3000
```