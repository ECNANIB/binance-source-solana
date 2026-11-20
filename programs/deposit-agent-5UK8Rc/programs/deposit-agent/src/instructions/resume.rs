use anchor_lang::prelude::*;

use utils::is_owner;
use crate::state::ConfigData;
use crate::DepositAgentError;

#[derive(Accounts)]
pub struct Resume<'info> {
    #[account(
        mut,
        constraint = is_owner(&config_data.owner, signer.key) @ DepositAgentError::SenderNotAllowed,
    )]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"config_data"],
        bump = config_data.bump,
    )]
    pub config_data: Account<'info, ConfigData>,
}
