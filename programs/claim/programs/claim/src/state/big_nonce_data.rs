use anchor_lang::prelude::*;

use crate::ClaimError;







#[account(zero_copy)]
pub struct BigNonceData {
    pub table: [u128; 8959],
}

impl BigNonceData {

    pub fn word_of(claim_id: u64) -> usize {
        ((claim_id >> 7) % 8959) as usize
    }


    pub fn bit_of(claim_id: u64) -> u64 {
        claim_id & 127
    }


    pub fn use_claim_id(&mut self, claim_id: u64) -> Result<()> {
        let word = Self::word_of(claim_id);
        let bit = Self::bit_of(claim_id);
        if self.table[word] & (1u128 << bit) != 0 {
            return Err(error!(ClaimError::NonceUsed));
        }
        self.table[word] |= 1u128 << bit;
        Ok(())
    }
}
