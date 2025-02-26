use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_interface::{Mint, TokenAccount},
};

use kamino_lending_interface::{
    deposit_reserve_liquidity_invoke_signed_with_program_id,
    redeem_reserve_collateral_invoke_signed_with_program_id, DepositReserveLiquidityAccounts,
    DepositReserveLiquidityIxArgs, RedeemReserveCollateralAccounts, RedeemReserveCollateralIxArgs,
};

use crate::state::Vault;

#[derive(Accounts)]
#[instruction()]
pub struct KaminoPayments<'info> {
    #[account(mut, associated_token::mint=vault.usdc_mint, associated_token::authority=vault)]
    pub vault_usdc_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, associated_token::mint=vault.collateral_mint, associated_token::authority=vault)]
    pub vault_collateral_ata: InterfaceAccount<'info, TokenAccount>,

    // states
    #[account(mut, seeds=[b"vault", vault.authority.key().as_ref()], bump=vault.bump)]
    pub vault: Account<'info, Vault>,

    // kamino accounts
    #[account(mut)]
    /// CHECK: neglect
    pub reserve: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: neglect
    pub reserve_liquidity_mint: UncheckedAccount<'info>,
    #[account(mut)]
    pub reserve_liquidity_supply: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub reserve_collateral_mint: InterfaceAccount<'info, Mint>,
    /// CHECK: neglect
    pub lending_market: UncheckedAccount<'info>,
    /// CHECK: neglect
    pub lending_market_authority: UncheckedAccount<'info>,

    /// CHECK: Kamino program (Not an Anchor program)
    pub kamino_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: Neglect
    pub sysvar_instructions: UncheckedAccount<'info>,
}

impl<'info> KaminoPayments<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let kamino_program_id = self.token_program.key();

        let vault_seeds: &[&[&[u8]]] =
            &[&[b"vault", self.vault.authority.as_ref(), &[self.vault.bump]]];
        let args = DepositReserveLiquidityIxArgs {
            liquidity_amount: amount,
        };

        let accounts = DepositReserveLiquidityAccounts {
            owner: &self.vault.to_account_info(),
            reserve: &self.reserve.to_account_info(),
            lending_market: &self.lending_market.to_account_info(),
            lending_market_authority: &self.lending_market_authority.to_account_info(),
            reserve_liquidity_mint: &self.reserve_liquidity_mint.to_account_info(),
            reserve_liquidity_supply: &self.reserve_liquidity_supply.to_account_info(),
            reserve_collateral_mint: &self.reserve_collateral_mint.to_account_info(),
            user_source_liquidity: &self.vault_usdc_ata.to_account_info(),
            user_destination_collateral: &self.vault_collateral_ata.to_account_info(),
            collateral_token_program: &self.token_program.to_account_info(),
            liquidity_token_program: &self.token_program.to_account_info(),
            instruction_sysvar_account: &self.sysvar_instructions.to_account_info(),
        };

        deposit_reserve_liquidity_invoke_signed_with_program_id(
            kamino_program_id,
            accounts,
            args,
            vault_seeds,
        )?;

        self.vault.total_k_usdc += amount;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let kamino_program_id = self.token_program.key();

        let vault_seeds: &[&[&[u8]]] =
            &[&[b"vault", self.vault.authority.as_ref(), &[self.vault.bump]]];
        let args = RedeemReserveCollateralIxArgs {
            collateral_amount: amount,
        };

        let accounts = RedeemReserveCollateralAccounts {
            owner: &self.vault.to_account_info(),
            reserve: &self.reserve.to_account_info(),
            lending_market: &self.lending_market.to_account_info(),
            lending_market_authority: &self.lending_market_authority.to_account_info(),
            reserve_liquidity_mint: &self.reserve_liquidity_mint.to_account_info(),
            reserve_liquidity_supply: &self.reserve_liquidity_supply.to_account_info(),
            reserve_collateral_mint: &self.reserve_collateral_mint.to_account_info(),
            user_source_collateral: &self.vault_collateral_ata.to_account_info(),
            user_destination_liquidity: &self.vault_usdc_ata.to_account_info(),
            collateral_token_program: &self.token_program.to_account_info(),
            liquidity_token_program: &self.token_program.to_account_info(),
            instruction_sysvar_account: &self.sysvar_instructions.to_account_info(),
        };

        redeem_reserve_collateral_invoke_signed_with_program_id(
            kamino_program_id,
            accounts,
            args,
            vault_seeds,
        )?;

        self.vault.total_k_usdc -= amount;
        Ok(())
    }
}
