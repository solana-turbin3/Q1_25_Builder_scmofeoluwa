use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub seed: u64,
    pub bump: u8,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive_amount: u64,
}

impl Escrow {
    pub const LEN: usize = 8 + Escrow::INIT_SPACE;
}
