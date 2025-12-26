# Guirnaldas Festivas - Difícil
## 📖 Historia
Las guirnaldas navideñas están hechas de luces de colores que pueden estar encendidas o apagadas. Cada guirnalda es una secuencia de luces. Aprendamos a trabajar con Vec de structs y patrones de encendido/apagado!

## 🎯 Concepto: Patrones y Secuencias
Lo que vas a aprender:

- Vec de structs con estado binario (encendida/apagada)
- Alternar estados (toggle)
- Encontrar posiciones específicas
- Aplicar patrones a secuencias
¿Por qué esto importa?
Los patrones de estado binario son comunes en blockchain:

- Estados de propuestas (aprobada/rechazada)
- Permisos de usuarios (activo/inactivo)
- Tokens bloqueados/desbloqueados
- Características habilitadas/deshabilitadas
## ✅ Objetivos
Completá las funciones para gestionar guirnaldas navideñas:

* crear_guirnalda(env: Env, cantidad: u32, color: Symbol) -> Vec<Luz>
  - Crear un Vec vacío
  - Agregar 'cantidad' luces apagadas
  - Cada luz tiene posición, color y está apagada

* encender_luz(env: Env, luces: Vec<Luz>, posicion: u32) -> Vec<Luz>
  - Recorrer todas las luces
  - Encender la luz en la posición especificada
  - Retornar nuevo Vec con el cambio

* alternar_luz(env: Env, luces: Vec<Luz>, posicion: u32) -> Vec<Luz>
  - Similar a encender_luz
  - Usar !luz.encendida para alternar
  - Si estaba encendida, apagarla (y viceversa)

* encender_alternadas(env: Env, luces: Vec<Luz>, pares: bool) -> Vec<Luz>
  - Encender luces en posiciones pares (0, 2, 4...) o impares (1, 3, 5...)
  - Usar módulo % 2 para verificar par/impar
  - Retornar nuevo Vec con el patrón aplicado

* contar_encendidas(env: Env, luces: Vec<Luz>) -> u32
  - Recorrer todas las luces
  - Contar las que tienen encendida == true
  - Retornar el contador

### Ejemplos
Entrada:
``` Rust
crear_guirnalda(5, symbol_short!("rojo"))
```
Salida:
``` Rust
Vec con 5 luces rojas apagadas
```
Entrada:
``` Rust
encender_alternadas(guirnalda, true)
```
Salida:
``` Rust
Enciende luces en posiciones 0, 2, 4...
```