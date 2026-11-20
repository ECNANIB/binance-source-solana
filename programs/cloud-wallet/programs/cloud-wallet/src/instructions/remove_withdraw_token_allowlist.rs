use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;

#[derive(Accounts)]
pub struct RemoveWithdrawTokenAllowlist<'info> {
    #[account(mut, constraint = config_data.owner == *signer.key @ CloudWalletError::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump, constraint = !config_data.paused @ CloudWalletError::Paused)]
    pub config_data: Account<'info, ConfigData>,
    #[account(mut, seeds = [b"withdraw_allowlist", token_mint.key().as_ref()], bump, close = signer)]
    pub withdraw_allowlist: Account<'info, EmptyData>,

    pub token_mint: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn remove_withdraw_token_allowlist(_ctx: Context<RemoveWithdrawTokenAllowlist>) -> Result<()> {
    Ok(())
}
