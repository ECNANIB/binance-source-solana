use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::invoke;
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












pub(crate) fn swap_with_sol_fee_log<'info>(
    ctx: Context<'_, '_, '_, 'info, SwapWithSolFeeLog<'info>>,
    amount: u64,
    fee_bps: u16,
    data: &[u8],
) -> Result<()> {
    crate::validate_fee_receiver(ctx.accounts.fee_receiver.key)?;
    let mut before = 0;
    if amount == 0 {
        before = ctx.accounts.user.lamports();
    } else {
        let fee_amt = amount * fee_bps as u64 / 10000;
        if fee_amt > 0 {
            let cpi_accounts = Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.fee_receiver.to_account_info(),
            };
            let cpi = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);
            transfer(cpi, fee_amt)?;
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
    let delta = ctx.accounts.user.lamports()
        .checked_sub(before)
        .unwrap();

    let fee_amt = delta * fee_bps as u64 / 10000;
    let cpi_accounts = Transfer {
        from: ctx.accounts.user.to_account_info(),
        to: ctx.accounts.fee_receiver.to_account_info(),
    };
    let cpi = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);
    transfer(cpi, fee_amt)?;
    emit_cpi!(crate::SwapWithFeeLogEvent {
        mint: None,
        receiver: *ctx.accounts.fee_receiver.key,
        amount: fee_amt,
    });
    Ok(())
}
