use anchor_lang::prelude::*;

use crate::errors::MemeshotVaultError;
use crate::state::ConfigData;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{TokenAccount, TokenInterface},
};

#[derive(Accounts)]
pub struct SyncNative<'info> {
    #[account(
        mut,
        constraint = config_data.withdraw_tos.contains(signer.key) || config_data.senders.contains(signer.key) @ MemeshotVaultError::SenderNotAllowed
    )]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub token_holder: UncheckedAccount<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump)]
    pub config_data: Account<'info, ConfigData>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = wsol_mint,
        associated_token::authority = token_holder,
        associated_token::token_program = token_program,
    )]
    pub wsol_account: InterfaceAccount<'info, TokenAccount>,
    pub wsol_mint: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn handler(ctx: Context<SyncNative>, amount: u64) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"token_holder",
        &[ctx.accounts.config_data.partially_filled_remaining_pct_base],
    ]];
    super::require_wsol_mint(ctx.accounts.wsol_mint.key)?;
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.token_holder.to_account_info(),
            to: ctx.accounts.wsol_account.to_account_info(),
        },
        signer_seeds,
    );
    anchor_lang::system_program::transfer(transfer_ctx, amount)?;
    let token_program = ctx.accounts.token_program.to_account_info();
    match *token_program.key {
        super::TOKEN_PROGRAM | super::TOKEN_2022_PROGRAM => {}
        _ => {
            let e = Err(MemeshotVaultError::InvalidProgram.into());
            drop(token_program);
            return e;
        }
    }
    anchor_spl::token_interface::sync_native(CpiContext::new_with_signer(
        token_program,
        anchor_spl::token_interface::SyncNative {
            account: ctx.accounts.wsol_account.to_account_info(),
        },
        signer_seeds,
    ))?;
    Ok(())
}
