use anchor_lang::prelude::*;

use crate::errors::MemeshotVaultError;
use crate::state::{ConfigData, TokenConfigData};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenInterface},
};

#[derive(Accounts)]
pub struct RemoveTradeToken<'info> {
    #[account(
        mut,
        constraint = super::is_registered(&config_data.owner, signer.key) @ MemeshotVaultError::SenderNotAllowed
    )]
    pub signer: Signer<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump)]
    pub config_data: Box<Account<'info, ConfigData>>,
    #[account(
        mut,
        close = signer,
        seeds = [b"config_data", token_mint.key().as_ref()],
        bump = token_config_data.bump
    )]
    pub token_config_data: Account<'info, TokenConfigData>,
    pub token_mint: InterfaceAccount<'info, Mint>,
}

pub fn handler(_ctx: Context<RemoveTradeToken>) -> Result<()> {
    Ok(())
}
