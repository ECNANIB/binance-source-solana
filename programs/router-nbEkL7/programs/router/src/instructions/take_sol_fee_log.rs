use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[event_cpi]
#[derive(Accounts)]
pub struct TakeSolFeeLog<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub fee_receiver: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}


use super::SwapWithFeeLogEvent;


pub(crate) fn take_sol_fee_log<'info>(
    ctx: Context<'_, '_, '_, 'info, TakeSolFeeLog<'info>>,
    fee_amount: u64,
    commission_rate: u16,
) -> Result<()> {
    let user_acc = &ctx.accounts.user;
    let __ea_bump = ctx.bumps.event_authority;
    require_gt!(fee_amount, 0);

    let fee_amt = fee_amount * commission_rate as u64 / 10000;
    if fee_amt > 0 {
        let cpi_accounts = Transfer {
            from: user_acc.to_account_info(),
            to: ctx.accounts.fee_receiver.to_account_info(),
        };
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, fee_amt)?;
    }
    struct EmitCtx<'a, 'info> {
        accounts: &'a TakeSolFeeLog<'info>, bumps: TakeSolFeeLogBumps
    }
    let ctx = EmitCtx { accounts: &ctx.accounts, bumps: TakeSolFeeLogBumps { event_authority: __ea_bump } };
    emit_cpi!(SwapWithFeeLogEvent {
        mint: None, receiver: ctx.accounts.fee_receiver.key(), amount: fee_amt, });
    Ok(())
}
