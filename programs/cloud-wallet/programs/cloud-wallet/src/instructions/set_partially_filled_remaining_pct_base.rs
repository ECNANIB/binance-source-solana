use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;

#[derive(Accounts)]
pub struct SetPartiallyFilledRemainingPctBase<'info> {
    #[account(mut, constraint = config_data.managers.contains(signer.key) @ CloudWalletError::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [b"config_data"], bump)]
    pub config_data: Account<'info, ConfigData>,
    pub system_program: Program<'info, System>,
}








pub(crate) fn set_partially_filled_remaining_pct_base(
    ctx: Context<SetPartiallyFilledRemainingPctBase>,
    base: u64,
) -> Result<()> {
    require_neq!(
        ctx.accounts.config_data.partially_filled_remaining_pct_base,
        base
    );
    ctx.accounts.config_data.partially_filled_remaining_pct_base = base;
    Ok(())
}
