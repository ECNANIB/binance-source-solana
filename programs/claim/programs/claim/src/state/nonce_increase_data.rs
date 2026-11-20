use anchor_lang::prelude::*;

#[account]
pub struct NonceIncreaseData {
    pub count: u64,
    pub bump: u8,
}
