use anchor_lang::prelude::*;

pub const CONFIG_DATA_SPACE: usize = 8 + 299;

#[account]
pub struct ConfigData {
    pub(crate) owner: Pubkey,
    pub(crate) pending_owner: Pubkey,
    pub token_holder: Pubkey,
    pub bump: u8,
    pub token_holder_bump: u8,
    pub paused: bool,
    pub managers: Vec<Pubkey>,
    pub withdraw_addrs: Vec<Pubkey>,
    pub chain_id: u64,
    pub(crate) nonce: u64,
    pub agg_router: Pubkey,
    pub secp256k1_pubkeys: Vec<[u8; 64]>,
    pub(crate) rebalance_addrs: Vec<Pubkey>,
    pub signatures: Vec<[u8; 64]>,
}

pub fn check_addr(a: &Pubkey, b: &Pubkey) -> bool {
    *a != Pubkey::default() && *a == *b
}

pub fn check_router(config: &ConfigData, k: &Pubkey) -> bool {
    config.agg_router == *k
        || crate::instructions::ROUTER_1 == *k
        || crate::instructions::ROUTER_2 == *k
}
