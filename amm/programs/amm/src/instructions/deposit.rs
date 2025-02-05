use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_interface::{mint_to, transfer_checked, Mint, MintTo, TokenAccount, TransferChecked},
};
use constant_product_curve::ConstantProduct;

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        let (x, y) = match self.mint_lp.supply == 0
            && self.vault_x.amount == 0
            && self.vault_y.amount == 0
        {
            true => (max_x, max_y),
            false => {
                let amounts = ConstantProduct::xy_deposit_amounts_from_l(
                    self.vault_x.amount,
                    self.vault_y.amount,
                    self.mint_lp.supply,
                    amount,
                    6,
                )
                .unwrap();
                (amounts.x, amounts.y)
            }
        };
        self.deposit_tokens(true, x)?;
        self.deposit_tokens(false, y)?;
        self.mint_lp(amount)?;
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

    pub fn mint_lp(&self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.mint_lp.to_account_info(),
            to: self.user_mint_lp_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };
        let seed_bytes = self.config.seed.to_le_bytes();
        let mint_x = self.mint_x.key();
        let mint_y = self.mint_y.key();
        let seeds = &[
            b"config",
            mint_x.as_ref(),
            mint_y.as_ref(),
            seed_bytes.as_ref(),
            &[self.config.bump],
        ];
        let signer_seeds = [&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);
        mint_to(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(seeds: u64)]
pub struct Deposit<'info> {
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
    #[account(init_if_needed, associated_token::mint=mint_lp, associated_token::authority=user, payer=user)]
    pub user_mint_lp_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, associated_token::mint=mint_x, associated_token::authority=config)]
    pub vault_x: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, associated_token::mint=mint_y, associated_token::authority=config)]
    pub vault_y: InterfaceAccount<'info, TokenAccount>,
    #[account(seeds=[b"config", mint_x.key().as_ref(), mint_y.key().as_ref(), seeds.to_le_bytes().as_ref()], bump)]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
