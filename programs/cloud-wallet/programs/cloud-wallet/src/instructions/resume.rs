use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;

#[derive(Accounts)]
pub struct Resume<'info> {
    #[account(
        mut,
        constraint = ({
            let k = signer.key;
            config_data.owner != Pubkey::default() && config_data.owner == *k
        }) @ CloudWalletError::SenderNotAllowed
    )]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [b"config_data"], bump = config_data.bump)]
    pub config_data: Account<'info, ConfigData>,
}

pub fn resume(ctx: Context<Resume>) -> Result<()> {
    ctx.accounts.config_data.paused = false;
    Ok(())
}
