use crate::state::ConfigData;
use crate::DepositAgentError;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Pause<'info> {
    #[account(mut, constraint = config_data.withdraw_tos.contains(signer.key) @ DepositAgentError::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [b"config_data"], bump = config_data.bump)]
    pub config_data: Account<'info, ConfigData>,
}

pub fn handler(ctx: Context<Pause>) -> Result<()> {
    ctx.accounts.config_data.paused = true;
    Ok(())
}
