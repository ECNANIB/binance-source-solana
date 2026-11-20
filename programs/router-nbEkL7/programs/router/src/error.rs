use anchor_lang::prelude::*;
#[error_code]
pub enum RouterError {
    #[msg("Invalid router")]
    InvalidRouter,
    #[msg("Address not exists")]
    AddrNotExists,
    #[msg("Invalid fee receiver")]
    InvalidFeeReceiver,
    #[msg("Invalid fee")]
    InvalidFee,
    #[msg("Invalid fee token account")]
    InvalidFeeTokenAccount,
    #[msg("Invalid original amount")]
    InvalidOriginalAmt,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Missing required account for the selected mode")]
    MissingRequiredAccount,
    #[msg("Unexpected account supplied for the selected mode")]
    UnexpectedAccount,
    #[msg("Outer proxy instruction data is too short")]
    ProxyDataTooShort,
    #[msg("Outer proxy instruction must be a V1 positive-slippage swap or its order_id (V3) version; V2/V4 are unsupported")]
    UnsupportedProxyInstruction,
    #[msg("Failed to decode outer proxy instruction data")]
    ProxyDecodeFailed,
    #[msg("Nothing to swap after reserving keep amount")]
    NothingToSwap,
    #[msg("Dynamic swap rewrite failed")]
    DynamicRewriteFailed,
    #[msg("Router spent more than the dynamic budget (keep amount breached)")]
    DynamicSpendExceeded,
    #[msg("Invalid commission rate")]
    InvalidCommissionRate,
    #[msg("Invalid commission account")]
    InvalidCommissionAccount,
    #[msg("Delta windows must be opened by a top-level instruction")]
    NotTopLevelInstruction,
    #[msg("No matching settle instruction follows this begin")]
    MissingSettleInstruction,
    #[msg("Instructions sysvar data is malformed")]
    MalformedInstructionsSysvar,
    #[msg("Watched token account is not owned by the authority")]
    WatchedAuthorityMismatch,
    #[msg("Watched lamport account must be a system-owned wallet")]
    WatchedNotSystemOwned,
    #[msg("Signer is not the authority recorded in the snapshot")]
    SnapshotAuthorityMismatch,
    #[msg("Mint does not match the snapshot")]
    SnapshotMintMismatch,
    #[msg("Token program does not own the watched account or mint")]
    TokenProgramMismatch,
    #[msg("Recipient is not a valid destination for this window")]
    InvalidDeltaRecipient,
    #[msg("Recipient must differ from the watched account")]
    SelfForward,
    #[msg("Watched balance decreased during the window")]
    BalanceDecreased,
    #[msg("Watched balance did not increase during the window")]
    NothingToForward,
    #[msg("Delta is below the requested minimum")]
    DeltaBelowMinimum,
    #[msg("Recipient would be left below the rent-exempt minimum")]
    RecipientBelowRentExempt,
    #[msg("A watched account that does not exist yet must be the authority's associated token account")]
    WatchedNotAssociatedTokenAccount,
    #[msg("Watched token account is not of the given mint")]
    WatchedMintMismatch,
}
