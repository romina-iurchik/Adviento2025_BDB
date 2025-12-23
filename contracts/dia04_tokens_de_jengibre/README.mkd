# Tokens de Jengibre - Fácil
## 📖 Historia
La abuela de Santa hornea galletas de jengibre para los elfos. Cada galleta tiene un nombre y un peso. Usaremos struct para organizarlas.

## 🎯 Concepto: Structs e impl
Lo que vas a aprender:

- struct - Agrupar datos relacionados
- impl - Implementar métodos
- self - Referencia a la instancia actual
- Métodos vs Funciones - Los métodos se llaman sobre instancias
## 💡 ¿Por qué esto importa para Soroban?
En Soroban (blockchain de Stellar) los structs son fundamentales. Los contratos y datos complejos son structs. Todo lo que almacenás en la blockchain es un struct.

## ✅ Objetivos
Implementá 4 funciones para el struct Galleta:

* Galleta::nueva(nombre, peso_gramos) - Constructor

  - Debe crear una nueva instancia de Galleta con los valores dados
es_grande() - Método de instancia

  - Debe retornar true si el peso es mayor a 50 gramos
describir() - Método de instancia

  - Debe retornar un String con formato: "Galleta {nombre} de {peso}g"
peso_total(galletas: &Vec<Galleta>) - Función independiente

  - Debe sumar el peso total de todas las galletas en el vector
### Ejemplos
Entrada:
``` Rust
Galleta::nueva("Estrella".to_string(), 30)
```
Salida:
``` Rust
Galleta { nombre: "Estrella", peso_gramos: 30 }
```
Entrada:
``` Rust
galleta.es_grande() // peso = 80g
```
Salida:
``` Rust
true
```
Entrada:
``` Rust
galleta.describir() // nombre = "Árbol", peso = 45g
```
Salida:
``` Rust
"Galleta Árbol de 45g"
```