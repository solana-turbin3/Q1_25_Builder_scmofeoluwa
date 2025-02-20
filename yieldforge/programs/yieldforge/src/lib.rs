pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("7aoRwRBaXufoCZMaD3H7qdvFqXy6Grb43bEdQ6YzbHoD");

#[program]
pub mod yieldforge {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64, authority: Pubkey) -> Result<()> {
        ctx.accounts.init(seed, authority, &ctx.bumps)?;
        Ok(())
    }
}
