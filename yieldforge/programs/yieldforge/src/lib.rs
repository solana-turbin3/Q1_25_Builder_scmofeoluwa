pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;

declare_id!("7fWbkEtHaPg4csLZkmR8Kt2zcfx92YBbmoL1rEDDnnDC");

#[program]
pub mod yieldforge {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64, authority: Pubkey) -> Result<()> {
        ctx.accounts.init(seed, authority, &ctx.bumps)?;
        Ok(())
    }

    pub fn deposit_into_vault(ctx: Context<VaultPayments>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn deposit_into_kamino(ctx: Context<SolendPayments>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }
}
