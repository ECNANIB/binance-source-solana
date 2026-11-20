use anchor_lang::prelude::*;

pub const CONFIG_DATA_SPACE: usize = 240;

#[account]
pub struct ConfigData {
    pub bump: u8,
    pub token_holder_b021: u8,
    pub tmp_token_holder_b021: u8,
    pub owner: Pubkey,
    pub pau021: bool,
    pub platform: Pubkey,
    pub router_program: Pubkey,
    pub send021: Vec<Pubkey>,
}
