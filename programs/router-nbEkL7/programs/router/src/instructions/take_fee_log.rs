use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, TokenAccount, TokenInterface, TransferChecked,
};

#[event_cpi]
#[derive(Accounts)]
pub struct TakeFeeLog<'info> {
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




use super::SwapWithFeeLogEvent;




pub(crate) fn take_fee_log<'info>(
    ctx: Context<'_, '_, '_, 'info, TakeFeeLog<'info>>,
    fee_amount: u64,
    commission_rate: u16,
    decimals: u8,
) -> Result<()> {
    let user_acc = &ctx.accounts.user;
    let __ea_bump = ctx.bumps.event_authority;
    require_gt!(fee_amount, 0);

    let fee_amt = fee_amount * commission_rate as u64 / 10000;
    if fee_amt > 0 {
        let cpi_accounts = TransferChecked {
            authority: user_acc.to_account_info(),
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.fee_token_account.to_account_info(),
            mint: ctx.accounts.fee_token_mint.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_ctx, fee_amt, decimals)?;
    }
    struct EmitCtx<'a, 'info> {
        accounts: &'a TakeFeeLog<'info>, bumps: TakeFeeLogBumps
    }
    let ctx = EmitCtx { accounts: &ctx.accounts, bumps: TakeFeeLogBumps { event_authority: __ea_bump } };
    emit_cpi!(SwapWithFeeLogEvent {
        mint: Some(ctx.accounts.user_token_account.mint),
        receiver: ctx.accounts.fee_token_account.owner,
        amount: fee_amt,
    });
    Ok(())

}
