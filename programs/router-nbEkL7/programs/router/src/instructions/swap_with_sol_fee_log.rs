use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[event_cpi]
#[derive(Accounts)]
pub struct SwapWithSolFeeLog<'info> {

    #[account(constraint = crate::is_allowed_router(router.key) @ crate::RouterError::InvalidRouter)]
    pub router: UncheckedAccount<'info>,
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub fee_receiver: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}




use super::SwapWithFeeLogEvent;







pub(crate) fn swap_with_sol_fee_log<'info>(
    ctx: Context<'_, '_, '_, 'info, SwapWithSolFeeLog<'info>>,
    fee_amount: u64,
    commission_rate: u16,
    data: &Vec<u8>,
) -> Result<()> {
    let data: &[u8] = data;
    let mut before = 0;
    if fee_amount == 0 {
        before = ctx.accounts.user.lamports();
        } else {
        let fee_amt = fee_amount * commission_rate as u64 / 10000;
        if fee_amt > 0 {
            let cpi_accounts = Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.fee_receiver.to_account_info(),
            };
            let cpi = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);
            transfer(cpi, fee_amt)?;
        }
    }
    super::route(
        ctx.accounts.router.key,
        ctx.remaining_accounts,
        0,
        ctx.remaining_accounts.len(),
        data,
    )?;
    if fee_amount != 0 || commission_rate == 0 {
        return Ok(());
    }










    let delta = ctx.accounts.user.lamports()
        .checked_sub(before)
        .unwrap();

    let fee_amt = delta * commission_rate as u64 / 10000;
    let cpi_accounts = Transfer {
        from: ctx.accounts.user.to_account_info(),
        to: ctx.accounts.fee_receiver.to_account_info(),
    };
    let cpi2 = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);
    transfer(cpi2, fee_amt)?;
    emit_cpi!(SwapWithFeeLogEvent {
        mint: None,
        receiver: *ctx.accounts.fee_receiver.key, amount: fee_amt,
    });
    Ok(())
}
