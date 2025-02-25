use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Daily total withdrawal limit exceeded")]
    TotalLimitExceeded,
    #[msg("Daily per-transaction withdrawal limit exceeded")]
    TxLimitExceeded,
    #[msg("Insufficient Funds")]
    InsufficientFunds
}
