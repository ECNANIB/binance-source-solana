use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer = payer, space = CONFIG_DATA_SPACE, seeds = [b"config_data"], bump)]
    pub config_data: Account<'info, ConfigData>,
    #[account(seeds = [b"token_holder"], bump)]

    pub token_holder: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn initialize(ctx: Context<Initialize>, owner: Pubkey, token_holder: Pubkey) -> Result<()> {
    ctx.accounts.config_data.owner = owner;
    ctx.accounts.config_data.token_holder = token_holder;
    ctx.accounts.config_data.bump = ctx.bumps.config_data;
    ctx.accounts.config_data.token_holder_bump = ctx.bumps.token_holder;
    Ok(())
}
