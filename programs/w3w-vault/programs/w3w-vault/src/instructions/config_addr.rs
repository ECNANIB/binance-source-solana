use anchor_lang::prelude::*;

use crate::state::ConfigData;
use crate::ErrorCode;


#[derive(Accounts)]
pub struct ConfigAddr<'info> {
    #[account(mut, constraint = config_data.addresses.contains(signer.key) @ ErrorCode::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [b"config_data"], bump = config_data.bump_b2)]
    pub config_data: Account<'info, ConfigData>,
    pub system_program: Program<'info, System>,
}
