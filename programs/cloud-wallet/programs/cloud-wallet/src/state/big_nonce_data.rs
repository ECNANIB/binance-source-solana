use anchor_lang::prelude::*;

use crate::errors::CloudWalletError;

pub const BIG_NONCE_WORDS: usize = 8959;

#[account(zero_copy)]
pub struct BigNonceData {
    pub table: [u128; BIG_NONCE_WORDS],
}
















impl BigNonceData {
    pub fn use_nonce(&mut self, nonce: u64) -> Result<()> {
        let bit = 1u128 << (nonce & 127);
        let w = ((nonce >> 7) as usize) % BIG_NONCE_WORDS;
        let cur = self.table[w];
        require!(cur & bit == 0, CloudWalletError::NonceUsed);
        self.table[w] |= bit;
        Ok(())
    }
}
