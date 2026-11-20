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










pub(crate) fn take_fee_log<'info>(
    ctx: Context<'_, '_, '_, 'info, TakeFeeLog<'info>>,
    amount: u64,
    fee_bps: u16,
    decimals: u8,
) -> Result<()> {
    require_gt!(amount, 0);


    crate::validate_fee_receiver(&ctx.accounts.fee_token_account.owner)?;
    let fee_amt = amount * fee_bps as u64 / 10000;
    if fee_amt > 0 {
        let cpi_accounts = TransferChecked {
            authority: ctx.accounts.user.to_account_info(),
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.fee_token_account.to_account_info(),
            mint: ctx.accounts.fee_token_mint.to_account_info(),
        };
        let cpi =
            CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

        transfer_checked(cpi, fee_amt, decimals)?;
    }
    emit_cpi!(crate::SwapWithFeeLogEvent {
        mint: Some(ctx.accounts.user_token_account.mint),
        receiver: ctx.accounts.fee_token_account.owner,
        amount: fee_amt,
    });
    Ok(())
}
