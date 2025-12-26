# Cena de Nochebuena - Medio
## 📖 Historia
¡Es Nochebuena! La mesa está llena de platillos deliciosos. Cada platillo tiene un nombre, cantidad de porciones y si está listo para servir. ¡Aprendamos a gestionar el menú completo y coordinar cuándo servir!

## 🎯 Lo que vas a aprender: Sistema de Coordinación y Estado

- Múltiples structs con estados relacionados: Cada platillo tiene su propio estado
- Validar que todos los elementos estén listos: Verificar antes de servir
- Calcular totales y promedios: Sumar porciones de todos los platillos
- Cambiar estados de forma masiva: Marcar platillos como listos
- Coordinar acciones dependientes: Solo servir cuando todo está listo

## 💡 ¿Por qué esto importa?
Este patrón es esencial para:

  - Sistemas multi-firma (todos deben aprobar)
  - Releases de software (todos los componentes listos)
  - Procesos de aprobación por etapas
  - Eventos con múltiples requisitos

## ✅ Objetivos
Implementá las funciones para gestionar la cena:

* agregar_platillo() - Agrega un platillo al menú (listo: false)
* marcar_listo() - Marca un platillo como listo para servir
* todos_listos() - Verifica si todos los platillos están listos
* porciones_totales() - Calcula el total de porciones del menú
* servir_cena() - Solo funciona si todos los platillos están listos
### Ejemplos
Entrada:
``` Rust
agregar_platillo("pavo", 8)
```
Salida:
``` Rust
Menu: [Platillo { nombre: pavo, porciones: 8, listo: false }]
```
Entrada:
``` Rust
marcar_listo("pavo")
```
Salida:
``` Rust
Ok(()) y el platillo pasa a listo: true
```
Entrada:
``` Rust
todos_listos() con 2 platillos, 1 listo
```
Salida:
``` Rust
false
```
