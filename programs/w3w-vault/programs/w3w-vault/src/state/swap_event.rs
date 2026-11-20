use anchor_lang::prelude::*;






















































#[event]
pub struct SwapEvent {

    pub nonce: u64,

    pub mint_in: Pubkey,

    pub amount: u64,

    pub pubkey_opt: Option<Pubkey>,
    pub out_amount: u64,


    pub recipient: Pubkey,
}
















#[event]
pub struct RebalanceOutSolEvent {


    pub pubkey_c0: Pubkey,
    pub amount: u64,

    pub pubkey_d0: Pubkey,
}














#[event]
pub struct RebalanceOutEvent {

    pub pubkey_1a8: Pubkey,

    pub pubkey_198: Pubkey,

    pub pubkey_1b8: Pubkey,
    pub amount: u64,
}
