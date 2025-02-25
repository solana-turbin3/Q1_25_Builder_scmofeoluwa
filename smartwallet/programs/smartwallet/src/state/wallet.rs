use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Wallet {
    pub seed: u64,
    pub bump: u8,
    pub owner: Pubkey,
    pub total_limit: u64,
    pub tx_limit: u64,
    pub next_reset_ts: i64,
    pub withdrawn_today: u64,
}

impl Wallet {
    pub const LEN: usize = 8 + Wallet::INIT_SPACE;
}
