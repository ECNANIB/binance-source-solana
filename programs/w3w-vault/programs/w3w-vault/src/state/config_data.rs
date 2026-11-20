use anchor_lang::prelude::*;





#[account]
pub struct ConfigData {


















































    pub bump_b2: u8,
    pub bump_b3: u8,
    pub paused: bool,
    pub pubkey_60: Pubkey,
    pub pubkey_80: Pubkey,
    pub partially_filled_remaining_pct_base: u64,
    pub addresses: Vec<Pubkey>,
    pub addresses_1: Vec<Pubkey>,
    pub addresses_2: Vec<Pubkey>,
    pub field007: u64,
    pub secp256k1_pubkeys: Vec<[u8; 64]>,
    pub flag_b1: bool,
}
