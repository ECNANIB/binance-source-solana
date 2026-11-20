use anchor_lang::prelude::*;

#[error_code]
pub enum ReferralSwapError {
    #[msg("not allowed")]
    NotAllowed,
    #[msg("invalid parameter")]
    InvalidParameter,
    #[msg("paused")]
    Paused,
    #[msg("insufficient output amount")]
    InsufficientOutputAmount,
    #[msg("from token over-consumed")]
    FromTokenOverConsumed,
    #[msg("math overflow")]
    MathOverflow,
    #[msg("address must not be zero")]
    ZeroAddress,
}
