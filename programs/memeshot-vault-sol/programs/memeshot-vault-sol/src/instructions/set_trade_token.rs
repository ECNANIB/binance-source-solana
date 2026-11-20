use anchor_lang::prelude::*;

use crate::errors::MemeshotVaultError;
use crate::state::{ConfigData, TokenConfigData};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenInterface},
};

#[derive(Accounts)]
pub struct SetTradeToken<'info> {
    #[account(
        mut,
        constraint = super::is_registered(&config_data.owner, &signer.key()) @ MemeshotVaultError::SenderNotAllowed
    )]
    pub signer: Signer<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump)]
    pub config_data: Box<Account<'info, ConfigData>>,
    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + 1 + 8,
        seeds = [b"config_data", token_mint.key().as_ref()],
        bump
    )]
    pub token_config_data: Account<'info, TokenConfigData>,
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SetTradeToken>, trade_amount_limit: u64) -> Result<()> {
    let token_config_data = &mut ctx.accounts.token_config_data;
    token_config_data.bump = ctx.bumps.token_config_data;
    token_config_data.max_trade_amount = trade_amount_limit;
    Ok(())
}
