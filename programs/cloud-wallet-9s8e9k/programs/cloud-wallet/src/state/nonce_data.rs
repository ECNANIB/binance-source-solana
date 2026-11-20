use anchor_lang::prelude::*;

use crate::errors::CloudWalletError;

#[account]
#[derive(Default)]
pub struct NonceData {
    pub index: u64,
    pub(crate) seed: u8,
    pub table_data: Vec<u128>,
}

pub const NONCE_TABLE_WORDS: u64 = 880;
pub const NONCE_PAGE: u64 = NONCE_TABLE_WORDS * 128;











impl NonceData {
    pub fn use_nonce(&mut self, nonce: u64) -> Result<()> {
        let bit = 1u128 << (nonce & 127);
        let word = usize::try_from((nonce >> 7) % NONCE_TABLE_WORDS).unwrap();
        require!(
            nonce / NONCE_PAGE == self.index,
            CloudWalletError::WrongNonceTableIndex
        );
        require!(
            word < self.table_data.len(),
            CloudWalletError::WithdrawIdOutOfRange
        );
        let slot = &mut self.table_data[word];
        require!(*slot & bit == 0, CloudWalletError::WithdrawIdUsed);
        *slot |= bit;
        Ok(())
    }
}
