use anchor_lang::prelude::*;

use crate::state::*;
use crate::ClaimError;



#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = CONFIG_DATA_SPACE,
        seeds = [b"config_data"],
        bump,
    )]
    pub config_data: Account<'info, ConfigData>,
    #[account(
        seeds = [b"token_holder"],
        bump,
    )]
    pub token_holder: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn handler(ctx: Context<Initialize>, owner: Pubkey) -> Result<()> {
    let config_data = &mut ctx.accounts.config_data;
    config_data.owner = owner;
    config_data.bump = ctx.bumps.config_data;
    config_data.token_holder_bump = ctx.bumps.token_holder;
    Ok(())
}
