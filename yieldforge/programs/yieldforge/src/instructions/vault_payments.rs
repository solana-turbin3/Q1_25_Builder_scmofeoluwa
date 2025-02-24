use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_interface::{transfer_checked, Mint, TokenAccount, TransferChecked},
};

use crate::state::{User, Vault};

#[derive(Accounts)]
#[instruction(seeds: u64)]
pub struct VaultPayments<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    #[account(mut, associated_token::mint=usdc_mint, associated_token::authority=user)]
    pub user_usdc_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, associated_token::mint=usdc_mint, associated_token::authority=vault)]
    pub vault_usdc_ata: InterfaceAccount<'info, TokenAccount>,

    // states
    #[account(seeds=[b"vault"], bump=vault.bump)]
    pub vault: Account<'info, Vault>,
    #[account(init, payer=user, space=User::LEN, seeds=[b"user", user.key().as_ref()], bump)]
    pub user_state: Account<'info, User>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> VaultPayments<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.user_usdc_ata.to_account_info(),
            to: self.vault_usdc_ata.to_account_info(),
            mint: self.usdc_mint.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_ctx, amount, self.usdc_mint.decimals)?;
        self.vault.total_usdc_deposits += amount;
        self.user_state.usdc_deposited += amount;
        Ok(())
    }
}
