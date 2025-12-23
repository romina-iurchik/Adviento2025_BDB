# Chocolate Caliente - Medio
## 📖 Historia
En el taller de Santa, los elfos preparan chocolate caliente para entrar en calor. Cada taza tiene ingredientes (cacao, leche, azúcar) y una temperatura. Aprendamos a combinar Storage con Structs personalizados!

## 🎯 Concepto: Storage + Structs Complejos
Lo que vas a aprender:

- Guardar structs personalizados en el storage
- Trabajar con múltiples campos de datos
- Validar estados (temperatura correcta)
- Combinar Map + Struct en storage
¿Por qué esto importa?
- Los contratos reales guardan datos complejos en el blockchain:
- Perfiles de usuarios con múltiples campos
- Configuraciones de contratos
- Estados de juegos o aplicaciones
- Registros con validaciones
## 💡 ¿Por qué esto importa para Soroban?
En contratos reales, no solo guardás números simples. Guardás objetos complejos con validaciones: perfiles de usuarios, tokens con metadata, configuraciones del protocolo, etc.

## ✅ Objetivos
Completá las 5 funciones para gestionar tazas de chocolate:

* preparar_taza(_env: Env, cacao: u32, leche: u32, azucar: u32) -> Taza
  - Crear una Taza con los ingredientes dados
  - Temperatura inicial: 25°C (ambiente)
  - Retornar el struct

* calentar(_env: Env, mut taza: Taza, grados: u32) -> Taza
  - Incrementar temperatura: taza.temperatura += grados;
  - Retornar la taza modificada

* enfriar(_env: Env, mut taza: Taza, grados: u32) -> Taza
  - Si grados > taza.temperatura: poner en 0
  - Si no: restar taza.temperatura -= grados;
  - Retornar la taza modificada

* esta_perfecta(_env: Env, taza: Taza) -> bool
  - Verificar: taza.temperatura >= 60 && taza.temperatura <= 80
  - Temperatura perfecta: entre 60 y 80 grados

* agregar_ingrediente(_env: Env, mut taza: Taza, ingrediente: Symbol, cantidad: u32) -> Taza
  - Si ingrediente es "cacao": taza.cacao += cantidad;
  - Si ingrediente es "leche": taza.leche += cantidad;
  - Si ingrediente es "azucar": taza.azucar += cantidad;
  - Usar symbol_short!() para comparar
### Ejemplos
Entrada:
``` Rust
preparar_taza(10, 200, 5)
```
Salida:
``` Rust
Taza { cacao: 10, leche: 200, azucar: 5, temperatura: 25 }
```
Entrada:
``` Rust
calentar(taza, 45)
```
Salida:
``` Rust
Temperatura: 70°C (25 + 45)
```
Entrada:
``` Rust
esta_perfecta(taza_70)
```
Salida:
``` Rust
true (70°C está entre 60-80)
```