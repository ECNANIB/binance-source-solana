use anchor_lang::prelude::*;

pub const CONFIG_DATA_SPACE: usize = 2835;

#[account]
pub struct ConfigData {
    pub(crate) owner: Pubkey,
    pub(crate) pending_owner: Pubkey,
    pub(crate) token_holder: Pubkey,
    pub(crate) bump: u8,
    pub(crate) token_holder_bump: u8,
    pub paused: bool,
    pub managers: Vec<Pubkey>,
    pub withdraw_addrs: Vec<Pubkey>,
    pub(crate) chain_id: u64,
    pub partially_filled_remaining_pct_base: u64,
    pub agg_router: Pubkey,
    pub secp256k1_pubkeys: Vec<[u8; 64]>,
    pub(crate) rebalance_addrs: Vec<Pubkey>,
    pub(crate) deposit_secp256k1_pubkeys: Vec<[u8; 64]>,
    pub withdraw_secp256k1_pubkeys: Vec<[u8; 64]>,
}

pub fn check_addr(a: &Pubkey, b: &Pubkey) -> bool {
    *a != Pubkey::default() && *a == *b
}

pub fn check_router(config_data: &ConfigData, key: &Pubkey) -> bool {
    config_data.agg_router == *key || *key == crate::instructions::ROUTER_ALT || *key == crate::instructions::JUPITER_V6
}
