use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BigNonceIncreaseArgs {
    pub name: Vec<u8>,
    pub index: u8,
    pub count: u64,
    pub space: u64,
}




#[derive(Accounts)]
#[instruction(args: BigNonceIncreaseArgs)]
pub struct BigNonceIncrease<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [args.name.as_slice(), args.count.to_le_bytes().as_ref()], bump = args.index, realloc = args.space as usize, realloc::payer = signer, realloc::zero = false)]
    pub nonce_data: AccountLoader<'info, BigNonceData>,
    #[account(mut, seeds = [b"nonce_increase_data", nonce_data.key().to_bytes().as_ref()], bump = nonce_increase_data.pda_bump, constraint = args.space > nonce_increase_data.version @ CloudWalletError::MathOverflow)]
    pub nonce_increase_data: Account<'info, NonceIncreaseData>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn big_nonce_increase(ctx: Context<BigNonceIncrease>, args: BigNonceIncreaseArgs) -> Result<()> {
    let nid = &mut ctx.accounts.nonce_increase_data;
    crate::instructions::big_nonce::check_nonce_table(args.name.as_slice())?;
    nid.version = args.space;
    Ok(())
}
