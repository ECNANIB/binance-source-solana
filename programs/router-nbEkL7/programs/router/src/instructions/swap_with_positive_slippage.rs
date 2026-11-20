use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};




#[event_cpi]
#[derive(Accounts)]
pub struct SwapWithPositiveSlippage<'info> {

    #[account(constraint = crate::is_allowed_router(router.key) @ crate::RouterError::InvalidRouter)]
    pub router: UncheckedAccount<'info>,
    pub user: Signer<'info>,
    #[account(mut, token::mint = from_token_mint, token::authority = user)]
    pub user_from_token_account: InterfaceAccount<'info, TokenAccount>,
    pub from_token_mint: InterfaceAccount<'info, Mint>,
    #[account(mut, token::mint = to_token_mint, token::authority = user)]
    pub user_to_token_account: InterfaceAccount<'info, TokenAccount>,
    pub to_token_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub fee_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, token::mint = to_token_mint)]
    pub ps_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}










pub fn swap_with_positive_slippage<'info>(
    ctx: Context<'_, '_, '_, 'info, SwapWithPositiveSlippage<'info>>,

    original_amount: u64,
    commission_rate: u16,
    wrap_sol: bool,
    fee_amount: u64,
    data: &[u8],
) -> Result<()> {



























    require!(commission_rate <= 10_000, crate::RouterError::InvalidFee);




    require!(original_amount > 0, crate::RouterError::InvalidOriginalAmt);
    crate::validate_fee_receiver(&ctx.accounts.ps_token_account.owner)?;
    let fee_account_mint = &ctx.accounts.fee_token_account.mint;
    let fee_mint = if wrap_sol {
        ctx.accounts.from_token_mint.key()
    } else {
        ctx.accounts.to_token_mint.key()
    };










    require_keys_eq!(*fee_account_mint, fee_mint, crate::RouterError::InvalidFeeTokenAccount);
    let from_before = ctx.accounts.user_from_token_account.amount;
    let to_before = ctx.accounts.user_to_token_account.amount;
    let mut comm1: u64 = 0;
    if wrap_sol && commission_rate > 0 {
        comm1 = (commission_rate as u128 * original_amount as u128 / 10_000) as u64;
        if comm1 > 0 {
            let cpi1 = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    authority: ctx.accounts.user.to_account_info(),
                    from: ctx.accounts.user_from_token_account.to_account_info(),
                    to: ctx.accounts.fee_token_account.to_account_info(),
                    mint: ctx.accounts.from_token_mint.to_account_info(),
                },
            );
            transfer_checked(cpi1, comm1, ctx.accounts.from_token_mint.decimals)?;
        }
    }
    super::route(
        ctx.accounts.router.key,
        ctx.remaining_accounts,
        0,
        ctx.remaining_accounts.len(),
        data,
    )?;
    ctx.accounts.user_from_token_account.reload()?;
    ctx.accounts.user_to_token_account.reload()?;
    let from_after = ctx.accounts.user_from_token_account.amount;


























    require_gte!(from_before, from_after, crate::RouterError::InvalidOriginalAmt);
    let spent = from_before - from_after;





    require_gt!(spent, 0u64, crate::RouterError::InvalidOriginalAmt);
    require_gte!(original_amount, spent, crate::RouterError::InvalidOriginalAmt);
    let to_after = ctx.accounts.user_to_token_account.amount;




    require_gte!(to_after, to_before, crate::RouterError::InvalidOriginalAmt);
    let received = to_after - to_before;
    let mut comm2: u64 = 0;
    if commission_rate > 0 && !wrap_sol {
        comm2 = (received as u128 * commission_rate as u128 / 10_000) as u64;
        if comm2 > 0 {
            let cpi2 = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    authority: ctx.accounts.user.to_account_info(),
                    from: ctx.accounts.user_to_token_account.to_account_info(),
                    to: ctx.accounts.fee_token_account.to_account_info(),
                    mint: ctx.accounts.to_token_mint.to_account_info(),
                },
            );
            transfer_checked(cpi2, comm2, ctx.accounts.to_token_mint.decimals)?;
        }
    }





    let rest = received.checked_sub(comm2).unwrap();
    let mut ps: u64 = 0;
    if rest > fee_amount {
        ps = rest - fee_amount;
        let cpi3 = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                authority: ctx.accounts.user.to_account_info(),
                from: ctx.accounts.user_to_token_account.to_account_info(),
                to: ctx.accounts.ps_token_account.to_account_info(),
                mint: ctx.accounts.to_token_mint.to_account_info(),
            },
        );
        transfer_checked(cpi3, ps, ctx.accounts.to_token_mint.decimals)?;
    }
    let net_in = net_fee(&ctx.accounts.from_token_mint.to_account_info(), comm1)?;
    let net_out = net_fee(&ctx.accounts.to_token_mint.to_account_info(), comm2)?;
    let net_ps = net_fee(&ctx.accounts.to_token_mint.to_account_info(), ps)?;
    emit_cpi!(super::PositiveSlippageLog {
        user: ctx.accounts.user.key(), router: ctx.accounts.router.key(),
        from_token_mint: Some(ctx.accounts.from_token_mint.key()), order_id: original_amount,
        to_token_mint: Some(ctx.accounts.to_token_mint.key()), user_from_token_account: ctx.accounts.fee_token_account.owner,
        fee_token_account: Some(fee_mint), original_amount: net_in, received_amount: net_out, user_to_token_account: ctx.accounts.ps_token_account.owner, fee_amount, positive_slippage: rest, balance_before: net_ps,
    });
    Ok(())
}
fn net_fee(mint: &AccountInfo, amount: u64) -> Result<u64> { super::net_of_transfer_fee(mint, amount) }





pub fn mint_decimals(ai: &AccountInfo) -> Result<u8> {
    let d = ai.try_borrow_data()?;
    if d.len() < 45 {
        return Err(error!(crate::RouterError::InvalidFeeTokenAccount));
    }
    Ok(d[44])
}
