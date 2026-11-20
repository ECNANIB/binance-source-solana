use anchor_lang::prelude::*;

use crate::state::ConfigData;
use crate::DepositAgentError;
use utils::is_owner;

#[derive(Accounts)]
pub struct Pause<'info> {
    #[account(
        mut,
        constraint = config_data.operators.contains(signer.key) || is_owner(&config_data.owner, signer.key) @ DepositAgentError::SenderNotAllowed,
    )]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"config_data"],
        bump = config_data.bump,
    )]
    pub config_data: Account<'info, ConfigData>,
}
