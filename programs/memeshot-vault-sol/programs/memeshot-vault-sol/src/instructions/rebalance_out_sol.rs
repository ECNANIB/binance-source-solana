use anchor_lang::prelude::*;

use crate::errors::MemeshotVaultError;
use crate::state::ConfigData;

#[derive(Accounts)]
pub struct RebalanceOutSol<'info> {
    #[account(
        mut,
        constraint = config_data.withdraw_tos.contains(signer.key) @ MemeshotVaultError::SenderNotAllowed
    )]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub token_holder: UncheckedAccount<'info>,
    #[account(
        mut,
        constraint = super::is_registered(to_user.key, &config_data.cloud_wallet_program)
            @ MemeshotVaultError::InvalidRebalanceTo
    )]
    pub to_user: UncheckedAccount<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump)]
    pub config_data: Account<'info, ConfigData>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct RebalanceOutSolEvent {
    pub token_holder: Pubkey,
    pub amount: u64,
    pub to_user: Pubkey,
}

pub(crate) fn handler(ctx: Context<RebalanceOutSol>, amount: u64) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"token_holder",
        &[ctx.accounts.config_data.partially_filled_remaining_pct_base],
    ]];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.token_holder.to_account_info(),
            to: ctx.accounts.to_user.to_account_info(),
        },
        signer_seeds,
    );
    anchor_lang::system_program::transfer(cpi_ctx, amount)?;
    emit!(RebalanceOutSolEvent {
        token_holder: ctx.accounts.signer.key(),
        amount,
        to_user: ctx.accounts.to_user.key(),
    });
    Ok(())
}
