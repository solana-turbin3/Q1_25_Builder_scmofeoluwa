use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User {
    pub owner: Pubkey,
    pub usdc_deposited: u64,
    pub bump: u8,
}

impl User {
    pub const LEN: usize = 8 + User::INIT_SPACE;
}
