use anchor_lang::prelude::*;

use crate::errors::CloudWalletError;

pub mod add_withdraw_allowlist;
pub mod add_withdraw_token_allowlist;
pub mod big_nonce;
pub mod close_wsol;
pub mod config_addr;
pub mod deposit;
pub mod emergency_withdraw;
pub mod emergency_withdraw_sol;
pub mod initialize;
pub mod nonce;
pub mod pause;
pub mod quick_withdraw;
pub mod remove_withdraw_allowlist;
pub mod remove_withdraw_token_allowlist;
pub mod resume;
pub mod set_partially_filled_remaining_pct_base;
pub mod set_secp256k1_pubkey;
pub mod swap;
pub mod sync_native;
pub mod transfer_ownership;
pub mod upgrade_to_v4;
pub mod withdraw;
pub mod withdraw_v2;
pub mod user_deposit;

pub use big_nonce::*;
pub use nonce::*;
pub use add_withdraw_allowlist::*;
pub use add_withdraw_token_allowlist::*;
pub use close_wsol::*;
pub use config_addr::*;
pub use deposit::*;
pub use emergency_withdraw::*;
pub use emergency_withdraw_sol::*;
pub use initialize::*;
pub use pause::*;
pub use quick_withdraw::*;
pub use remove_withdraw_allowlist::*;
pub use remove_withdraw_token_allowlist::*;
pub use resume::*;
pub use set_partially_filled_remaining_pct_base::*;
pub use set_secp256k1_pubkey::*;
pub use swap::*;
pub use sync_native::*;
pub use transfer_ownership::*;
pub use upgrade_to_v4::*;
pub use user_deposit::*;
pub use withdraw::*;
pub use withdraw_v2::*;

pub const WSOL_MINT: Pubkey =
    anchor_lang::prelude::pubkey!("So11111111111111111111111111111111111111112");
pub const ALT_MINT: Pubkey =
    anchor_lang::prelude::pubkey!("9pan9bMn5HatX4EJdBwg9VgCa7Uz5HL8N1m5D3NdXejP");

pub const ROUTER_ALT: Pubkey =
    anchor_lang::prelude::pubkey!("HuTkmnrv4zPnArMqpbMbFhfwzTR7xfWQZHH1aQKzDKFZ");
pub const JUPITER_V6: Pubkey =
    anchor_lang::prelude::pubkey!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");

pub use crate::state::{check_addr, check_router};













pub fn check_partially_filled(expected: u64, actual: u64, pct_base: u64) -> Result<()> {
    if actual == expected {
        return Ok(());
    }
    if actual > expected {
        let diff = actual - expected;
        let remaining = actual / pct_base;
        if remaining > diff { return Ok(()); }
    }
    Err(error!(CloudWalletError::OrderPartiallyFilled).with_values((expected, actual)))
}

pub fn check_mint(mint: &Pubkey) -> Result<()> {
    match *mint {
        WSOL_MINT => Ok(()),
        ALT_MINT => Ok(()),
        _ => Err(CloudWalletError::InvalidProgram.into()),
    }
}
