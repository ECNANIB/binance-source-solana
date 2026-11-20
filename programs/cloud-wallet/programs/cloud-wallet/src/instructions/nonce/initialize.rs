use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;




#[derive(Accounts)]
#[instruction(count: u64)]
pub struct NonceInitialize<'info> {
    #[account(init, payer = payer, space = NONCE_DATA_SPACE, seeds = [b"nonce_data", count.to_le_bytes().as_ref()], bump)]
    pub nonce_data: Account<'info, NonceData>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn nonce_initialize(ctx: Context<NonceInitialize>, count: u64) -> Result<()> {
    ctx.accounts.nonce_data.set_inner(NonceData {
        pos: count,
        nonce_bump: ctx.bumps.nonce_data,
        buckets: Vec::new(),
    });
    Ok(())
}
