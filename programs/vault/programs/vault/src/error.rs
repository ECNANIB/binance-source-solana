use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("nonce out of table")]
    NonceOutOfTable,
    #[msg("nonce used")]
    NonceUsed,
    #[msg("not allowed")]
    NotAllowed,
    #[msg("invalid parameter")]
    InvalidParameter,
    #[msg("invalid signature")]
    InvalidSignature,
    #[msg("exceeds limit")]
    ExceedsLimit,
    #[msg("paused")]
    Paused,
    #[msg("math overflow")]
    MathOverflow,
}
