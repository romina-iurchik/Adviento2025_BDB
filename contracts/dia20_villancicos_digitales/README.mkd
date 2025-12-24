# Villancicos Digitales - Medio
## 📖 Historia
Ha llegado la hora de crear tu propia lista de reproducción navideña. Cada villancico tiene un título, artista y duración en segundos. ¡Necesitamos un reproductor digital que gestione la playlist perfecta!

## 🎯 Lo que vas a aprender: Playlist Management System

- Storage con Vec de structs - Guardar estructuras complejas
- Calcular duraciones totales - Iterar y sumar valores
- Buscar por criterios - Encontrar máximos/mínimos
- Filtrar por artista - Búsqueda en colecciones
- Gestionar playlist completa - Operaciones CRUD

## 💡 ¿Por qué esto importa?
Este patrón se usa en aplicaciones reales:

- Playlists de música/video
- Listas de contenido ordenadas
- Cálculos de totales (duración, precio, etc.)
- Filtros por múltiples criterios
- 
## ✅ Objetivos
Completá las funciones para gestionar una playlist de villancicos:

* agregar_villancico(env: Env, titulo: Symbol, artista: Symbol, duracion: u32)
  - Obtener el Vec del storage (o crear uno nuevo)
  - Crear el struct Villancico
  - Agregarlo al Vec con push_back
  - Guardar el Vec actualizado en storage
  - 
* obtener_playlist(env: Env) -> Vec<Villancico>
  - Obtener el Vec del storage
  - Retornar la playlist completa (o Vec vacío)
  - 
* duracion_total(env: Env) -> u32
  - Obtener la playlist del storage
  - Iterar sobre todos los villancicos
  - Sumar todas las duraciones
  - Retornar el total
  - 
* buscar_por_artista(env: Env, artista: Symbol) -> Vec<Villancico>
  - Obtener la playlist del storage
  - Crear un Vec nuevo para resultados
  - Iterar y agregar los que coincidan con el artista
  - Retornar el Vec de resultados
  - 
* villancico_mas_largo(env: Env) -> Option<Villancico>
  - Obtener la playlist del storage
  - Si está vacía, retornar None
  - Encontrar el villancico con mayor duración
  - Retornar Some(villancico)
  - 
### Ejemplos
Entrada:
``` Rust
agregar_villancico(symbol_short!("JingleB"), symbol_short!("Clasico"), 180)
```
Salida:
``` Rust
Agrega 'Jingle Bells' de 3 minutos
```
Entrada:
``` Rust
buscar_por_artista(symbol_short!("Clasico"))
```
Salida:
``` Rust
Retorna todos los villancicos clásicos
```