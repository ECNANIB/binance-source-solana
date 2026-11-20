use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseConfig<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        close = payer,
        seeds = [crate::CONFIG_DATA_SEED],
        bump
    )]
    pub config_data: LazyAccount<'info, ConfigData>,
    pub system_program: Program<'info, System>,
}
