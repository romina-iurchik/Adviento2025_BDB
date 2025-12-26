# Gran Final Estelar - Difícil
## 📖 Historia
¡Es Navidad! Has completado 24 días de desafíos y ahora llega el gran final. Vamos a crear un contrato completo que integre TODO lo aprendido: un sistema de regalos navideños con destinatarios, prioridades, estado de entrega y estadísticas globales. ¡Es hora de brillar como una estrella! ⭐

## 🎯 Lo que vas a aprender: Proyecto Integrador Completo

- Combinar TODOS los conceptos aprendidos: Storage + Map + Vec + Struct + Enum + Option + Result
- Sistema CRUD completo: Crear, leer, actualizar y eliminar regalos
- Múltiples operaciones: Registrar, marcar entregado, filtrar, buscar, eliminar
- Estadísticas y agregaciones complejas: Calcular totales, pendientes, por prioridad
- Validaciones de negocio múltiples: No entregar dos veces, verificar existencia
- Arquitectura de contrato real: Como se usa en producción
- 
## 💡 ¿Por qué esto importa?
Este es un contrato de nivel producción. Integra:

- Gestión de entidades complejas con IDs únicos
- Estados y transiciones (pendiente → entregado)
- Validaciones de negocio robustas
- Consultas y reportes avanzados
- Manejo robusto de errores
- 
## ✅ Objetivos
Implementá el sistema completo de gestión de regalos navideños con 8 funciones:

* registrar_regalo() - Crea un regalo con ID único auto-incrementado
* marcar_entregado() - Marca un regalo como entregado (validando que exista y no esté ya entregado)
* obtener_regalos_pendientes() - Filtra regalos no entregados
* obtener_por_destinatario() - Busca regalos de un destinatario específico
* calcular_estadisticas() - Calcula totales, entregados, pendientes y alta prioridad
* eliminar_regalo() - Elimina un regalo del sistema
* total_por_prioridad() - Cuenta regalos por nivel de prioridad
* obtener_todos() - Retorna todos los regalos

### Ejemplos
Entrada:
``` Rust
registrar_regalo("Ana", "muneca", Prioridad::Alta)
```
Salida:
``` Rust
1 (ID del regalo registrado)
```
Entrada:
``` Rust
obtener_regalos_pendientes()
```
Salida:
``` Rust
Vec con todos los regalos donde entregado == false
```
Entrada:
``` Rust
calcular_estadisticas()
```
Salida:
``` Rust
Estadisticas { total_regalos: 4, entregados: 2, pendientes: 2, alta_prioridad: 2 }
```