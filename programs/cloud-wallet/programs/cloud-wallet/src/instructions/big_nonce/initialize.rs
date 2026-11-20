use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;



#[derive(Accounts)]
#[instruction(args: BigNonceInitializeArgs)]
pub struct BigNonceInitialize<'info> {
    #[account(init, payer = payer, space = BIG_NONCE_DATA_SPACE, seeds = [args.name.as_slice(), args.count.to_le_bytes().as_ref()], bump)]
    pub nonce_data: AccountLoader<'info, BigNonceData>,
    #[account(init, payer = payer, space = NONCE_INCREASE_DATA_SPACE + 8, seeds = [b"nonce_increase_data", nonce_data.key().to_bytes().as_ref()], bump)]
    pub nonce_increase_data: Account<'info, NonceIncreaseData>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BigNonceInitializeArgs {
    pub name: Vec<u8>,
    pub index: u8,
    pub count: u64,
}

pub(crate) fn big_nonce_initialize(ctx: Context<BigNonceInitialize>, args: BigNonceInitializeArgs) -> Result<()> {
    check_name(args.name.as_slice())?;

    ctx.accounts.nonce_increase_data.version = 0;
    ctx.accounts.nonce_increase_data.pda_bump = ctx.bumps.nonce_increase_data;
    Ok(())
}

fn check_name(s: &[u8]) -> Result<()> {
    if !matches!(s, b"big_nonce_data") {
        return Err(CloudWalletError::InvalidNonceTableType.into());
    }
    Ok(())
}
