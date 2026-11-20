use anchor_lang::prelude::*;
use std::collections::HashMap;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token_interface::{
    transfer_checked, TokenAccount, TokenInterface, TransferChecked,
};

#[derive(Accounts)]
pub struct Swap<'info> {


    #[account(constraint = crate::is_allowed_router(router.key) @ crate::RouterError::InvalidRouter)]
    pub router: UncheckedAccount<'info>,
}

#[event_cpi]
#[derive(Accounts)]
pub struct SwapWithFeeLog<'info> {

    #[account(constraint = crate::is_allowed_router(router.key) @ crate::RouterError::InvalidRouter)]
    pub router: UncheckedAccount<'info>,
    pub user: Signer<'info>,
    #[account(
        mut,
        token::mint = fee_token_mint,
        token::authority = user,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, token::mint = fee_token_mint)]
    pub fee_token_account: InterfaceAccount<'info, TokenAccount>,



    pub fee_token_mint: UncheckedAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
}








pub(crate) fn swap_with_fee_log<'info>(
    ctx: Context<'_, '_, '_, 'info, SwapWithFeeLog<'info>>,
    amount: u64,
    fee_bps: u16,
    decimals: u8,
    data: &[u8],
) -> Result<()> {
    crate::validate_fee_receiver(&ctx.accounts.fee_token_account.owner)?;
    let mut before = 0;
    if amount == 0 {
        before = ctx.accounts.user_token_account.amount;
    } else {
        let fee_amt = amount * fee_bps as u64 / 10000;
        if fee_amt > 0 {
            let cpi_accounts = TransferChecked {
                authority: ctx.accounts.user.to_account_info(),
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.fee_token_account.to_account_info(),
                mint: ctx.accounts.fee_token_mint.to_account_info(),
            };
            let cpi = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
            transfer_checked(cpi, fee_amt, decimals)?;
        }
    }
    crate::constants::route(
        ctx.accounts.router.key,
        ctx.remaining_accounts,
        ctx.remaining_accounts.len() as u8,
        data,
    )?;











    if amount != 0 || fee_bps == 0 {
        return Ok(());
    }
    let after = reloaded_amount(&mut ctx.accounts.user_token_account)?;
    let delta = after
        .checked_sub(before)
        .unwrap();
    let fee_amt = delta * fee_bps as u64 / 10000;
    let cpi_accounts = TransferChecked {
        authority: ctx.accounts.user.to_account_info(),
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.fee_token_account.to_account_info(),
        mint: ctx.accounts.fee_token_mint.to_account_info(),
    };
    let cpi = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    transfer_checked(cpi, fee_amt, decimals)?;
    emit_cpi!(crate::SwapWithFeeLogEvent {
        mint: Some(ctx.accounts.user_token_account.mint),
        receiver: ctx.accounts.fee_token_account.owner,
        amount: fee_amt,
    });
    Ok(())
}

pub(crate) fn swap<'info>(
    ctx: Context<'_, '_, '_, 'info, Swap<'info>>,
    data: Vec<u8>,
) -> Result<()> {
    crate::constants::route(
        ctx.accounts.router.key,
        ctx.remaining_accounts,
        ctx.remaining_accounts.len() as u8,
        &data,
    )
}

fn reloaded_amount<'info>(acct: &mut InterfaceAccount<'info, TokenAccount>) -> Result<u64> {
    acct.reload()?;
    Ok(acct.amount)
}
