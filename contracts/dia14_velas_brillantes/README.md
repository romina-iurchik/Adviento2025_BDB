# Velas Brillantes - Medio
## 📖 Historia
En la mesa navideña hay velas de diferentes colores. Cada vela tiene un color, altura inicial y cuánto se ha consumido. Aprendamos a usar enums para representar colores y gestionar el estado de las velas!

## 🎯 Concepto: Enums + Structs Combinados
Lo que vas a aprender:

- Usar enum para representar opciones (colores)
- Combinar enum con struct
- Validar que una vela no se consuma más de lo que tiene
- Gestionar estado con múltiples propiedades
¿Por qué esto importa?
- Los enums son fundamentales en contratos inteligentes:
- Estados de una orden: Pendiente, Aprobada, Rechazada
- Tipos de tokens: Nativo, Custom
- Roles de usuarios: Admin, Usuario, Invitado
- Categorías de productos

## 💡 ¿Por qué esto importa para Soroban?
Los enums son esenciales para modelar estados finitos. En un contrato de subastas tenés: Activa, Finalizada, Cancelada. En un token: Activo, Pausado, Bloqueado. Los enums + structs permiten crear tipos de datos complejos y seguros.

## ✅ Objetivos
Completá las 5 funciones para gestionar velas:

* crear_vela(_env: Env, color: Color, altura: u32) -> Vela
 - Crear una Vela con el color y altura dados
 - Inicializar consumida: 0
 - Retornar el struct

* encender(_env: Env, mut vela: Vela, centimetros: u32) -> Vela
 - Incrementar vela.consumida += centimetros;
 - Si vela.consumida > vela.altura_inicial: limitar a altura_inicial
 - Retornar la vela modificada

* altura_actual(_env: Env, vela: Vela) -> u32
 - Calcular: vela.altura_inicial - vela.consumida
 - Retornar la altura restante

* esta_apagada(_env: Env, vela: Vela) -> bool
 - Verificar: vela.consumida >= vela.altura_inicial
 - Retornar true si está completamente consumida

* cambiar_color(_env: Env, mut vela: Vela, nuevo_color: Color) -> Vela
 - Actualizar: vela.color = nuevo_color;
 - Retornar la vela modificada

### Ejemplos
Entrada:
``` Rust
crear_vela(Color::Rojo, 20)
Salida:
``` Rust
Vela { color: Rojo, altura_inicial: 20, consumida: 0 }
```
Entrada:
``` Rust
encender(vela, 5)
```
Salida:
``` Rust
consumida: 5, altura_actual: 15
```
Entrada:
``` Rust
esta_apagada(vela_consumida)
```
Salida:
``` Rust
true (consumida >= altura_inicial)
```