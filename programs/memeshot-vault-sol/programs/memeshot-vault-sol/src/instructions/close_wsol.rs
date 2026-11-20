use anchor_lang::prelude::*;

use crate::errors::MemeshotVaultError;
use crate::state::ConfigData;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{close_account, CloseAccount, TokenAccount, TokenInterface},
};

#[derive(Accounts)]
pub struct CloseWsol<'info> {
    #[account(
        mut,
        constraint = config_data.withdraw_tos.contains(signer.key) || config_data.senders.contains(signer.key) @ MemeshotVaultError::SenderNotAllowed
    )]
    pub signer: Signer<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump)]
    pub config_data: Box<Account<'info, ConfigData>>,
    #[account(mut)]
    pub token_holder: UncheckedAccount<'info>,
    #[account(mut, token::mint = wsol_mint)]
    pub token_holder_wsol_account: InterfaceAccount<'info, TokenAccount>,
    pub wsol_mint: UncheckedAccount<'info>,
    pub wsol_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn handler(ctx: Context<CloseWsol>) -> Result<()> {
    super::require_wsol_mint(ctx.accounts.wsol_mint.key)?;
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"token_holder",
        &[ctx.accounts.config_data.partially_filled_remaining_pct_base],
    ]];
    close_account(CpiContext::new_with_signer(
        ctx.accounts.wsol_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.token_holder_wsol_account.to_account_info(),
            destination: ctx.accounts.token_holder.to_account_info(),
            authority: ctx.accounts.token_holder.to_account_info(),
        },
        signer_seeds,
    ))?;
    Ok(())
}
