use anchor_lang::prelude::*;

use crate::errors::CloudWalletError;

pub mod close;
pub mod increase;
pub mod initialize;

pub use close::*;
pub use increase::*;
pub use initialize::*;

pub fn check_nonce_table(name: &[u8]) -> Result<()> {
    match name {
        b"big_nonce_data" => Ok(()),
        _ => Err(CloudWalletError::InvalidNonceTableType.into()),
    }
}
