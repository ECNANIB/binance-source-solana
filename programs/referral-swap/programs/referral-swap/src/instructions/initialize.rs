use anchor_lang::prelude::*;

use crate::state::ConfigData;

pub const CONFIG_DATA_SEED: &[u8] = b"config_data";
pub const TOKEN_HOLDER_SEED: &[u8] = b"token_holder";
pub const TMP_TOKEN_HOLDER_SEED: &[u8] = b"tmp_token_holder";
#[derive(Accounts)]
pub struct Initial021<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 240,
        seeds = [b"config_data"],
        bump
    )]
    pub config_data: Account<'info, ConfigData>,

    #[account(seeds = [b"token_holder"], bump)]
    pub token_holder: UncheckedAccount<'info>,

    #[account(seeds = [b"tmp_token_holder"], bump)]
    pub tmp_token_holder: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn handler(ctx: Context<Initial021>, owner: Pubkey) -> Result<()> {
    let config_data = &mut ctx.accounts.config_data;
    config_data.owner = owner;
    config_data.bump = ctx.bumps.config_data;
    config_data.token_holder_b021 = ctx.bumps.token_holder;
    config_data.tmp_token_holder_b021 = ctx.bumps.tmp_token_holder;

    Ok(())
}
