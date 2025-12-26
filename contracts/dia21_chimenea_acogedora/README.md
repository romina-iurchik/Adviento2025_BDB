# Chimenea Acogedora - Medio
## 📖 Historia
La chimenea es el corazón del hogar en Navidad. Necesita leña para mantener el fuego encendido. Cada leño tiene un tamaño y aporta calor. ¡Aprendamos a gestionar el fuego y calcular cuánto calor genera!

## 🎯 Lo que vas a aprender: Gestión de Recursos con Estado

- Struct con múltiples propiedades relacionadas: La chimenea tiene estado (encendida/apagada), cantidad de leños y tamaño promedio
- Cálculos derivados: El calor total se calcula basándose en las propiedades actuales
- Validaciones de estado: No se puede encender sin leños, no se puede quemar si no hay
- Consumo de recursos: Al quemar leños, la cantidad disminuye y puede apagar el fuego
- Agregar recursos dinámicamente: Los leños se agregan y se recalcula el promedio

## 💡 ¿Por qué esto importa?
Este patrón aparece en muchos contratos:

  - Gestión de energía/combustible en juegos blockchain
  - Sistemas de recursos consumibles (tokens, energía, tiempo)
  - Validaciones de estado antes de acciones
  - Cálculos basados en propiedades acumuladas
## ✅ Objetivos
Implementá las funciones para gestionar la chimenea:

* crear_chimenea()
   - Crea una chimenea apagada sin leños
* agregar_leno()
   - Agrega un leño y recalcula el tamaño promedio
* encender()
   - Enciende la chimenea si hay leños
* calcular_calor()
   - Calcula el calor generado
* quemar_leno()
   - Quema un leño y maneja el apagado automático
Ejemplos
Entrada:
``` Rust
crear_chimenea()
``` 
Salida:
``` Rust
Chimenea { encendida: false, cantidad_lenos: 0, tamano_promedio: 0 }
``` 
Entrada:
``` Rust
agregar_leno(chimenea, 10)
``` 
Salida:
``` Rust
Chimenea con cantidad_lenos: 1, tamano_promedio: 10
``` 
Entrada:
``` Rust
calcular_calor(chimenea encendida con 2 leños de tamaño 10)
``` 
Salida:
``` Rust
200 (2 * 10 * 10)
``` 