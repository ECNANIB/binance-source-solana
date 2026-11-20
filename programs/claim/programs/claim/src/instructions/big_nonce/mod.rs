use anchor_lang::prelude::*;

use crate::ClaimError;

pub mod close;
pub mod increase;
pub mod initialize;

pub use close::*;
pub use increase::*;
pub use initialize::*;

pub fn check_id(id: &[u8]) -> Result<()> {
    match id {
        b"big_nonce_data" => Ok(()),
        _ => Err(ClaimError::InvalidParameter.into()),
    }
}
