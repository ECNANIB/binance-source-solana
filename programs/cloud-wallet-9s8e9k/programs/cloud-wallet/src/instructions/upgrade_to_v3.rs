use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;


#[derive(Accounts)]
pub struct UpgradeToV3<'info> {
    #[account(mut, constraint = config_data.owner == *payer.key @ CloudWalletError::SenderNotAllowed)]
    pub payer: Signer<'info>,
    #[account(mut, seeds = [b"config_data"], bump, realloc = CONFIG_DATA_SPACE_V3, realloc::payer = payer, realloc::zero = false)]
    pub config_data: Account<'info, ConfigData>,
    pub system_program: Program<'info, System>,
}














pub(crate) fn upgrade_to_v3(ctx: Context<UpgradeToV3>, signature: [u8; 64]) -> Result<()> {
    require_eq!(
        ctx.accounts.config_data.signatures.len(),
        0,
        CloudWalletError::Rejected
    );
    ctx.accounts.config_data.signatures.push(signature);
    Ok(())
}
