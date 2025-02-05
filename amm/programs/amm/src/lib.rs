pub mod instructions;
pub mod states;

use anchor_lang::prelude::*;
pub use instructions::*;

declare_id!("6Q8auT1VMAWJ6FyUA56SXF6nBx39oizCPthC5ZDSfL6L");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.init_amm(seed, fee, authority, &ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y)?;
        Ok(())
    }

    pub fn swap(ctx: Context<Deposit>) -> Result<()> {
        Ok(())
    }
}
