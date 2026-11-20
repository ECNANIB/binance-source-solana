use anchor_lang::prelude::*;

use crate::state::ConfigData;
use crate::ErrorCode;


#[derive(Accounts)]
pub struct UpgradeToV2<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, seeds = [b"config_data"], bump)]
    pub config_data: UncheckedAccount<'info>,
    #[account(seeds = [b"token_holder"], bump)]
    pub token_holder: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
