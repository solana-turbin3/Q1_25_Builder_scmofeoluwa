use anchor_lang::prelude::*;

use crate::state::Wallet;
use crate::helper::*;

#[derive(Accounts)]
#[instruction()]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init, payer=owner, space=Wallet::LEN, seeds=[b"wallet", owner.key().as_ref()], bump)]
    pub wallet: Account<'info, Wallet>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, owner: Pubkey, seed: u64, bumps: &InitializeBumps) -> Result<()>{
        let clock = Clock::get()?;
        self.wallet.set_inner(Wallet {
            seed,
            bump: bumps.wallet,
            owner,
            total_limit: 50_000_000_000,
            tx_limit: 2_000_000_000,
            next_reset_ts: calculate_next_reset(clock.unix_timestamp),
            withdrawn_today: 0,
        });
        Ok(())
    }
}
