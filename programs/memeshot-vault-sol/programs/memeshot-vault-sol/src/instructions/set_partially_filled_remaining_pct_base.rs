use anchor_lang::prelude::*;

use crate::errors::MemeshotVaultError;
use crate::state::ConfigData;






#[derive(Accounts)]
pub struct SetPartiallyFilledRemainingPctBase<'info> {
    #[account(
        mut,
        constraint = config_data.withdraw_tos.contains(signer.key) @ MemeshotVaultError::SenderNotAllowed
    )]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [b"config_data"], bump)]
    pub config_data: Account<'info, ConfigData>,
    pub system_program: Program<'info, System>,
}


pub(crate) fn handler(
    ctx: Context<SetPartiallyFilledRemainingPctBase>,
    pct_base: u64,
) -> Result<()> {
    require_neq!(ctx.accounts.config_data.expire_duration, pct_base);
    ctx.accounts.config_data.expire_duration = pct_base;
    Ok(())
}
