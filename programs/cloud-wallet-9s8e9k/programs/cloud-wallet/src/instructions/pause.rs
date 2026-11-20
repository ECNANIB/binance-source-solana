use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;

#[derive(Accounts)]
pub struct Pause<'info> {
    #[account(mut, constraint = config_data.managers.contains(signer.key) @ CloudWalletError::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [b"config_data"], bump = config_data.bump)]
    pub config_data: Account<'info, ConfigData>,
}
pub(crate) fn pause(ctx: Context<Pause>) -> Result<()> {
    ctx.accounts.config_data.paused = true;
    Ok(())
}
