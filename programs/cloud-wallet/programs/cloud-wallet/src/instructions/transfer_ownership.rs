use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;

#[derive(Accounts)]
pub struct TransferOwnership<'info> {
    #[account(
        mut,
        constraint = ({
            let k = owner.key;
            config_data.owner != Pubkey::default() && config_data.owner == *k
        }) @ CloudWalletError::SenderNotAllowed
    )]
    pub owner: Signer<'info>,

    pub new_owner: AccountInfo<'info>,
    #[account(mut, seeds = [b"config_data"], bump = config_data.bump)]
    pub config_data: Account<'info, ConfigData>,
}

pub fn transfer_ownership(ctx: Context<TransferOwnership>) -> Result<()> {
    ctx.accounts.config_data.owner = *ctx.accounts.new_owner.key;
    Ok(())
}
