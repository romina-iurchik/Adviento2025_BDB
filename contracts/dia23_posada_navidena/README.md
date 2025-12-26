# Posada Navideña - Medio
## 📖 Historia
En la posada navideña llegan peregrinos buscando refugio. Cada habitación tiene una capacidad máxima y puede estar ocupada o disponible. ¡Aprendamos a gestionar reservas y ocupación de espacios!

## 🎯 Lo que vas a aprender: Sistema de Reservas y Capacidad

- Gestionar recursos limitados: Cada habitación tiene capacidad máxima
- Validar capacidad antes de asignar: No se puede reservar más del espacio disponible
- Liberar recursos ocupados: Los peregrinos pueden irse liberando espacio
- Calcular disponibilidad: Verificar si hay espacio antes de reservar
- Map con structs complejos: Almacenar múltiples habitaciones con su estado
## 💡 ¿Por qué esto importa?
Este patrón es esencial para:

- Sistemas de reservas (hoteles, vuelos, eventos)
- Gestión de recursos limitados (pools de liquidez)
- Asignación de espacios (parking, storage)
- Control de capacidad en eventos
## ✅ Objetivos
Implementá las funciones para gestionar la posada:

* crear_habitacion() - Crea una habitación con capacidad inicial
* reservar() - Reserva espacio verificando capacidad disponible
* liberar() - Libera espacio ocupado sin llegar a negativo
* esta_disponible() - Verifica si hay espacio disponible
* ocupacion_total() - Calcula la ocupación total de la posada
* guardar_habitacion() - Guarda una habitación en storage

Ejemplos
Entrada:
``` Rust
crear_habitacion(1, 4)
```
Salida:
``` Rust
Habitacion { numero: 1, capacidad: 4, ocupados: 0 }
```
Entrada:
``` Rust
reservar(1, 2) con habitacion de capacidad 4
```
Salida:
``` Rust
Ok(()) y ocupados pasa a 2
```
Entrada:
``` Rust
esta_disponible(1) con 2 ocupados de 4
```
Salida:
``` Rust
true
```