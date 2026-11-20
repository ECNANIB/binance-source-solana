use anchor_lang::prelude::*;

use crate::errors::MemeshotVaultError;
use crate::state::ConfigData;
use crate::CONFIG_DATA_SPACE;



#[derive(Accounts)]
pub struct IncreaseConfigDataSpace<'info> {
    #[account(
        mut,
        constraint = config_data.owner == *payer.key @ MemeshotVaultError::SenderNotAllowed
    )]
    pub payer: Signer<'info>,
    #[account(
        mut,
        realloc = CONFIG_DATA_SPACE,
        realloc::payer = payer,
        realloc::zero = false,
        seeds = [b"config_data"],
        bump
    )]
    pub config_data: Account<'info, ConfigData>,
    pub system_program: Program<'info, System>,
}






pub(crate) fn handler(ctx: Context<IncreaseConfigDataSpace>, space: u64) -> Result<()> {

    require_eq!(
        ctx.accounts.config_data.expire_duration,
        0,
        MemeshotVaultError::NumberMismatch
    );
    ctx.accounts.config_data.expire_duration = space;
    Ok(())
}
