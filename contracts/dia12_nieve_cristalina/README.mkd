# Nieve Cristalina - Difícil
## 📖 Historia
Los copos de nieve caen del cielo y cada persona puede recolectarlos. Pero solo el dueño de los copos puede transferirlos a otros. Es hora de aprender sobre Address y autenticación en Soroban!

## 🎯 Concepto: Address y Autenticación en Soroban
Lo que vas a aprender:

- Address - Representa una cuenta o contrato en Stellar
- require_auth() - Verifica que el dueño autorice la operación
- Map<Address, u32> - Asociar balances a direcciones
- Validar permisos antes de modificar datos

* ¿Por qué esto importa en blockchain?

En blockchain, la seguridad es fundamental. No podés confiar en que el usuario sea quien dice ser. require_auth() verifica criptográficamente que:

La persona realmente controla esa dirección
Firmó la transacción con su clave privada
Nadie puede actuar en nombre de otro sin autorización
Este patrón es la base de:

Tokens y NFTs: Solo el dueño puede transferir
DAOs: Solo los miembros pueden votar
DeFi: Solo el usuario puede retirar sus fondos
Identidad: Verificación descentralizada
## 💡 ¿Por qué esto importa para Soroban?
Todos los contratos reales necesitan autenticación. Sin require_auth(), cualquiera podría robar tokens, votos, o fondos. Es la primera línea de defensa en seguridad blockchain.

## ✅ Objetivos
Completá las 3 funciones para gestionar copos de nieve:

* recolectar(env: Env, mut copos: Map<Address, u32>, persona: Address, cantidad: u32) -> Map<Address, u32>
  - Verificar autenticación: persona.require_auth();
  - Obtener balance actual: let actual = copos.get(persona.clone()).unwrap_or(0);
  - Sumar cantidad: actual + cantidad
  - Actualizar map: copos.set(persona, actual + cantidad);
  - Importante: Usar .clone() en persona porque se usa dos veces

* ver_copos(_env: Env, copos: Map<Address, u32>, persona: Address) -> u32
  - Consultar cuántos copos tiene alguien
  - Retornar: copos.get(persona).unwrap_or(0)
  - No requiere autenticación (es solo lectura)

* transferir(env: Env, mut copos: Map<Address, u32>, de: Address, a: Address, cantidad: u32) -> Map<Address, u32>
  - Verificar autenticación del emisor: de.require_auth();
  - Obtener balance del emisor: let saldo_de = copos.get(de.clone()).unwrap_or(0);
  - Validar fondos suficientes: if saldo_de < cantidad { panic!("Saldo insuficiente"); }
  - Obtener balance del receptor: let saldo_a = copos.get(a.clone()).unwrap_or(0);
  - Restar del emisor: copos.set(de, saldo_de - cantidad);
  - Sumar al receptor: copos.set(a, saldo_a + cantidad);
### Ejemplos
Entrada:
``` Rust
recolectar(map![], alice, 10)
```
Salida:
``` Rust
map![alice => 10]
```
Entrada:
``` Rust
ver_copos(map![alice => 10], alice)
```
Salida:
``` Rust
10
```
Entrada:
``` Rust
transferir(map![alice => 20], alice, bob, 5)
```
Salida:
``` Rust
map![alice => 15, bob => 5]
```