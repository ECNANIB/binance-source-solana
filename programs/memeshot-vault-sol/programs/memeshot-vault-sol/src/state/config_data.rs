use anchor_lang::prelude::*;

#[account]
pub struct ConfigData {
    pub bump: u8,
    pub partially_filled_remaining_pct_base: u8,
    pub is_frozen: bool,
    pub owner: Pubkey,
    pub agg_router: Pubkey,
    pub cloud_wallet_program: Pubkey,
    pub senders: Vec<Pubkey>,
    pub withdraw_tos: Vec<Pubkey>,
    pub rebalance_out_holder: Pubkey,
    pub expire_duration: u64,
}
