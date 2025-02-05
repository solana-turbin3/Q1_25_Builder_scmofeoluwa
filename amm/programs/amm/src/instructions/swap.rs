use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_interface::{transfer_checked, Mint, TokenAccount, TransferChecked},
};
use constant_product_curve::{ConstantProduct, LiquidityPair};

impl<'info> Swap<'info> {
    pub fn swap(&mut self, is_x: bool, amount: u64, min: u64) -> Result<()> {
        let mut curve = ConstantProduct::init(
            self.vault_x.amount,
            self.vault_y.amount,
            self.mint_lp.supply,
            self.config.fee,
            None,
        )
        .unwrap();
        let p = match is_x {
            true => LiquidityPair::X,
            false => LiquidityPair::Y,
        };

        let res = curve.swap(p, amount, min).unwrap();
        self.deposit_tokens(is_x, res.deposit)?;
        self.withdraw_tokens(is_x, res.deposit)?;
        Ok(())
    }

    pub fn deposit_tokens(&self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to, mint, decimals) = match is_x {
            true => (
                self.user_mint_x_ata.to_account_info(),
                self.vault_x.to_account_info(),
                self.mint_x.to_account_info(),
                self.mint_x.decimals,
            ),
            false => (
                self.user_mint_y_ata.to_account_info(),
                self.vault_y.to_account_info(),
                self.mint_y.to_account_info(),
                self.mint_y.decimals,
            ),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from,
            to,
            mint,
            authority: self.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_ctx, amount, decimals)?;
        Ok(())
    }

    pub fn withdraw_tokens(&self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to, mint, decimals) = match is_x {
            true => (
                self.vault_x.to_account_info(),
                self.user_mint_x_ata.to_account_info(),
                self.mint_x.to_account_info(),
                self.mint_x.decimals,
            ),
            false => (
                self.vault_y.to_account_info(),
                self.user_mint_y_ata.to_account_info(),
                self.mint_y.to_account_info(),
                self.mint_y.decimals,
            ),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from,
            to,
            mint,
            authority: self.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_ctx, amount, decimals)?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(seeds: u64)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(seeds=[b"lp", config.key().as_ref()], bump)]
    pub mint_lp: InterfaceAccount<'info, Mint>,
    #[account(mut, associated_token::mint=mint_x, associated_token::authority=user)]
    pub user_mint_x_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, associated_token::mint=mint_y, associated_token::authority=user)]
    pub user_mint_y_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, associated_token::mint=mint_x, associated_token::authority=config)]
    pub vault_x: InterfaceAccount<'info, TokenAccount>,
    #[account(associated_token::mint=mint_y, associated_token::authority=config)]
    pub vault_y: InterfaceAccount<'info, TokenAccount>,
    #[account(seeds=[b"config", mint_x.key().as_ref(), mint_y.key().as_ref(), seeds.to_le_bytes().as_ref()], bump)]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
