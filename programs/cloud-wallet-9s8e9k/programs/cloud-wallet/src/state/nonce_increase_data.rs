use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct NonceIncreaseData {
    pub increase_count: u64,
    pub seed: u8,
}
