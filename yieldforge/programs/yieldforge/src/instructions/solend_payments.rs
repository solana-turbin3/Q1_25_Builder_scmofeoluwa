use anchor_lang::{
    prelude::*,
    solana_program::{instruction::Instruction, program::invoke_signed},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_interface::{Mint, TokenAccount},
};
use borsh::{BorshDeserialize, BorshSerialize};

use crate::state::Vault;

#[derive(Accounts)]
#[instruction()]
pub struct SolendPayments<'info> {
    #[account(mut, associated_token::mint=vault.usdc_mint, associated_token::authority=vault)]
    pub vault_usdc_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, associated_token::mint=vault.collateral_mint, associated_token::authority=vault)]
    pub vault_collateral_ata: InterfaceAccount<'info, TokenAccount>,

    // states
    #[account(mut, seeds=[b"vault", vault.authority.key().as_ref()], bump=vault.bump)]
    pub vault: Account<'info, Vault>,

    // solend accounts
    #[account(mut)]
    /// CHECK: neglect
    pub reserve: UncheckedAccount<'info>,
    #[account(mut)]
    pub reserve_liquidity_supply: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub reserve_collateral_mint: InterfaceAccount<'info, Mint>,
    /// CHECK: neglect
    pub lending_market: UncheckedAccount<'info>,
    /// CHECK: neglect
    pub lending_market_authority: UncheckedAccount<'info>,

    /// CHECK: Solend program (Not an Anchor program)
    pub solend_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct DepositReserveLiquidity {
    tag: u8,
    liquidity_amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct RedeemReserveCollateral {
    tag: u8,
    collateral_amount: u64,
}

impl<'info> SolendPayments<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let solend_program_id = self.solend_program.key();

        let vault_seeds: &[&[&[u8]]] =
            &[&[b"vault", self.vault.authority.as_ref(), &[self.vault.bump]]];
        let ix_data = DepositReserveLiquidity {
            tag: 4,
            liquidity_amount: amount,
        }
        .try_to_vec()?;

        let ix = Instruction {
            program_id: solend_program_id,
            accounts: vec![
                AccountMeta::new(self.vault_usdc_ata.key(), false),
                AccountMeta::new(self.vault_collateral_ata.key(), false),
                AccountMeta::new(self.reserve.key(), false),
                AccountMeta::new(self.reserve_liquidity_supply.key(), false),
                AccountMeta::new(self.reserve_collateral_mint.key(), false),
                AccountMeta::new_readonly(self.lending_market.key(), false),
                AccountMeta::new_readonly(self.lending_market_authority.key(), false),
                AccountMeta::new_readonly(self.vault.key(), true),
                AccountMeta::new_readonly(self.token_program.key(), false),
            ],
            data: ix_data,
        };

        invoke_signed(
            &ix,
            &[
                self.vault_usdc_ata.to_account_info(),
                self.vault_collateral_ata.to_account_info(),
                self.reserve.to_account_info(),
                self.reserve_liquidity_supply.to_account_info(),
                self.reserve_collateral_mint.to_account_info(),
                self.lending_market.to_account_info(),
                self.lending_market_authority.to_account_info(),
                self.vault.to_account_info(),
                self.token_program.to_account_info(),
            ],
            vault_seeds,
        )?;

        self.vault.total_c_usdc += amount;

        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let solend_program_id = self.solend_program.key();
        let vault_seeds: &[&[&[u8]]] =
            &[&[b"vault", self.vault.authority.as_ref(), &[self.vault.bump]]];
        let ix_data = RedeemReserveCollateral {
            tag: 5,
            collateral_amount: amount,
        }
        .try_to_vec()?;

        let ix = Instruction {
            program_id: solend_program_id,
            accounts: vec![
                AccountMeta::new(self.vault_collateral_ata.key(), false),
                AccountMeta::new(self.vault_usdc_ata.key(), false),
                AccountMeta::new(self.reserve.key(), false),
                AccountMeta::new(self.reserve_collateral_mint.key(), false),
                AccountMeta::new(self.reserve_liquidity_supply.key(), false),
                AccountMeta::new_readonly(self.lending_market.key(), false),
                AccountMeta::new_readonly(self.lending_market_authority.key(), false),
                AccountMeta::new_readonly(self.vault.key(), true),
                AccountMeta::new_readonly(self.token_program.key(), false),
            ],
            data: ix_data,
        };

        invoke_signed(
            &ix,
            &[
                self.vault_collateral_ata.to_account_info(),
                self.vault_usdc_ata.to_account_info(),
                self.reserve.to_account_info(),
                self.reserve_collateral_mint.to_account_info(),
                self.reserve_liquidity_supply.to_account_info(),
                self.lending_market.to_account_info(),
                self.lending_market_authority.to_account_info(),
                self.vault.to_account_info(),
                self.token_program.to_account_info(),
            ],
            vault_seeds,
        )?;

        self.vault.total_c_usdc -= amount;

        Ok(())
    }
}
