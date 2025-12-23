# Árbol Digital - Fácil
## 📖 Historia
Los elfos están decorando el árbol de Navidad. Necesitan una lista para organizar todos los adornos.

## 🎯 Concepto: Vec (Listas Dinámicas)
Lo que vas a aprender:

- Vec<String> - Lista de textos
- .push() - Agregar elementos
- .len() - Contar elementos
- .get() - Acceder por índice
## 💡 ¿Por qué esto importa para Soroban?
Los contratos inteligentes trabajan con colecciones de datos todo el tiempo: usuarios, transacciones, tokens. Vec es la estructura fundamental para manejar listas dinámicas.

## ✅ Objetivos
Completá las 4 funciones para que todos los tests pasen:

* crear_lista() - Crear un vector vacío
* agregar_adorno() - Agregar un adorno al árbol
* contar_adornos() - Contar cuántos adornos hay
* primer_adorno() - Obtener el primer adorno de la lista
### Ejemplos

Entrada:
``` Rust
crear_lista()
```
Salida:
``` Rust
[]
```
Entrada:
``` Rust
agregar_adorno(lista, "Estrella")
```
Salida:
``` Rust
Vec con 1 elemento
```
Entrada:
``` Rust
contar_adornos(&lista)
```
Salida:
``` Rust
2
```