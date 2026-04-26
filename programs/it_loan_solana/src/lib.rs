use anchor_lang::prelude::*;

// ID del programa 
declare_id!("6oCnq4QoidfiHLCLdiUNDDq5xdwciTxTjMWpukKeznWV");

#[program]
pub mod it_loan_solana {
    use super::*;

    // 1. CREATE (PDA): Inicializa la dulcería
    pub fn inicializar_dulceria(ctx: Context<CrearDulceria>, nombre_dulceria: String) -> Result<()> {
        let gestor = &mut ctx.accounts.gestor;
        gestor.owner = ctx.accounts.owner.key();
        gestor.nombre_dulceria = nombre_dulceria;
        gestor.dulces = Vec::new();
        
        msg!("Dulcería '{}' inicializada.", gestor.nombre_dulceria);
        Ok(())
    }

    // 2. CREATE (Dato): Registra un dulce
    pub fn registrar_dulce(
        ctx: Context<GestionarDulce>, 
        dulce: String, 
        cliente: String, 
        cantidad: u8
    ) -> Result<()> {
        let gestor = &mut ctx.accounts.gestor;
        require!(gestor.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let nuevo_dulce = DulceRegistrado {
            nombre_dulce: dulce,
            vendido_a: cliente,
            cantidad: cantidad,
        };

        gestor.dulces.push(nuevo_dulce);
        msg!("Dulce registrado exitosamente.");
        Ok(())
    }

    // 3. UPDATE
    pub fn editar_dulce(
        ctx: Context<GestionarDulce>, 
        dulce: String, 
        nuevo_cliente: String, 
        nueva_cantidad: u8
    ) -> Result<()> {
        let gestor = &mut ctx.accounts.gestor;
        require!(gestor.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let lista = &mut gestor.dulces;
        for i in 0..lista.len() {
            if lista[i].nombre_dulce == dulce {
                lista[i].vendido_a = nuevo_cliente;
                lista[i].cantidad = nueva_cantidad;
                msg!("Registro de '{}' actualizado.", dulce);
                return Ok(());
            }
        }
        Err(Errores::DulceNoEncontrado.into())
    }

    // 4. DELETE
    pub fn eliminar_dulce(ctx: Context<GestionarDulce>, dulce: String) -> Result<()> {
        let gestor = &mut ctx.accounts.gestor;
        require!(gestor.owner == ctx.accounts.owner.key(), Errores::NoEresElOwner);

        let lista = &mut gestor.dulces;
        let index = lista.iter().position(|p| p.nombre_dulce == dulce);

        if let Some(i) = index {
            lista.remove(i);
            msg!("Dulce '{}' eliminado del registro.", dulce);
            Ok(())
        } else {
            Err(Errores::DulceNoEncontrado.into())
        }
    }

    // 5. READ
    pub fn ver_dulces(ctx: Context<GestionarDulce>) -> Result<()> {
        msg!("Dulcería: {}", ctx.accounts.gestor.nombre_dulceria);
        msg!("Dulces registrados: {:#?}", ctx.accounts.gestor.dulces);
        Ok(())
    }
}

// --- ESTADO DEL PROGRAMA ---

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct DulceRegistrado {
    #[max_len(30)]
    pub nombre_dulce: String,
    #[max_len(40)]
    pub vendido_a: String,
    pub cantidad: u8,
}

#[account]
#[derive(InitSpace)]
pub struct GestorDulceria {
    pub owner: Pubkey,
    #[max_len(40)]
    pub nombre_dulceria: String,
    #[max_len(10)]
    pub dulces: Vec<DulceRegistrado>,
}

// --- CONTEXTOS ---

#[derive(Accounts)]
pub struct CrearDulceria<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + GestorDulceria::INIT_SPACE,
        seeds = [b"dulceria", owner.key().as_ref()],
        bump
    )]
    pub gestor: Account<'info, GestorDulceria>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionarDulce<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub gestor: Account<'info, GestorDulceria>,
}

// --- ERRORES ---

#[error_code]
pub enum Errores {
    #[msg("No tienes permisos sobre esta dulcería.")]
    NoEresElOwner,
    #[msg("El dulce no se encuentra en el registro.")]
    DulceNoEncontrado,
}
