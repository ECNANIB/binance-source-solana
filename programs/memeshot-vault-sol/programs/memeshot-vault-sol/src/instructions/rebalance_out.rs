use anchor_lang::prelude::*;

use crate::errors::MemeshotVaultError;
use crate::state::ConfigData;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, TokenAccount, TokenInterface, TransferChecked},
};

#[derive(Accounts)]
pub struct RebalanceOut<'info> {
    #[account(
        mut,
        constraint = config_data.withdraw_tos.contains(signer.key) @ MemeshotVaultError::SenderNotAllowed
    )]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub token_holder: UncheckedAccount<'info>,
    #[account(
        constraint = super::is_registered(to_user.key, &config_data.cloud_wallet_program)
            @ MemeshotVaultError::InvalidRebalanceTo
    )]
    pub to_user: UncheckedAccount<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump)]
    pub config_data: Account<'info, ConfigData>,
    #[account(mut)]
    pub token_holder_token_account: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = token_mint,
        associated_token::authority = to_user,
        associated_token::token_program = token_program,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_mint: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct RebalanceOutEvent {
    pub token_holder: Pubkey,
    pub to_user: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
}

pub(crate) fn handler(ctx: Context<RebalanceOut>, decimals: u8, amount: u64) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"token_holder",
        &[ctx.accounts.config_data.partially_filled_remaining_pct_base],
    ]];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.token_holder_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.token_holder.to_account_info(),
            mint: ctx.accounts.token_mint.to_account_info(),
        },
        signer_seeds,
    );
    transfer_checked(cpi_ctx, amount, decimals)?;
    emit!(RebalanceOutEvent {
        token_holder: ctx.accounts.to_user.key(),
        to_user: ctx.accounts.signer.key(),
        token_mint: ctx.accounts.token_mint.key(),
        amount,
    });
    Ok(())
}
