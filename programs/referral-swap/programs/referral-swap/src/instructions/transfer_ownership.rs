use anchor_lang::prelude::*;

use crate::error::ReferralSwapError;
use crate::state::ConfigData;

#[derive(Accounts)]
pub struct TransferOwners021<'info> {
    #[account(mut,
        constraint = config_data.owner == *owner.key @ ReferralSwapError::NotAllowed)]
    pub owner: Signer<'info>,

    pub new_owner: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"config_data"],
        bump = config_data.bump
    )]
    pub config_data: Account<'info, ConfigData>,
}





pub(crate) fn handler(ctx: Context<TransferOwners021>) -> Result<()> {
    let new_owner = ctx.accounts.new_owner.key;
    require!(*new_owner != Pubkey::default(), ReferralSwapError::ZeroAddress);
    ctx.accounts.config_data.owner = *new_owner;
    Ok(())
}
