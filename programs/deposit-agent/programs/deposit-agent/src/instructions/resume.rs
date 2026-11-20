use crate::state::ConfigData;
use crate::DepositAgentError;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Resume<'info> {
    #[account(mut, constraint = config_data.owner == *signer.key @ DepositAgentError::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [b"config_data"], bump = config_data.bump)]
    pub config_data: Account<'info, ConfigData>,
}

pub fn handler(ctx: Context<Resume>) -> Result<()> {
    ctx.accounts.config_data.paused = false;
    Ok(())
}
