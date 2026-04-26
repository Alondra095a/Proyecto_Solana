# CRUD Dulcería en Solana ALONDRA ÁLVAREZ ZEPEDA
Programa desarrollado en Rust con el framework Anchor sobre la blockchain de Solana.
Permite gestionar el registro de dulces de una dulcería usando una cuenta PDA.

---

## ✅ inicializarDulceria — CREATE (PDA)
Crea la cuenta PDA que representa la dulcería en la blockchain.
Solo se puede ejecutar una vez por wallet y establece al creador como owner.
La cuenta guarda el nombre de la dulcería y la lista de dulces registrados.

```rust
pub fn inicializar_dulceria(ctx: Context<CrearDulceria>, nombre_dulceria: String) -> Result<()> {
    let gestor = &mut ctx.accounts.gestor;
    gestor.owner = ctx.accounts.owner.key();
    gestor.nombre_dulceria = nombre_dulceria;
    gestor.dulces = Vec::new();
    Ok(())
}
```

---

## ✅ registrarDulce — CREATE
Agrega un nuevo dulce con nombre, cliente y cantidad.
Valida que quien ejecuta la instrucción sea el owner de la dulcería.
Si no es el owner, lanza el error `NoEresElOwner`.

```rust
pub fn registrar_dulce(ctx: Context<GestionarDulce>, dulce: String, cliente: String, cantidad: u8) -> Result<()> {
    let gestor = &mut ctx.accounts.gestor;
    require!(gestor.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);
    gestor.dulces.push(DulceRegistrado { nombre_dulce: dulce, vendido_a: cliente, cantidad });
    Ok(())
}
```

---

## ✅ editarDulce — UPDATE
Actualiza el cliente y cantidad de un dulce existente.
Busca el dulce por nombre dentro de la lista y modifica sus datos.
Si el dulce no existe, lanza el error `DulceNoEncontrado`.

```rust
pub fn editar_dulce(ctx: Context<GestionarDulce>, dulce: String, nuevo_cliente: String, nueva_cantidad: u8) -> Result<()> {
    let lista = &mut ctx.accounts.gestor.dulces;
    for i in 0..lista.len() {
        if lista[i].nombre_dulce == dulce {
            lista[i].vendido_a = nuevo_cliente;
            lista[i].cantidad = nueva_cantidad;
            return Ok(());
        }
    }
    Err(Errores::DulceNoEncontrado.into())
}
```

---

## ✅ eliminarDulce — DELETE
Elimina un dulce del registro buscándolo por nombre.
Usa `iter().position()` para encontrar el índice y lo remueve del vector.
Si no se encuentra, lanza el error `DulceNoEncontrado`.

```rust
pub fn eliminar_dulce(ctx: Context<GestionarDulce>, dulce: String) -> Result<()> {
    let lista = &mut ctx.accounts.gestor.dulces;
    let index = lista.iter().position(|p| p.nombre_dulce == dulce);
    if let Some(i) = index {
        lista.remove(i);
        Ok(())
    } else {
        Err(Errores::DulceNoEncontrado.into())
    }
}
```

---

## ✅ verDulces — READ
Muestra todos los dulces registrados en la dulcería.
Imprime en consola el nombre de la dulcería y la lista completa de dulces con sus datos.

```rust
pub fn ver_dulces(ctx: Context<GestionarDulce>) -> Result<()> {
    msg!("Dulcería: {}", ctx.accounts.gestor.nombre_dulceria);
    msg!("Dulces registrados: {:#?}", ctx.accounts.gestor.dulces);
    Ok(())
}
```

---


