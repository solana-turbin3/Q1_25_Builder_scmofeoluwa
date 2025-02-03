pub mod states;
pub mod instructions;

use anchor_lang::prelude::*;
pub use instructions::*;

declare_id!("FL2jkurMwrKKY6iB56WoYLWxgW8JMXL7hLuhii3CEkQr");


#[program]
pub mod escrow {

    use super::*;

    pub fn make(ctx: Context<MakeOffer>, seed: u64, receive_amount: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive_amount, &ctx.bumps)?;
        ctx.accounts.deposit(receive_amount)?;
        Ok(())
    }

    pub fn take(ctx: Context<TakeOffer>) -> Result<()> {
        ctx.accounts.exchange()?;
        ctx.accounts.close_vault()?;
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund()?;
        ctx.accounts.close_vault()?;
        Ok(())
    }
}
