# Muñeco de Nieve - Difícil
## 📖 Historia
Los niños construyen un muñeco de nieve con 3 bolas de diferentes tamaños. Pero el muñeco se derrite con el calor y crece con más nieve. Aprendamos a gestionar estados que pueden cambiar completamente!

## 🎯 Concepto: Gestión de Estado Complejo
Lo que vas a aprender:

- Validar que un struct cumpla requisitos (las 3 bolas deben existir)
- Calcular propiedades derivadas (altura total)
- Modificar múltiples campos a la vez
- Usar Option<T> implícitamente (estados que pueden no existir)

¿Por qué esto importa?
- Los contratos reales manejan estados que pueden estar completos o incompletos:
- KYC de usuarios (con/sin documentos verificados)
 - Préstamos (pendientes, activos, pagados)
 - Propuestas de gobernanza (borrador, activa, ejecutada)
 - NFTs (acuñados vs. quemados)

## 💡 ¿Por qué esto importa para Soroban?
Los structs complejos con validaciones son la base de los contratos seguros. Validar estados antes de operar previene errores y ataques. Un muñeco sin bolas no debería poder calcular altura.

## ✅ Objetivos
Completá las 5 funciones para gestionar el muñeco de nieve:

* crear_muneco(_env: Env, inferior: u32, media: u32, superior: u32) -> Muneco
  - Crear un Muneco con los valores dados
  - bola_inferior: inferior
  - bola_media: media
  - bola_superior: superior

* altura_total(_env: Env, muneco: Muneco) -> u32
  - Sumar los 3 radios
  - Multiplicar por 2 (para obtener diámetros)
  - Retornar el resultado

* esta_completo(_env: Env, muneco: Muneco) -> bool
  - Verificar que las 3 bolas tengan radio > 0
  - Usar el operador &&
  - Retornar true si todas cumplen

* derretir(_env: Env, mut muneco: Muneco, cantidad: u32) -> Muneco
  - Para cada bola, restar la cantidad
  - Si quedaría negativa, dejarla en 0
  - Usar if cantidad > muneco.bola_X para cada bola
  - Retornar el muneco modificado

* agregar_nieve(_env: Env, mut muneco: Muneco, bola: u32, cantidad: u32) -> Muneco
  - Si bola == 1: incrementar bola_inferior
  - Si bola == 2: incrementar bola_media
  - Si bola == 3: incrementar bola_superior
  - Retornar el muneco modificado

### Ejemplos
Entrada:
``` Rust
crear_muneco(50, 35, 20)
```
Salida:
``` Rust
Muneco { bola_inferior: 50, bola_media: 35, bola_superior: 20 }
```
Entrada:
``` Rust
altura_total(muneco)
```
Salida:
``` Rust
210 (50+35+20)*2
```
Entrada:
``` Rust
derretir(muneco, 10)
```
Salida:
``` Rust
Cada bola reduce 10cm
```