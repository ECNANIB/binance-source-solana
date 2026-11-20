use anchor_lang::prelude::*;

use crate::state::*;
use crate::ClaimError;

#[derive(Accounts)]
pub struct Pause<'info> {
    #[account(
        mut,
        constraint = config_data.owner == *signer.key @ ClaimError::NotAllowed,
    )]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"config_data"],
        bump = config_data.bump,
    )]
    pub config_data: Account<'info, ConfigData>,
}

#[derive(Accounts)]
pub struct Resume<'info> {
    #[account(
        mut,
        constraint = config_data.owner == *signer.key @ ClaimError::NotAllowed,
    )]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"config_data"],
        bump = config_data.bump,
    )]
    pub config_data: Account<'info, ConfigData>,
}

#[derive(Accounts)]
pub struct TransferOwnership<'info> {
    #[account(
        mut,
        constraint = config_data.owner == *owner.key @ ClaimError::NotAllowed,
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
