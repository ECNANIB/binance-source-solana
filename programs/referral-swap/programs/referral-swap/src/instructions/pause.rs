use anchor_lang::prelude::*;

use crate::error::ReferralSwapError;
use crate::state::ConfigData;

#[derive(Accounts)]
pub struct Pa021<'info> {
    #[account(mut,
        constraint = config_data.owner == *signer.key @ ReferralSwapError::NotAllowed)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"config_data"],
        bump = config_data.bump
    )]
    pub config_data: Account<'info, ConfigData>,
}

pub(crate) fn handler(ctx: Context<Pa021>) -> Result<()> {
    ctx.accounts.config_data.pau021 = true;
    Ok(())
}
