use anchor_lang::prelude::*;

use crate::errors::CloudWalletError;

#[account]
#[derive(Default)]
pub struct NonceData {
    pub pos: u64,
    pub(crate) nonce_bump: u8,
    pub buckets: Vec<u128>,
}

pub const NONCE_TABLE_WORDS: u64 = 880;
pub const NONCE_BUCKET: u64 = NONCE_TABLE_WORDS * 128;
pub const NONCE_DATA_SPACE: usize = 21;










impl NonceData {
    pub fn use_nonce(&mut self, nonce: u64) -> Result<()> {
        let bit = 1u128 << (nonce & 127);
        let word = usize::try_from((nonce >> 7) % NONCE_TABLE_WORDS).unwrap();
        require!(
            nonce / NONCE_BUCKET == self.pos,
            CloudWalletError::WrongNonceTableIndex
        );
        require!(
            word < self.buckets.len(),
            CloudWalletError::WithdrawIdOutOfRange
        );
        let slot = &mut self.buckets[word];
        require!(*slot & bit == 0, CloudWalletError::WithdrawIdUsed);
        *slot |= bit;
        Ok(())
    }
}
