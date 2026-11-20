use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;

#[derive(Accounts)]
pub struct CloseWsol<'info> {
    #[account(mut, constraint = config_data.managers.contains(signer.key) @ CloudWalletError::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump, constraint = !config_data.paused @ CloudWalletError::Paused)]
    pub config_data: Box<Account<'info, ConfigData>>,
    #[account(mut)]

    pub token_holder: UncheckedAccount<'info>,
    #[account(mut, token::mint = wsol_mint)]
    pub token_holder_wsol_account: InterfaceAccount<'info, TokenAccount>,

    pub wsol_mint: UncheckedAccount<'info>,

    pub wsol_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn close_wsol(ctx: Context<CloseWsol>) -> Result<()> {
    crate::instructions::check_mint(ctx.accounts.wsol_mint.key)?;

    let signer_seeds: &[&[&[u8]]] = &[&[b"token_holder", &[ctx.accounts.config_data.token_holder_bump]]];

    anchor_spl::token_interface::close_account(CpiContext::new_with_signer(
        ctx.accounts.wsol_program.to_account_info(),
        anchor_spl::token_interface::CloseAccount {
            account: ctx.accounts.token_holder_wsol_account.to_account_info(),
            destination: ctx.accounts.token_holder.to_account_info(),
            authority: ctx.accounts.token_holder.to_account_info(),
        },
        signer_seeds,
    ))?;
    Ok(())
}
