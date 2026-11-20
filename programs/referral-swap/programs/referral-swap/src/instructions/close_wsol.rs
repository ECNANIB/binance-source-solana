use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;
use anchor_spl::token_interface::{TokenAccount, TokenInterface};

use crate::error::ReferralSwapError;
use crate::state::ConfigData;

#[derive(Accounts)]
pub struct CloseW021<'info> {
    #[account(mut,
        constraint = config_data.send021.contains(signer.key) @ ReferralSwapError::NotAllowed)]
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"config_data"],
        bump = config_data.bump,
        constraint = !config_data.pau021 @ ReferralSwapError::Paused
    )]
    pub config_data: Box<Account<'info, ConfigData>>,

    #[account(
        mut,
        seeds = [b"token_holder"],
        bump = config_data.token_holder_b021
    )]
    pub token_holder: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = wsol_mint,
        associated_token::authority = token_holder
    )]
    pub token_holder_wsol_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"tmp_token_holder"],
        bump = config_data.tmp_token_holder_b021
    )]
    pub tmp_token_holder: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = wsol_mint,
        associated_token::authority = tmp_token_holder
    )]
    pub tmp_token_holder_wsol_account: InterfaceAccount<'info, TokenAccount>,

    pub wsol_mint: UncheckedAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

fn check_wsol_mint(mint: &Pubkey) -> Result<()> {
    match mint {
        &anchor_spl::token::spl_token::native_mint::ID
        | &anchor_spl::token_2022::spl_token_2022::native_mint::ID => Ok(()),
        _ => Err(ReferralSwapError::InvalidParameter.into()),
    }
}

pub(crate) fn handler(ctx: Context<CloseW021>) -> Result<()> {
    check_wsol_mint(ctx.accounts.wsol_mint.key)?;

    if ctx.accounts.token_holder_wsol_account.amount == 0 {
        return Ok(());
    }

    let before = ctx.accounts.token_holder.lamports();
    let amount = ctx.accounts.token_holder_wsol_account.amount;
    let cpi_close;
    let cpi_refund;
    let cpi_transfer;
    {
    let holder_seeds: &[&[&[u8]]] = &[&[b"token_holder", &[ctx.accounts.config_data.token_holder_b021]]];
    cpi_transfer =
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token_2022::TransferChecked {
                from: ctx.accounts.token_holder_wsol_account.to_account_info(),
                to: ctx.accounts.tmp_token_holder_wsol_account.to_account_info(),
                authority: ctx.accounts.token_holder.to_account_info(),
                mint: ctx.accounts.wsol_mint.to_account_info(),
            },
            holder_seeds,
        );
    anchor_spl::token_2022::transfer_checked(
        cpi_transfer,
        amount,
        9,
    )?;
    }

    let tmp_seeds: &[&[&[u8]]] = &[&[b"tmp_token_holder", &[ctx.accounts.config_data.tmp_token_holder_b021]]];

    cpi_close = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        anchor_spl::token_2022::CloseAccount {
            account: ctx.accounts.tmp_token_holder_wsol_account.to_account_info(),
            destination: ctx.accounts.token_holder.to_account_info(),
            authority: ctx.accounts.tmp_token_holder.to_account_info(),
        },
        tmp_seeds,
    );
    anchor_spl::token_2022::close_account(cpi_close)?;

    let after = ctx.accounts.token_holder.lamports();
    let extra = after
        .checked_sub(before)
        .and_then(|delta| delta.checked_sub(amount))
        .ok_or(ReferralSwapError::MathOverflow)?;
    if extra != 0 {
        let refund_seeds: &[&[&[u8]]] = &[&[b"token_holder", &[ctx.accounts.config_data.token_holder_b021]]];

        cpi_refund = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.token_holder.to_account_info(),
                to: ctx.accounts.signer.to_account_info(),
            },
            refund_seeds,
        );
        anchor_lang::system_program::transfer(cpi_refund, extra)?;
    }

    Ok(())
}
