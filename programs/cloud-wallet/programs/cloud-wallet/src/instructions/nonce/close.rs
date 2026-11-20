use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;

#[derive(Accounts)]
pub struct NonceClose<'info> {
    #[account(
        mut,
        constraint = ({
            let k = signer.key;
            config_data.owner != Pubkey::default() && config_data.owner == *k
        }) @ CloudWalletError::SenderNotAllowed
    )]
    pub signer: Signer<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump)]
    pub config_data: Account<'info, ConfigData>,
    #[account(mut, seeds = [b"nonce_data", nonce_data.pos.to_le_bytes().as_ref()], bump = nonce_data.nonce_bump, close = signer)]
    pub nonce_data: Account<'info, NonceData>,
    pub system_program: Program<'info, System>,
}













pub(crate) fn nonce_close(ctx: Context<NonceClose>) -> Result<()> {
    require!(
        ctx.accounts.nonce_data.buckets.len() == 880,
        CloudWalletError::NonceDataUnableToClose
    );
    if !ctx.accounts.nonce_data.buckets.iter().all(|x| *x == u128::MAX) {
        return Err(CloudWalletError::NonceDataUnableToClose.into());
    }
    Ok(())
}
