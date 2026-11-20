use anchor_lang::prelude::*;
#[error_code]
pub enum DepositAgentError {
    #[msg("paused")]
    Paused,
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
    #[msg("invalid withdraw to")]
    InvalidWithdrawTo,
    #[msg("invalid program")]
    InvalidProgram,
    #[msg("rejected")]
    Rejected,
}
