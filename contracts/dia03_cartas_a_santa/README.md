# Cartas a Santa - Medio
## 📖 Historia
Los niños le escriben cartas a Santa, pero a veces las cartas pueden tener problemas: son muy cortas, muy largas o sin nombre. ¡Aprendamos a manejar estos errores con Result!

## 🎯 Concepto: Result y Manejo de Errores
Lo que vas a aprender:

- Result<T, E> - Un tipo que puede ser éxito (Ok) o error (Err)
- Ok(valor) - La operación fue exitosa
- Err(error) - La operación falló
- enum - Para definir los tipos de errores posibles
- Validaciones - Cómo verificar que los datos sean correctos
## 💡 ¿Por qué esto importa para Soroban?
Los contratos inteligentes DEBEN manejar errores (fondos insuficientes, permisos denegados, datos inválidos). Result es LA forma de hacerlo en Rust y Soroban. Un contrato sin manejo de errores es un contrato inseguro.

## ✅ Objetivos
Completá las 2 funciones para que todos los tests pasen:

* validar_carta() - Verificar que la carta tenga el largo correcto
  - Debe retornar Err(ErrorCarta::MuyCorta) si tiene menos de 5 caracteres
  - Debe retornar Err(ErrorCarta::MuyLarga) si tiene más de 100 caracteres
  - Debe retornar Ok(carta.to_string()) si está bien

* validar_nombre() - Verificar que la carta tenga un nombre
  - Debe retornar Err(ErrorCarta::SinNombre) si el nombre está vacío
  - Debe retornar Ok(nombre.to_string()) si tiene nombre
### Ejemplos
Entrada:
``` Rust
validar_carta("Querido Santa, quiero una bici")
```
Salida:
``` Rust
Ok("Querido Santa, quiero una bici")
```
Entrada:
``` Rust
validar_carta("Hola")
```
Salida:
``` Rust
Err(ErrorCarta::MuyCorta)
```
Entrada:
``` Rust
validar_nombre("Ana")
```
Salida:
``` Rust
Ok("Ana")
```
Entrada:
``` Rust
validar_nombre("")
```
Salida:
``` Rust
Err(ErrorCarta::SinNombre)
```