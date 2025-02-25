use anchor_lang::prelude::*;

use crate::state::Wallet;
use crate::helper::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction()]
pub struct Withdraw<'info>{
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut, seeds=[b"wallet", owner.key().as_ref()], bump=wallet.bump)]
    pub wallet: Account<'info, Wallet>,
    #[account(mut)]
    pub destination: SystemAccount<'info>,
    pub system_program: Program<'info, System>
}

impl<'info> Withdraw<'info>{
    pub fn withdraw_sol(&mut self, amount: u64) -> Result<()>{
        let clock = Clock::get()?;

        if clock.unix_timestamp >= self.wallet.next_reset_ts {
            self.wallet.withdrawn_today = 0;
            self.wallet.next_reset_ts = calculate_next_reset(clock.unix_timestamp)
        }

        if self.wallet.withdrawn_today + amount > self.wallet.total_limit {
            return err!(ErrorCode::TotalLimitExceeded)
        }

        if amount > self.wallet.tx_limit {
            return err!(ErrorCode::TxLimitExceeded)
        }

        if self.wallet.get_lamports() < amount {
            return err!(ErrorCode::InsufficientFunds)
        }

        self.wallet.withdrawn_today += amount;

        self.wallet.sub_lamports(amount)?;
        self.destination.add_lamports(amount)?;
        Ok(())
    }
}
