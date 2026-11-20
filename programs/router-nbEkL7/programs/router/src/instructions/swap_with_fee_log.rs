use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
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
    #[account(mut, token::mint = fee_token_mint, token::authority = user)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, token::mint = fee_token_mint)]
    pub fee_token_account: InterfaceAccount<'info, TokenAccount>,
    pub fee_token_mint: UncheckedAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
}






use super::SwapWithFeeLogEvent;










pub(crate) fn swap_with_fee_log<'info>(
    ctx: Context<'_, '_, '_, 'info, SwapWithFeeLog<'info>>,
    fee_amount: u64,
    commission_rate: u16,
    decimals: u8,
    data: &Vec<u8>,
) -> Result<()> {

    let data: &[u8] = data;
    let mut before = 0;

    if fee_amount != 0 {
        let fee_amt = fee_amount * commission_rate as u64 / 10000;
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
    } else {
        before = ctx.accounts.user_token_account.amount;
    }
    fn swap_with_fee_log_rest<'info>(ctx: Context<'_, '_, '_, 'info, SwapWithFeeLog<'info>>, before: u64, fee_amount: u64, commission_rate: u16, decimals: u8, data: &[u8]) -> Result<()> { if let Err(e) = super::route(
        ctx.accounts.router.key,
        ctx.remaining_accounts,
        0,
        ctx.remaining_accounts.len(),
        data,
    ) { return Err(e); }
    if fee_amount == 0 && commission_rate != 0 {










        let after = reloaded_amount(&mut ctx.accounts.user_token_account)?;
        let delta = after
            .checked_sub(before)
        .unwrap();
        let amt = delta * commission_rate as u64 / 10000;
        let cpi_accounts = TransferChecked {
            authority: ctx.accounts.user.to_account_info(),
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.fee_token_account.to_account_info(),
            mint: ctx.accounts.fee_token_mint.to_account_info(),
        };
        let cpi = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts); if let Err(e) = transfer_checked(cpi, amt, decimals) { return Err(e); }
        emit_cpi!(SwapWithFeeLogEvent {
            mint: Some(ctx.accounts.user_token_account.mint),
            receiver: ctx.accounts.fee_token_account.owner,
            amount: amt,
        });
    }
    Ok(()) }
    swap_with_fee_log_rest(ctx, before, fee_amount, commission_rate, decimals, data) }








pub fn swap<'info>(
    ctx: Context<'_, '_, '_, 'info, Swap<'info>>,
    data: Vec<u8>,
) -> Result<()> {
    super::route(
        ctx.accounts.router.key,
        ctx.remaining_accounts,
        0,
        ctx.remaining_accounts.len(),
        &data,
    )?;
    Ok(())
}



fn reloaded_amount<'info>(acct: &mut InterfaceAccount<'info, TokenAccount>) -> Result<u64> {
    acct.reload()?;
    Ok(acct.amount)
}
