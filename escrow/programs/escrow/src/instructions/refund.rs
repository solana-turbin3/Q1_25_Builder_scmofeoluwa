use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token::Token, token_interface::{close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TransferChecked}
};
use crate::states::*;

impl<'info> Refund<'info> {
    pub fn refund(&mut self) -> Result<()>{
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked{
            from: self.vault.to_account_info(),
            to: self.maker_mint_a_ata.to_account_info(),
            mint: self.mint_a.to_account_info(),
            authority: self.escrow.to_account_info()
        };
        let seed_bytes = self.escrow.seed.to_le_bytes();
        let seeds = &[b"escrow", self.escrow.maker.as_ref(), seed_bytes.as_ref(), &[self.escrow.bump]];
        let signer_seeds = [&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);
        transfer_checked(cpi_ctx, self.escrow.receive_amount, self.mint_a.decimals)?;
        Ok(())
    }

    pub fn close_vault(&mut self) -> Result<()>{
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = CloseAccount{
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info()
        };
        let seed_bytes = self.escrow.seed.to_le_bytes();
        let seeds = &[b"escrow", self.escrow.maker.as_ref(), seed_bytes.as_ref(), &[self.escrow.bump]];
        let signer_seeds = [&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);
        close_account(cpi_ctx)?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(seeds: u64)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(mut, associated_token::mint=mint_a, associated_token::authority=maker)]
    pub maker_mint_a_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, has_one=maker, seeds=[b"escrow", maker.key.as_ref(), seeds.to_le_bytes().as_ref()], bump=escrow.bump, close=maker)]
    pub escrow: Account<'info, Escrow>,
    #[account(mut, associated_token::mint=mint_a, associated_token::authority=escrow)]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>
}
