use anchor_lang::prelude::*;

use crate::state::ConfigData;
use crate::ErrorCode;


#[derive(Accounts)]
pub struct SetPartiallyFilledRemainingPctBase<'info> {
    #[account(mut, constraint = config_data.addresses_2.contains(signer.key) @ ErrorCode::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [b"config_data"], bump)]
    pub config_data: Account<'info, ConfigData>,
    pub system_program: Program<'info, System>,
}












pub(crate) fn handler(ctx: Context<SetPartiallyFilledRemainingPctBase>, pct_base: u64) -> Result<()> {
    require_neq!(ctx.accounts.config_data.partially_filled_remaining_pct_base, pct_base);
    ctx.accounts.config_data.partially_filled_remaining_pct_base = pct_base;
    Ok(())
}
