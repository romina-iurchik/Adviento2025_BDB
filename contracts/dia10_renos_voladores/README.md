# Renos Voladores - Medio
## 📖 Historia
Rudolph, Dasher, Dancer... cada reno tiene características únicas (nombre, velocidad, energía). Necesitamos crear un struct personalizado con #[contracttype] para representarlos y simular vuelos que consumen energía.

## 🎯 Concepto: Structs Personalizados en Soroban
Lo que vas a aprender:

- #[contracttype] - Macro que permite guardar structs en storage de Soroban
- #[derive(Clone, Debug, Eq, PartialEq)] - Traits requeridos para structs de contrato
- Structs con múltiples campos (nombre, velocidad, energia)
- Métodos que reciben y retornan structs modificados
- mut para modificar structs localmente
- Lógica de límites (energía entre 0 y 100)
- Cálculos con división entera
- Diferencia con Rust normal:
- Rust puro: Structs regulares con #[derive(...)]
- Soroban: #[contracttype] permite serializar/guardar en blockchain
- Fórmulas del ejercicio:
- Consumo de energía: distancia / velocidad
- Energía máxima: 100
- Energía mínima: 0
- Puede volar: (distancia / velocidad) <= energia
## 💡 ¿Por qué esto importa para Soroban?
Los structs personalizados son la base de los contratos reales. Tokens, NFTs, usuarios, propuestas de DAO... todo son structs con #[contracttype].

## ✅ Objetivos
Implementá 4 métodos que gestionan renos:

* crear_reno(env: Env, nombre: Symbol, velocidad: u32) -> Reno
  - Retornar un Reno con energia inicial de 100
  - Usar la sintaxis: Reno { nombre, velocidad, energia: 100 }

* volar(env: Env, mut reno: Reno, distancia: u32) -> Reno
  - Calcular consumo: distancia / reno.velocidad
  - Si consumo > reno.energia, poner energia en 0
  - Si no, restar: reno.energia -= consumo
  - Retornar el reno modificado
  - 
* descansar(env: Env, mut reno: Reno, cantidad: u32) -> Reno
  - Sumar cantidad a reno.energia
  - Si supera 100, limitarlo a 100
  - Retornar el reno modificado
  - 
* puede_volar(env: Env, reno: Reno, distancia: u32) -> bool
  - Calcular energía necesaria: distancia / reno.velocidad
  - Retornar true si necesario <= reno.energia
  - 
### Ejemplos
Entrada:
``` Rust
crear_reno('Rudolph', 10)
```
Salida:
``` Rust
Reno { nombre: Rudolph, velocidad: 10, energia: 100 }
```
Entrada:
``` Rust
volar(reno, 50)
```
Salida:
``` Rust
Reno con energia: 95 (consumió 5)
```
Entrada:
``` Rust
descansar(reno, 30)
Salida:
``` Rust
Reno con energia: 80 (restauró 30 desde 50)
```
Entrada:
``` Rust
puede_volar(reno, 500)
```
``` Rust
Salida:
true (necesita 50, tiene 100)
```