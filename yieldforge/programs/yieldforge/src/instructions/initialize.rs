use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_interface::{Mint, TokenAccount},
};

use crate::state::Vault;

#[derive(Accounts)]
#[instruction(seeds: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer=authority, space=Vault::LEN, seeds=[b"vault"], bump)]
    pub vault: Account<'info, Vault>,
    pub usdc_mint: InterfaceAccount<'info, Mint>,
    pub collateral_mint: InterfaceAccount<'info, Mint>,
    #[account(init, payer=authority, associated_token::mint=usdc_mint, associated_token::authority=vault)]
    pub usdc_account: InterfaceAccount<'info, TokenAccount>,
    #[account(init, payer=authority, associated_token::mint=collateral_mint, associated_token::authority=vault)]
    pub collateral_account: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, seed: u64, authority: Pubkey, bumps: &InitializeBumps) -> Result<()> {
        self.vault.set_inner(Vault {
            seed,
            authority,
            bump: bumps.vault,
            usdc_mint: self.usdc_mint.key(),
            collateral_mint: self.collateral_mint.key(),
            total_usdc_deposits: 0,
            total_k_usdc: 0,
        });

        Ok(())
    }
}
