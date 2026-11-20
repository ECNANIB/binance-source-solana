use anchor_lang::prelude::*;

use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{self, TokenAccount, TokenInterface, TransferChecked};
use crate::state::{ConfigData, RebalanceOutEvent, RebalanceOutSolEvent};
use crate::ErrorCode;
use super::is_rebalance_to;


#[derive(Accounts)]
pub struct RebalanceOut<'info> {
    #[account(mut, constraint = config_data.addresses_2.contains(signer.key) @ ErrorCode::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub token_holder: UncheckedAccount<'info>,
    #[account(constraint = is_rebalance_to(to_user.key, &config_data.pubkey_80) @ ErrorCode::InvalidRebalanceTo)]
    pub to_user: UncheckedAccount<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump_b2)]
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

#[derive(Accounts)]
pub struct RebalanceOutSol<'info> {
    #[account(mut, constraint = config_data.addresses_2.contains(signer.key) @ ErrorCode::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub token_holder: UncheckedAccount<'info>,
    #[account(mut, constraint = is_rebalance_to(to_user.key, &config_data.pubkey_80) @ ErrorCode::InvalidRebalanceTo)]
    pub to_user: UncheckedAccount<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump_b2)]
    pub config_data: Account<'info, ConfigData>,
    pub system_program: Program<'info, System>,
}












pub(crate) fn rebalance_out_handler(ctx: Context<RebalanceOut>, decimals: u8, amount: u64) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[b"token_holder", &[ctx.accounts.config_data.bump_b3]]];

















    let cpi_xfer = CpiContext {
        program: ctx.accounts.token_program.to_account_info(),
        accounts: TransferChecked {
            from: ctx.accounts.token_holder_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.token_holder.to_account_info(),
            mint: ctx.accounts.token_mint.to_account_info(),
        },
        signer_seeds,
        remaining_accounts: Vec::new(),
    };
    token_interface::transfer_checked(
        cpi_xfer, amount, decimals)?;

    emit!(RebalanceOutEvent {
        pubkey_1a8: ctx.accounts.to_user.key(),
        pubkey_198: ctx.accounts.signer.key(),
        pubkey_1b8: ctx.accounts.token_mint.key(),
        amount,
    });
    Ok(())
}






pub(crate) fn rebalance_out_sol_handler(ctx: Context<RebalanceOutSol>, amount: u64) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[b"token_holder", &[ctx.accounts.config_data.bump_b3]]];











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
        pubkey_c0: ctx.accounts.signer.key(),
        amount,
        pubkey_d0: ctx.accounts.to_user.key(),
    });
    Ok(())
}
