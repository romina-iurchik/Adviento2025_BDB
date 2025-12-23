# Corona de Adviento - Difícil
## 📖 Historia
La corona de Adviento tiene 4 velas que se van encendiendo semana a semana hasta Navidad. Cada domingo se enciende una nueva vela. Aprendamos a usar Vec de structs y gestionar colecciones complejas!

## 🎯 Concepto: Vec de Structs Complejos
Lo que vas a aprender:

- Trabajar con Vec<Struct> (listas de objetos complejos)
- Iterar sobre structs en un vector
- Contar elementos que cumplen condiciones
- Modificar elementos específicos en un vector

¿Por qué esto importa?
- En contratos reales es común tener listas de objetos:
- Lista de órdenes de compra
- Lista de usuarios con roles
- Lista de tokens en un portfolio
- Lista de transacciones pendientes

## 💡 ¿Por qué esto importa para Soroban?
Los Vec<Struct> son fundamentales para gestionar colecciones de datos complejos. En un marketplace: Vec<Producto>, en un DAO: Vec<Propuesta>, en un juego: Vec<Personaje>. Saber iterar, filtrar y modificar elementos es esencial.

## ✅ Objetivos
Completá las 5 funciones para gestionar la corona de Adviento:

* crear_corona(env: Env) -> Vec<VelaCorona>
 - Crear un Vec vacío: Vec::new(&env)
 - Iterar con for i in 1..=4
 - Agregar 4 VelaCorona con número 1 a 4 y encendida: false
 - Retornar el vector

* encender_siguiente(env: Env, velas: Vec<VelaCorona>) -> Vec<VelaCorona>
 - Crear un Vec nuevo
 - Iterar sobre las velas: for i in 0..velas.len()
 - Obtener cada vela: let mut vela = velas.get(i).unwrap();
 - Si la vela NO está encendida y no encendimos ninguna todavía: encenderla
 - Agregar la vela al Vec nuevo
 - Retornar el vector actualizado

* contar_encendidas(_env: Env, velas: Vec<VelaCorona>) -> u32
 - Crear contador = 0
 - Iterar sobre las velas
 - Si vela.encendida == true: incrementar contador
 - Retornar contador

* todas_encendidas(env: Env, velas: Vec<VelaCorona>) -> bool
 - Llamar a Self::contar_encendidas(env, velas)
 - Comparar con 4
 - Retornar el resultado

* apagar_todas(env: Env, velas: Vec<VelaCorona>) -> Vec<VelaCorona>
 - Crear Vec nuevo
 - Iterar sobre las velas
 - Modificar cada vela: vela.encendida = false;
 - Agregar al Vec nuevo
 - Retornar el vector actualizado
### Ejemplos
Entrada:
``` Rust
crear_corona()
```
Salida:
``` Rust
Vec con 4 VelaCorona, todas apagadas
```
Entrada:
``` Rust
encender_siguiente(corona)
```
Salida:
``` Rust
Solo la primera vela se enciende
```
Entrada:
``` Rust
contar_encendidas(corona_con_2_encendidas)
```
Salida:
``` Rust
2
```