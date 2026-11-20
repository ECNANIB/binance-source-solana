use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;



#[derive(Accounts)]
#[instruction(args: BigNonceInitializeArgs)]
pub struct BigNonceInitialize<'info> {
    #[account(init, payer = payer, space = BIG_NONCE_DATA_SPACE, seeds = [args.nonce_table_type.as_slice(), args.count.to_le_bytes().as_ref()], bump)]
    pub nonce_data: AccountLoader<'info, BigNonceData>,
    #[account(init, payer = payer, space = NONCE_INCREASE_DATA_SPACE, seeds = [b"nonce_increase_data", nonce_data.key().to_bytes().as_ref()], bump)]
    pub nonce_increase_data: Account<'info, NonceIncreaseData>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BigNonceInitializeArgs {
    pub nonce_table_type: Vec<u8>,
    pub nonce_index: u8,
    pub count: u64,
}

pub(crate) fn big_nonce_initialize(
    ctx: Context<BigNonceInitialize>,
    args: BigNonceInitializeArgs,
) -> Result<()> {
    check_nonce_table_type(args.nonce_table_type.as_slice())?;

    ctx.accounts.nonce_increase_data.increase_count = 0;
    ctx.accounts.nonce_increase_data.seed = ctx.bumps.nonce_increase_data;
    Ok(())
}

fn check_nonce_table_type(s: &[u8]) -> Result<()> {
    if !matches!(s, b"big_nonce_data") {
        return Err(CloudWalletError::InvalidNonceTableType.into());
    }
    Ok(())
}
