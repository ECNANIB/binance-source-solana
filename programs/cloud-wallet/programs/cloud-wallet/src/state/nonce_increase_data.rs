use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct NonceIncreaseData {
    pub(crate) version: u64,
    pub pda_bump: u8,
}
