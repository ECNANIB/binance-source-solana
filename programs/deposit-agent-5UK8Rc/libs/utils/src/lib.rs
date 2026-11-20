use anchor_lang::prelude::*;

pub fn is_owner(owner: &Pubkey, key: &Pubkey) -> bool {
    *owner != Pubkey::default() && owner == key
}

pub fn is_wsol_mint(key: &Pubkey) -> bool {
    const WSOL_MINT: Pubkey =
        anchor_lang::solana_program::pubkey!("So11111111111111111111111111111111111111112");
    const ALT_WSOL_MINT: Pubkey =
        anchor_lang::solana_program::pubkey!("9pan9bMn5HatX4EJdBwg9VgCa7Uz5HL8N1m5D3NdXejP");
    *key == WSOL_MINT || *key == ALT_WSOL_MINT
}
