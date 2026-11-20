use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ConfigAddr<'info> {
    #[account(
        mut,
        constraint = config_data.owner == *signer.key @ crate::VaultError::NotAllowed
    )]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [crate::CONFIG_DATA_SEED],
        bump = config_data.data_bump
    )]
    pub config_data: Account<'info, ConfigData>,
    pub system_program: Program<'info, System>,
}
