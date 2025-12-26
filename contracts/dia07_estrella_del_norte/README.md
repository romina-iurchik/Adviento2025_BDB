# Estrella del Norte - Difícil
## 📖 Historia
La estrella más brillante del cielo guía a los Reyes Magos, pero solo puede brillar si tiene suficiente energía (mínimo 10). Introducción al manejo de errores personalizados en Soroban usando #[contracterror].

## 🎯 Concepto: Errores Personalizados
Lo que vas a aprender:

- #[contracterror] - Macro para definir errores personalizados del contrato
- #[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)] - Traits requeridos
- #[repr(u32)] - Representación numérica de los errores
- Result<T, Error> - Tipo de retorno para funciones que pueden fallar
- Validación con if y retorno temprano con return Err(...)
- Códigos de error numéricos (optimización para blockchain)
## Diferencia clave con Rust normal:
Rust puro: Usamos Result<T, String> o anyhow::Error con mensajes descriptivos
Soroban: Usamos enum con códigos numéricos u32 (más eficiente en blockchain, reduce costos de gas)
## 💡 ¿Por qué esto importa para Soroban?
En blockchain, cada byte cuenta. Los errores con códigos numéricos son más eficientes que strings. Este patrón es estándar en todos los contratos Soroban productivos.

## ✅ Objetivos
Implementá 3 métodos que validan y retornan Result<T, Error>:

* encender(env: Env, energia: u32) -> Result<u32, Error>
  - Si energia < 10 → retornar Err(Error::EnergiaInsuficiente)
  - Si no → retornar Ok(energia)
  
* puede_brillar(env: Env, energia: u32, ya_encendida: bool) -> Result<bool, Error>
  - Si ya_encendida == true → retornar Err(Error::YaBrillando)
  - Si energia < 10 → retornar Err(Error::EnergiaInsuficiente)
  - Si no → retornar Ok(true)
  - Importante: El orden de validación importa (primero verificar si ya está encendida)

* cargar_energia(env: Env, actual: u32, incremento: u32) -> Result<u32, Error>
  - Calcular total = actual + incremento
  - Si total < 10 → retornar Err(Error::EnergiaInsuficiente)
  - Si no → retornar Ok(total)
### Ejemplos
Entrada:
``` Rust
encender(15)
```
Salida:
``` Rust
Ok(15)
```
Entrada:
``` Rust
encender(5)
```
Salida:
``` Rust
Err(Error::EnergiaInsuficiente)
```
Entrada:
``` Rust
puede_brillar(15, true)
```
Salida:
``` Rust
Err(Error::YaBrillando)
```
Entrada:
``` Rust
puede_brillar(15, false)
```
Salida:
``` Rust
Ok(true)
```
Entrada:
``` Rust
cargar_energia(5, 10)
```
Salida:
``` Rust
Ok(15)
```
Entrada:
``` Rust
cargar_energia(3, 5)
```
Salida:
``` Rust
Err(Error::EnergiaInsuficiente)
```