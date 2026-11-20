use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;

#[event]
pub struct EmergencyWithdrawSolEvent {
    pub signer: Pubkey,
    pub amount: u64,
    pub to_user: Pubkey,
}

#[derive(Accounts)]
pub struct EmergencyWithdrawSol<'info> {
    #[account(mut, constraint = config_data.managers.contains(signer.key) @ CloudWalletError::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(mut)]

    pub token_holder: UncheckedAccount<'info>,
    #[account(mut, constraint = crate::instructions::check_addr(to_user.key, &config_data.token_holder) @ CloudWalletError::InvalidRebalanceTo)]

    pub to_user: UncheckedAccount<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump)]
    pub config_data: Account<'info, ConfigData>,
    pub system_program: Program<'info, System>,
}
pub(crate) fn emergency_withdraw_sol(ctx: Context<EmergencyWithdrawSol>, amount: u64) -> Result<()> {

    let signer_seeds: &[&[&[u8]]] = &[&[b"token_holder", &[ctx.accounts.config_data.token_holder_bump]]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.token_holder.to_account_info(),
            to: ctx.accounts.to_user.to_account_info(),
        },
        signer_seeds,
    );
    anchor_lang::system_program::transfer(cpi_ctx, amount)?;
    emit!(EmergencyWithdrawSolEvent {
        to_user: ctx.accounts.to_user.key(),
        signer: ctx.accounts.signer.key(),
        amount,
    });
    Ok(())
}
