use anchor_lang::prelude::*;

use crate::errors::MemeshotVaultError;
use crate::state::ConfigData;

#[derive(Accounts)]
pub struct Pause<'info> {
    #[account(
        mut,
        constraint = config_data.withdraw_tos.contains(signer.key) @ MemeshotVaultError::SenderNotAllowed
    )]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [b"config_data"], bump = config_data.bump)]
    pub config_data: Account<'info, ConfigData>,
}

pub fn handler(ctx: Context<Pause>) -> Result<()> {
    ctx.accounts.config_data.is_frozen = true;
    Ok(())
}
