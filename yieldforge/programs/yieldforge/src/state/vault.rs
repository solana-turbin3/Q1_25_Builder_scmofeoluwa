use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub authority: Pubkey,
    pub seed: u64,
    pub bump: u8,
    pub usdc_mint: Pubkey,
    pub collateral_mint: Pubkey,
    pub total_usdc_deposits: u64,
    pub total_c_usdc: u64,
    pub total_k_usdc: u64,
}

impl Vault {
    pub const LEN: usize = 8 + Vault::INIT_SPACE;
}
