use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;

#[derive(Accounts)]
pub struct AddWithdrawAllowlist<'info> {
    #[account(mut, constraint = config_data.owner == *signer.key @ CloudWalletError::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump, constraint = !config_data.paused @ CloudWalletError::Paused)]
    pub config_data: Account<'info, ConfigData>,

    pub user: UncheckedAccount<'info>,
    #[account(init_if_needed, payer = signer, space = EMPTY_DATA_SPACE, seeds = [b"withdraw_config_flag", token_mint.key().as_ref()], bump)]
    pub withdraw_config_flag: Account<'info, EmptyData>,
    #[account(init, payer = signer, space = EMPTY_DATA_SPACE, seeds = [b"withdraw_config", token_mint.key().as_ref(), user.key().as_ref()], bump)]
    pub withdraw_config: Account<'info, EmptyData>,

    pub token_mint: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn add_withdraw_allowlist(_ctx: Context<AddWithdrawAllowlist>) -> Result<()> {
    Ok(())
}
