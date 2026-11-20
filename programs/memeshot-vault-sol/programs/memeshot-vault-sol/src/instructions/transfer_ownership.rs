use anchor_lang::prelude::*;

use crate::errors::MemeshotVaultError;
use crate::state::ConfigData;

#[derive(Accounts)]
pub struct TransferOwnership<'info> {
    #[account(
        mut,
        constraint = config_data.owner == *owner.key @ MemeshotVaultError::SenderNotAllowed
    )]
    pub owner: Signer<'info>,
    pub new_owner: AccountInfo<'info>,
    #[account(mut, seeds = [b"config_data"], bump = config_data.bump)]
    pub config_data: Account<'info, ConfigData>,
}

pub fn handler(ctx: Context<TransferOwnership>) -> Result<()> {
    ctx.accounts.config_data.owner = ctx.accounts.new_owner.key();
    Ok(())
}
