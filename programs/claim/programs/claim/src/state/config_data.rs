use anchor_lang::prelude::*;

pub const CONFIG_DATA_SPACE: usize = 303;

#[account]
pub struct ConfigData {
    pub bump: u8,
    pub token_holder_bump: u8,
    pub owner: Pubkey,
    pub pause: bool,
    pub signers: Vec<[u8; 64]>,
}
