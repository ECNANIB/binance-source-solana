use crate::state::ConfigData;
use crate::CONFIG_DATA_SPACE;
use anchor_lang::prelude::*;




#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = CONFIG_DATA_SPACE,
        seeds = [b"config_data"],
        bump
    )]
    pub config_data: Account<'info, ConfigData>,
    #[account(seeds = [b"token_holder"], bump)]
    pub token_holder: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn handler(
    ctx: Context<Initialize>,
    owner: Pubkey,
    agg_router: Pubkey,
    cloud_wallet_program: Pubkey,
    expire_duration: u64,
) -> Result<()> {
    ctx.accounts.config_data.owner = owner;
    ctx.accounts.config_data.agg_router = agg_router;
    ctx.accounts.config_data.rebalance_out_holder = cloud_wallet_program;
    ctx.accounts.config_data.bump = ctx.bumps.config_data;
    ctx.accounts.config_data.partially_filled_remaining_pct_base = ctx.bumps.token_holder;
    ctx.accounts.config_data.expire_duration = expire_duration;
    Ok(())
}
