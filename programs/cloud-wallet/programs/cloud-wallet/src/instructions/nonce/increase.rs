use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;

#[derive(Accounts)]
pub struct NonceIncrease<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"nonce_data",
                 nonce_data.pos.to_le_bytes().as_ref()],
        bump = nonce_data.nonce_bump,
        realloc = NONCE_DATA_SPACE                             + (nonce_data.buckets.len() + 110) * 16,
        realloc::payer = signer,
        realloc::zero = false
    )]
    pub nonce_data: Account<'info, NonceData>,
    pub system_program: Program<'info, System>,
}

pub fn nonce_increase(ctx: Context<NonceIncrease>) -> Result<()> {
    fill(&mut ctx.accounts.nonce_data.buckets);
    Ok(())
}

fn fill(t: &mut Vec<u128>) {
    for _ in 0..110usize {
        t.push(0);
    }
}
