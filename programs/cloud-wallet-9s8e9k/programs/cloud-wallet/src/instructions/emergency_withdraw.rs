use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;

#[event]
pub struct EmergencyWithdrawEvent {
    pub signer: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub to_user: Pubkey,
}

#[derive(Accounts)]
pub struct EmergencyWithdraw<'info> {
    #[account(mut, constraint = config_data.managers.contains(signer.key) @ CloudWalletError::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(mut)]

    pub token_holder: UncheckedAccount<'info>,
    #[account(constraint = crate::instructions::check_addr(to_user.key, &config_data.token_holder) @ CloudWalletError::InvalidRebalanceTo)]

    pub to_user: UncheckedAccount<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump)]
    pub config_data: Account<'info, ConfigData>,
    #[account(mut)]

    pub token_holder_token_account: UncheckedAccount<'info>,
    #[account(init_if_needed, payer = signer, associated_token::mint = token_mint, associated_token::authority = to_user)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_mint: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
pub(crate) fn emergency_withdraw(
    ctx: Context<EmergencyWithdraw>,
    token_decimals: u8,
    amount: u64,
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"token_holder",
        &[ctx.accounts.config_data.token_holder_bump],
    ]];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        anchor_spl::token_interface::TransferChecked {
            from: ctx.accounts.token_holder_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.token_holder.to_account_info(),
            mint: ctx.accounts.token_mint.to_account_info(),
        },
        signer_seeds,
    );
    anchor_spl::token_interface::transfer_checked(cpi_ctx, amount, token_decimals)?;
    emit!(EmergencyWithdrawEvent {
        signer: ctx.accounts.signer.key(),
        token_mint: ctx.accounts.token_mint.key(),
        amount,
        to_user: ctx.accounts.to_user.key(),
    });
    Ok(())
}
