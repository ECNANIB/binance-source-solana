use anchor_lang::prelude::*;



#[error_code]
pub enum RouterError {
    #[msg("Invalid router")]
    InvalidRouter,
    #[msg("Address not exists")]
    AddrNotExists,
    #[msg("Invalid fee receiver")]
    InvalidFeeReceiver,
}
