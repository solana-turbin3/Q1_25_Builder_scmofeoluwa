use anchor_lang::{prelude::*, solana_program::program::{invoke_signed, invoke_unchecked}};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_interface::{Mint, TokenAccount},
};

use kamino_lending_interface::{
     deposit_reserve_liquidity_invoke_signed, deposit_reserve_liquidity_invoke_signed_with_program_id, deposit_reserve_liquidity_ix, deposit_reserve_liquidity_verify_account_keys, DepositReserveLiquidityAccounts, DepositReserveLiquidityIxArgs, DepositReserveLiquidityIxData, DepositReserveLiquidityKeys
};
// use borsh::{BorshDeserialize, BorshSerialize};

use crate::state::Vault;

#[derive(Accounts)]
#[instruction(seeds: u64)]
pub struct SolendPayments<'info> {
    #[account(mut, associated_token::mint=vault.usdc_mint, associated_token::authority=vault)]
    pub vault_usdc_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, associated_token::mint=vault.collateral_mint, associated_token::authority=vault)]
    pub vault_collateral_ata: InterfaceAccount<'info, TokenAccount>,

    // states
    #[account(mut, seeds=[b"vault"], bump=vault.bump)]
    pub vault: Account<'info, Vault>,

    // solend accounts
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

    /// CHECK: Solend program (Not an Anchor program)
    pub kamino_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: Neglect
    pub sysvar_instructions: UncheckedAccount<'info>,
}

// #[derive(BorshSerialize, BorshDeserialize)]
// pub struct DepositReserveLiquidity {
//     tag: u8,
//     liquidity_amount: u64,
// }

impl<'info> SolendPayments<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        // let cpi_program = self.token_program.to_account_info();
        // let cpi_accounts = DepositReserveLiquidityAccounts{
        //     owner: &self.vault.to_account_info(),
        //     reserve: &self.reserve.to_account_info(),
        //     lending_market: &self.lending_market.to_account_info(),
        //     lending_market_authority: &self.lending_market_authority.to_account_info(),
        //     reserve_liquidity_mint: &self.reserve_liquidity_mint.to_account_info(),
        //     reserve_liquidity_supply: &self.reserve_liquidity_supply.to_account_info(),
        //     reserve_collateral_mint: &self.reserve_collateral_mint.to_account_info(),
        //     user_source_liquidity: &self.vault_usdc_ata.to_account_info(),
        //     user_destination_collateral: &self.vault_collateral_ata.to_account_info(),
        //     collateral_token_program: &self.token_program.to_account_info(),
        //     liquidity_token_program: &self.token_program.to_account_info(),
        //     instruction_sysvar_account: &self.sysvar_instructions.to_account_info()
        // };
        //
        //
        // let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        // deposit_reserve_liquidity_invoke(cpi_ctx, amount);

        // let keys = DepositReserveLiquidityKeys {
        //     owner: self.vault.key(),
        //     reserve: self.reserve.key(),
        //     lending_market: self.lending_market.key(),
        //     lending_market_authority: self.lending_market_authority.key(),
        //     reserve_liquidity_mint: self.reserve_liquidity_mint.key(),
        //     reserve_liquidity_supply: self.reserve_liquidity_supply.key(),
        //     reserve_collateral_mint: self.reserve_collateral_mint.key(),
        //     user_source_liquidity: self.vault_usdc_ata.key(),
        //     user_destination_collateral: self.vault_collateral_ata.key(),
        //     collateral_token_program: self.token_program.key(),
        //     liquidity_token_program: self.token_program.key(),
        //     instruction_sysvar_account: self.sysvar_instructions.key(),
        // };
        //
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
        let signer_seeds: &[&[&[u8]]] = &[&[b"vault", &[self.vault.bump]]];

        deposit_reserve_liquidity_invoke_signed(accounts, args, signer_seeds)?;
        // deposit_reserve_liquidity_invoke_signed_with_program_id(self.kamino_program.key(), accounts, args, signer_seeds)?;

        self.vault.total_k_usdc += amount;

        Ok(())
    }
}
