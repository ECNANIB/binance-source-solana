use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;

#[derive(Accounts)]
#[instruction(args: BigNonceCloseArgs)]
pub struct BigNonceClose<'info> {
    #[account(mut, constraint = config_data.owner == *signer.key @ CloudWalletError::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump)]
    pub config_data: Account<'info, ConfigData>,
    #[account(mut, seeds = [args.name.as_slice(), args.count.to_le_bytes().as_ref()], bump = args.index, close = signer)]
    pub nonce_data: AccountLoader<'info, BigNonceData>,
    #[account(mut, seeds = [b"nonce_increase_data", nonce_data.key().to_bytes().as_ref()], bump = nonce_increase_data.pda_bump, close = signer)]
    pub nonce_increase_data: Account<'info, NonceIncreaseData>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn big_nonce_close(ctx: Context<BigNonceClose>, args: BigNonceCloseArgs) -> Result<()> {
    crate::instructions::big_nonce::check_nonce_table(args.name.as_slice())?;
    msg!("NOTICE: Closing nonce account. Make sure it is no longer needed.");
    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BigNonceCloseArgs {
    pub name: Vec<u8>,
    pub index: u8,
    pub count: u64,
}
