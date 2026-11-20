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
    #[account(mut, seeds = [args.nonce_table_type.as_slice(), args.count.to_le_bytes().as_ref()], bump = args.nonce_index, close = signer)]
    pub nonce_data: AccountLoader<'info, BigNonceData>,
    #[account(mut, seeds = [b"nonce_increase_data", nonce_data.key().to_bytes().as_ref()], bump = nonce_increase_data.seed, close = signer)]
    pub nonce_increase_data: Account<'info, NonceIncreaseData>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BigNonceCloseArgs {
    pub nonce_table_type: Vec<u8>,
    pub nonce_index: u8,
    pub count: u64,
}

pub(crate) fn big_nonce_close(_ctx: Context<BigNonceClose>, args: BigNonceCloseArgs) -> Result<()> {
    check_nonce_table_type(args.nonce_table_type.as_slice())?;

    msg!("NOTICE: Closing nonce account. Make sure it is no longer needed.");
    Ok(())
}

fn check_nonce_table_type(s: &[u8]) -> Result<()> {
    if !matches!(s, b"big_nonce_data") {
        return Err(CloudWalletError::InvalidNonceTableType.into());
    }
    Ok(())
}
