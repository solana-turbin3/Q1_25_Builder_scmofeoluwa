pub mod constants;
pub mod error;
pub mod helper;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("7JkhfrHcdUWvbeDun2mVU3KWPzNpmQhUPkL8pxKXvuUX");

#[program]
pub mod smartwallet {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, owner: Pubkey, seed: u64) -> Result<()> {
        ctx.accounts.initialize(owner, seed, &ctx.bumps)?;
        Ok(())
    }

    pub fn withdraw_sol(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw_sol(amount)?;

        Ok(())
    }
}
