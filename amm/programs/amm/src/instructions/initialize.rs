use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_interface::{Mint, TokenAccount},
};

impl<'info> Initialize<'info> {
    pub fn init_amm(
        &mut self,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>,
        bumps: &InitializeBumps,
    ) -> Result<()> {
        self.config.set_inner(Config {
            seed,
            fee,
            initializer: authority,
            bump: bumps.config,
            mint_x: self.mint_x.key(),
            mint_y: self.mint_y.key(),
            locked: false,
            lp_bump: bumps.mint_lp,
        });
        Ok(())
    }

    pub fn test(&mut self) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(seeds: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(init, payer=initializer, seeds=[b"lp", config.key().as_ref()], bump, mint::decimals=6, mint::authority=config)]
    pub mint_lp: InterfaceAccount<'info, Mint>,
    #[account(init, associated_token::mint=mint_x, associated_token::authority=config, payer=initializer)]
    pub vault_x: InterfaceAccount<'info, TokenAccount>,
    #[account(init, associated_token::mint=mint_y, associated_token::authority=config, payer=initializer)]
    pub vault_y: InterfaceAccount<'info, TokenAccount>,
    #[account(init, payer=initializer, space=Config::LEN, seeds=[b"config", mint_x.key().as_ref(), mint_y.key().as_ref(), seeds.to_le_bytes().as_ref()], bump)]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
