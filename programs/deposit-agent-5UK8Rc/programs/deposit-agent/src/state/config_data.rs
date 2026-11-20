use anchor_lang::prelude::*;

use crate::MAX_CONFIG_ADDRS;

#[account]
pub struct ConfigData {
    pub bump: u8,
    pub expiration: u64,
    pub paused: bool,
    pub owner: Pubkey,
    pub program_addr: Option<Pubkey>,
    pub secp256k1_pubkeys: Vec<[u8; 64]>,
    pub withdraw_tos: Vec<Pubkey>,
    pub operators: Vec<Pubkey>,
}
