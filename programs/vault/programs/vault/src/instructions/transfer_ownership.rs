use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TransferOwnership<'info> {
    #[account(
        mut,
        constraint = config_data.owner == *owner.key @ crate::VaultError::NotAllowed
    )]
    pub owner: Signer<'info>,

    pub new_owner: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [crate::CONFIG_DATA_SEED],
        bump = config_data.data_bump
    )]
    pub config_data: Account<'info, ConfigData>,
}
