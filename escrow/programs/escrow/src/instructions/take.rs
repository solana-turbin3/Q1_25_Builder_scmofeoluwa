use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token::Token, token_interface::{close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TransferChecked}
};
use crate::states::*;

impl<'info> TakeOffer<'info> {
    pub fn exchange(&mut self) -> Result<()>{
        //transfer from taker to maker
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked{
            from: self.taker_mint_b_ata.to_account_info(),
            to: self.maker_mint_b_ata.to_account_info(),
            mint: self.mint_b.to_account_info(),
            authority: self.taker.to_account_info()
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_ctx, self.escrow.receive_amount, self.mint_b.decimals)?;

        //transfer from vault to taker
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked{
            from: self.vault.to_account_info(),
            to: self.taker_mint_a_ata.to_account_info(),
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
            destination: self.taker.to_account_info(),
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
pub struct TakeOffer<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)] 
    pub maker: SystemAccount<'info>,
    #[account(address=escrow.mint_a)] 
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(address=escrow.mint_b)] 
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(init_if_needed, associated_token::mint=mint_a, associated_token::authority=taker, payer=taker)]
    pub taker_mint_a_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, associated_token::mint=mint_b, associated_token::authority=taker)]
    pub taker_mint_b_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(init_if_needed, associated_token::mint=mint_b, associated_token::authority=maker, payer=maker)]
    pub maker_mint_b_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, has_one=mint_a, has_one=mint_b, seeds=[b"escrow", maker.key.as_ref(), seeds.to_le_bytes().as_ref()], bump=escrow.bump, close=taker)]
    pub escrow: Account<'info, Escrow>,
    #[account(mut, associated_token::mint=mint_a, associated_token::authority=escrow)]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>
}
