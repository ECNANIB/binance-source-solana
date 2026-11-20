use anchor_lang::prelude::*;



#[account]
pub struct ConfigData {
    pub data_bump: u8,
    pub token_holder_bump: u8,
    pub tmp_bump: u8,
    pub owner: Pubkey,
    pub paused: bool,
    pub platform: Pubkey,
    pub split_contract: Pubkey,
    pub addrs: Vec<Pubkey>,
}
