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







pub(crate) fn take_sol_fee_log<'info>(
    ctx: Context<'_, '_, '_, 'info, TakeSolFeeLog<'info>>,
    amount: u64,
    fee_bps: u16,
) -> Result<()> {
    require_gt!(amount, 0);


    crate::validate_fee_receiver(ctx.accounts.fee_receiver.key)?;
    let fee_amt = amount * fee_bps as u64 / 10000;
    if fee_amt > 0 {
        let cpi_accounts = Transfer {
            from: ctx.accounts.user.to_account_info(),
            to: ctx.accounts.fee_receiver.to_account_info(),
        };
        let cpi = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);
        transfer(cpi, fee_amt)?;
    }
    emit_cpi!(crate::SwapWithFeeLogEvent {
        mint: None,
        receiver: *ctx.accounts.fee_receiver.key,
        amount: fee_amt,
    });
    Ok(())
}


#[event]
pub struct SwapWithFeeLogEvent {
    pub mint: Option<Pubkey>,
    pub receiver: Pubkey,
    pub amount: u64,
}
