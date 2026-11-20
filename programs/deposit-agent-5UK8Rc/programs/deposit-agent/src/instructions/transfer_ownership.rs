use anchor_lang::prelude::*;

use crate::state::ConfigData;
use crate::DepositAgentError;

#[derive(Accounts)]
pub struct TransferOwnership<'info> {
    #[account(
        mut,
        constraint = config_data.owner == *owner.key @ DepositAgentError::SenderNotAllowed,
    )]
    pub owner: Signer<'info>,

    pub new_owner: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"config_data"],
        bump = config_data.bump,
    )]
    pub config_data: Account<'info, ConfigData>,
}
