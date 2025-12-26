# Trineo Automatizado - Medio
## 📖 Historia
¡PRIMER contrato inteligente en Soroban! El trineo de Santa necesita un sistema automatizado para gestionar regalos. Este día marca la transición de Rust puro a Soroban (smart contracts en Stellar blockchain).

## 🎯 Concepto: Primer Contrato Soroban
Lo que vas a aprender:

- #[contract] - Macro que define un contrato inteligente
- #[contractimpl] - Implementa los métodos públicos del contrato
- Env - Entorno de ejecución de Soroban
- Symbol - Tipo optimizado para texto corto en blockchain
- Vec<Symbol> - Vector de Soroban (similar a Vec de Rust)

## Diferencias clave Rust → Soroban:
- String → Symbol
- Vec<String> → Vec<Symbol>
- .push() → .push_back()
- .len() funciona igual en ambos
## 💡 ¿Por qué esto importa para Soroban?
Este es tu primer contrato inteligente real. Todo lo que aprendiste de Rust se aplica aquí, solo cambian los tipos para optimizar el blockchain. Los contratos de Soroban son Rust con macros especiales.

## ✅ Objetivos
Implementá 3 métodos públicos en el contrato TrineoContract:

* agregar_regalo(env: Env, regalos: Vec<Symbol>, regalo: Symbol) -> Vec<Symbol>

* Agregar un regalo a la lista usando .push_back()
 - contar_regalos(env: Env, regalos: Vec<Symbol>) -> u32

* Retornar la cantidad de regalos usando .len()
 - puede_despegar(env: Env, regalos: Vec<Symbol>) -> bool

* Verificar que haya mínimo 3 regalos (regalos.len() >= 3)

### Ejemplos
Entrada:
``` Rust
agregar_regalo(env, vec![&env], symbol_short!("bici"))
```
Salida:
``` Rust
Vec con 1 elemento
```
Entrada:
``` Rust
contar_regalos(env, vec![&env, symbol_short!("a"), symbol_short!("b")])
```
Salida:
``` Rust
2
```
Entrada:
``` Rust
puede_despegar(env, vec![&env, symbol_short!("a"), symbol_short!("b")])
```
Salida:
``` Rust
false
```