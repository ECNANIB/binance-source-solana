


use anchor_lang::prelude::*;

use crate::state::ConfigData;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + 1 + 8 + 1 + 32 + 33 + (4 + 64 * 12) + (4 + 32 * 12) + (4 + 32 * 4),
        seeds = [b"config_data"],
        bump,
    )]
    pub config_data: Account<'info, ConfigData>,
    pub system_program: Program<'info, System>,
}
