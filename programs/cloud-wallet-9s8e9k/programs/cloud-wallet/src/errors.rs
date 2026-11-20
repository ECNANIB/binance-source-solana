use anchor_lang::prelude::*;

#[error_code]
pub enum CloudWalletError {
    #[msg("withdraw id used")]
    WithdrawIdUsed,
    #[msg("withdraw id out of range")]
    WithdrawIdOutOfRange,
    #[msg("wrong nonce table index")]
    WrongNonceTableIndex,
    #[msg("nonce data unable to close")]
    NonceDataUnableToClose,
    #[msg("paused")]
    Paused,
    #[msg("math overflow")]
    MathOverflow,
    #[msg("sender not allowed")]
    SenderNotAllowed,
    #[msg("transaction expired")]
    Expired,
    #[msg("invalid signature")]
    InvalidSignature,
    #[msg("invalid config addr type")]
    InvalidConfigAddrType,
    #[msg("addr exists")]
    AddrExists,
    #[msg("addr not exists")]
    AddrNotExists,
    #[msg("config addrs extends limit")]
    ConfigAddrsExtendsLimit,
    #[msg("invalid rebalance out holder")]
    InvalidRebalanceTo,
    #[msg("invalid program")]
    InvalidProgram,
    #[msg("number mismatch")]
    NumberMismatch,
    #[msg("nonce out of table")]
    NonceOutOfTable,
    #[msg("nonce used")]
    NonceUsed,
    #[msg("rejected")]
    Rejected,
    #[msg("invalid nonce table type")]
    InvalidNonceTableType,
    #[msg("order partially filled")]
    OrderPartiallyFilled,
    #[msg("return amount not enough")]
    ReturnAmountNotEnough,
}
