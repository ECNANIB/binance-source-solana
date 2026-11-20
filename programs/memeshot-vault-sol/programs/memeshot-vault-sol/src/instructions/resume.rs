use anchor_lang::prelude::*;

use crate::errors::MemeshotVaultError;
use crate::state::ConfigData;

#[derive(Accounts)]
pub struct Resume<'info> {
    #[account(
        mut,
        constraint = super::is_registered(&config_data.owner, signer.key) @ MemeshotVaultError::SenderNotAllowed
    )]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [b"config_data"], bump = config_data.bump)]
    pub config_data: Account<'info, ConfigData>,
}

pub fn handler(ctx: Context<Resume>) -> Result<()> {
    ctx.accounts.config_data.is_frozen = false;
    Ok(())
}
