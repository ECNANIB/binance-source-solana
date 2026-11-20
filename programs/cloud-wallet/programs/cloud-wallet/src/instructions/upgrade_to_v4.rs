use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;


#[derive(Accounts)]
pub struct UpgradeToV4<'info> {
    #[account(
        mut,
        constraint = config_data.owner == *payer.key
            @ CloudWalletError::SenderNotAllowed
    )]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"config_data"],
        bump,
        realloc = CONFIG_DATA_SPACE_V4,
        realloc::payer = payer,
        realloc::zero = false
    )]
    pub config_data: Account<'info, ConfigData>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn upgrade_to_v4(ctx: Context<UpgradeToV4>, pubkey: [u8; 64]) -> Result<()> {
    if !ctx.accounts.config_data.withdraw_secp256k1_pubkeys.is_empty() {
        return Err(
            error!(CloudWalletError::NumberMismatch).with_values((
                ctx.accounts.config_data.withdraw_secp256k1_pubkeys.len(),
                0,
            )),
        );
    }
    ctx.accounts.config_data.withdraw_secp256k1_pubkeys = vec![pubkey];
    Ok(())
}
