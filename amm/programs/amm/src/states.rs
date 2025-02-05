use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub initializer: Option<Pubkey>,
    pub seed: u64,
    pub bump: u8,
    pub fee: u16,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub locked: bool,
    pub lp_bump: u8,
}

impl Config {
    pub const LEN: usize = 8 + Config::INIT_SPACE;
}
