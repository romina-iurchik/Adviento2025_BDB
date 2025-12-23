# Luces Navideñas - Fácil
## 📖 Historia
Los elfos instalan luces navideñas y necesitan un contador que NO se pierda cuando apagan el sistema. Introducción al almacenamiento persistente en blockchain (Storage).

## 🎯 Concepto: Storage Persistente
Lo que vas a aprender:

- Storage persistente: Los datos persisten en blockchain entre llamadas
- env.storage().instance() - Almacenamiento de instancia del contrato
- .set(&key, &value) - Guardar datos
- .get(&key) - Recuperar datos
- .unwrap_or(default) - Manejar valores opcionales con valor por defecto
- const COUNTER: Symbol - Constante para la key del storage

## Diferencia clave con Rust normal:
Rust puro: Variables desaparecen al terminar la función
Soroban: Storage persiste en blockchain indefinidamente
## 💡 ¿Por qué esto importa para Soroban?
El storage persistente es la base de los contratos inteligentes. Tus datos se guardan en la blockchain y nadie puede borrarlos. Un contador simple hoy, mañana será el balance de tokens o el estado de un juego.

## ✅ Objetivos
* Implementá 4 métodos que manejan un contador persistente:

* inicializar(env: Env)
 - Guardar 0 en el storage: env.storage().instance().set(&COUNTER, &0_u32);
  
* encender_luz(env: Env) -> u32
  - Obtener valor actual del storage
  - Incrementar en 1
  - Guardar nuevo valor en storage
  - Retornar el nuevo valor

* contar_luces(env: Env) -> u32
  - Leer y retornar el valor del storage: env.storage().instance().get(&COUNTER).unwrap_or(0)

* apagar_todo(env: Env)
  - Resetear contador a 0 en el storage
### Ejemplos
Entrada:
``` Rust
inicializar(env) -> contar_luces(env)
```
Salida:
``` Rust
0
```
Entrada:
``` Rust
encender_luz(env) -> encender_luz(env) -> encender_luz(env)
```
Salida:
``` Rust
1, 2, 3
```
Entrada:
``` Rust
encender_luz(env) x2 -> contar_luces(env)
```
Salida:
``` Rust
2 (persiste entre llamadas)
```
Entrada:
``` Rust
encender_luz(env) x2 -> apagar_todo(env) -> contar_luces(env)
```
Salida:
``` Rust
0 (reseteado)
```