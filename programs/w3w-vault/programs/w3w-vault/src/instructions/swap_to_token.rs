use anchor_lang::prelude::*;

use crate::state::config_data::LazyConfigData;

use anchor_spl::token_interface::{self, TokenAccount, TokenInterface, TransferChecked};
use crate::instructions::{agg_router_swap, check_nonce, check_partially_filled, is_agg_router, token_account_amount};
use crate::state::{ConfigData, SwapEvent, TokenConfigData};
use crate::ErrorCode;



















#[event_cpi]
#[derive(Accounts)]
#[instruction(params: SwapToTokenParams)]
pub struct SwapToToken<'info> {




    #[account(mut, constraint = config_data.load()?.addresses_1.contains(signer.key) @ ErrorCode::SenderNotAllowed)]
    pub signer: Signer<'info>,




    #[account(
        seeds = [b"config_data"],
        bump = config_data.load()?.bump_b2,
        constraint = !config_data.load()?.paused @ ErrorCode::Paused,
    )]
    pub config_data: LazyAccount<'info, ConfigData>,





    #[account(mut, seeds = [b"nonce_data", &params.nonce.to_le_bytes()], bump)]
    pub nonce_data: UncheckedAccount<'info>,
    #[account(
        seeds = [b"config_data", token_holder_from_token_account.mint.as_ref()],
        bump = from_token_config_data.bump,
        constraint = params.amount <= from_token_config_data.fiel00p @ ErrorCode::TradeAmountExceedsLimit,
    )]
    pub from_token_config_data: Box<Account<'info, TokenConfigData>>,
    pub token_holder: UncheckedAccount<'info>,
    #[account(mut)]
    pub token_holder_from_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub token_holder_to_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub user_to_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(constraint = is_agg_router(&*config_data.load_pubkey_60()?, agg_router.key) @ ErrorCode::InvalidProgram)]
    pub agg_router: UncheckedAccount<'info>,
    pub to_token_mint: UncheckedAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SwapToTokenParams {
    pub nonce: u64,
    pub amount: u64,
    pub min_out: u64,
    pub expire_at: u32,
    pub decimals: u8,
    pub data: Vec<u8>,
}

pub(crate) fn handler<'info>(ctx: Context<'_, '_, '_, 'info, SwapToToken<'info>>, params: SwapToTokenParams) -> Result<()> {
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;
    let expire_at = params.expire_at as i64;
    require_gt!(expire_at, now, ErrorCode::Expired);
    check_nonce(
        &ctx.accounts.signer,
        &ctx.accounts.nonce_data,
        &ctx.accounts.system_program,
        params.nonce, ctx.bumps.nonce_data,
    )?;
    let spend_before = ctx.accounts.token_holder_from_token_account.amount;
    let received_before = ctx.accounts.token_holder_to_token_account.amount;
    let holder_before = ctx.accounts.token_holder.lamports();
    { let signer_seeds: &[&[&[u8]]; 1] = &[&[b"token_holder", &[*ctx.accounts.config_data.load_bump_b3()?]]];
    agg_router_swap(
        ctx.accounts.agg_router.key, ctx.accounts.token_holder.key,
        ctx.remaining_accounts,
        ctx.remaining_accounts.len() as u8,
        &params.data,
        signer_seeds,
    )?; }
    let holder_after = ctx.accounts.token_holder.lamports();


















    require_gte!(holder_after, holder_before, ErrorCode::NumberMismatch);
    ctx.accounts.token_holder_from_token_account.reload()?;
    let spend_after = ctx.accounts.token_holder_from_token_account.amount;











    let spent = spend_before.checked_sub(spend_after).ok_or(error!(ErrorCode::MathOverflow).with_values((spend_before, spend_after)))?;
    check_partially_filled(spent, params.amount, *ctx.accounts.config_data.load_partially_filled_remaining_pct_base()?)?;
    ctx.accounts.token_holder_to_token_account.reload()?;
    let received_after = ctx.accounts.token_holder_to_token_account.amount;











    let received = received_after.checked_sub(received_before).ok_or(error!(ErrorCode::MathOverflow).with_values((received_before, received_after)))?;
    let out_before = ctx.accounts.user_to_token_account.amount;
    {
        let signer_seeds: &[&[&[u8]]] = &[&[b"token_holder", &[*ctx.accounts.config_data.load_bump_b3()?]]];
    let program = ctx.accounts.token_program.to_account_info();
    let accounts =
        TransferChecked {
            from: ctx.accounts.token_holder_to_token_account.to_account_info(),
            to: ctx.accounts.user_to_token_account.to_account_info(),
            authority: ctx.accounts.token_holder.to_account_info(),
            mint: ctx.accounts.to_token_mint.to_account_info(),
        };
    let cpi_xfer = CpiContext::new_with_signer(program, accounts, signer_seeds);


    token_interface::transfer_checked(
        cpi_xfer,
        received, params.decimals)?; }
    ctx.accounts.user_to_token_account.reload()?;
    let out_after = ctx.accounts.user_to_token_account.amount;















    let out_amount = out_after.checked_sub(out_before).ok_or(error!(ErrorCode::MathOverflow).with_values((out_before, out_after)))?;




    require_gte!(out_amount, params.min_out, ErrorCode::ReturnAmountNotEnough);
    emit_cpi!(SwapEvent {
        nonce: params.nonce, mint_in: ctx.accounts.token_holder_from_token_account.mint, amount: params.amount,
        pubkey_opt: Some(ctx.accounts.token_holder_to_token_account.mint),
        out_amount, recipient: ctx.accounts.user_to_token_account.owner });
    Ok(())
}
