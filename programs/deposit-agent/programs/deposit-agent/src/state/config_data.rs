use anchor_lang::prelude::*;

pub const CONFIG_DATA_SEED: &[u8] = b"config_data";

#[account]
pub struct ConfigData {
    pub bump: u8,
    pub expire_duration: u64,
    pub paused: bool,
    pub owner: Pubkey,
    pub deposit_addr: Option<Pubkey>,
    pub signer_pubkeys: Vec<[u8; 64]>,
    pub senders: Vec<Pubkey>,
    pub withdraw_tos: Vec<Pubkey>,
}

impl ConfigData {
    pub const MAX_PUBKEYS: usize = 11;
    pub const MAX_SENDERS: usize = 16;
    pub const MAX_WITHDRAW_TOS: usize = 4;
    pub const SPACE: usize = 8
        + 1
        + 8
        + 1
        + 32
        + 33
        + (4 + 64 * 10)
        + (4 + 32 * Self::MAX_SENDERS)
        + (4 + 32 * Self::MAX_WITHDRAW_TOS);
}
