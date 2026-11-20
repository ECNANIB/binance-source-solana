use anchor_lang::prelude::*;

#[error_code]
pub enum MemeshotVaultError {
    #[msg("invalid config address type")]
    InvalidConfigAddrType,
    #[msg("config address extends limit")]
    ConfigAddrsExtendsLimit,
    #[msg("address already exists")]
    AddrExists,
    #[msg("address does not exist")]
    AddrNotExists,
    #[msg("sender is now allowed")]
    SenderNotAllowed,
    #[msg("program is invalid")]
    InvalidProgram,
    #[msg("transaction expired")]
    Expired,
    #[msg("number mismatch")]
    NumberMismatch,
    #[msg("return amount not enough")]
    ReturnAmountNotEnough,
    #[msg("paused")]
    Paused,
    #[msg("invalid rebalance out holder")]
    InvalidRebalanceTo,
    #[msg("invalid balance snapshot")]
    SnapshotInvalid,
    #[msg("math overflow")]
    MathOverflow,
    #[msg("trade amount exceeds limit")]
    TradeAmountExceedsLimit,
    #[msg("order partially filled")]
    OrderPartiallyFilled,
}
