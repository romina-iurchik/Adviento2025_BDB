# Primera Regalo de Navidad - Fácil

## 📖 Historia
¡Santa está organizando su lista de regalos para Navidad! Necesita tu ayuda para llevar el control usando variables en Rust.

## 🎯 Lo que vas a aprender
En este primer día del Adviento Stellar, aprenderás los fundamentos de Rust:

- Variables inmutables: En Rust, por defecto las variables NO pueden cambiar
- Tipos numéricos: u32 (número entero sin signo de 32 bits)
- Operadores: + para sumar, >= para comparar
- Retorno implícito: La última expresión sin ; se retorna automáticamente
## 💡 ¿Por qué esto importa para Soroban?
En blockchain TODO es inmutable por defecto para evitar bugs costosos. Rust nos entrena en este mindset desde el día 1.

Los contratos inteligentes manejan dinero real, por eso Rust nos OBLIGA a ser explícitas sobre qué puede cambiar y qué no.

✅ Objetivos
Completá las 3 funciones para que todos los tests pasen:

* contar_regalos() - Debe retornar 100
* sumar_regalos() - Debe sumar dos números
* hay_suficientes() - Debe verificar si hay al menos 50 regalos


### Ejemplos:

Entrada:
``` Rust
contar_regalos()
```
Salida:
``` Rust
100
```
Entrada:
``` Rust
sumar_regalos(30, 20)
```
Salida:
``` Rust
50
```
Entrada:
``` Rust
hay_suficientes(50)
```
Salida:
``` Rust
true
```