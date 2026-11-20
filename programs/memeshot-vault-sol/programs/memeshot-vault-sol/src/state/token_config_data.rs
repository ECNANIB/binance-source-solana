use anchor_lang::prelude::*;

#[account]
pub struct TokenConfigData {
    pub bump: u8,
    pub max_trade_amount: u64,
}
