use anchor_lang::prelude::*;

use crate::state::config_data::LazyConfigData;

use anchor_spl::token_interface::{self, TokenAccount, TokenInterface, TransferChecked};
use crate::instructions::{agg_router_swap, check_nonce, check_partially_filled, is_agg_router, token_account_amount};
use crate::state::{ConfigData, SwapEvent, TokenConfigData};
use crate::ErrorCode;



















#[event_cpi]
#[derive(Accounts)]
#[instruction(params: SwapToSolV2Params)]
pub struct SwapToSo001<'info> {




    #[account(mut, constraint = config_data.load_addresses_1()?.contains(signer.key) @ ErrorCode::SenderNotAllowed)]
    pub signer: Signer<'info>,







    #[account(
        seeds = [b"config_data"],
        bump = *config_data.load_bump_b2()?,
        constraint = !*config_data.load_paused()? @ ErrorCode::Paused,
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
    #[account(mut, token::mint = wsol_mint)]
    pub token_holder_wsol_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub user: UncheckedAccount<'info>,
    #[account(constraint = is_agg_router(&*config_data.load_pubkey_60()?, agg_router.key) @ ErrorCode::InvalidProgram)]
    pub agg_router: UncheckedAccount<'info>,
    pub wsol_mint: UncheckedAccount<'info>,
    pub wsol_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SwapToSolV2Params {
    pub nonce: u64,
    pub amount: u64,
    pub min_out: u64,
    pub expire_at: u32,
    pub flag: u8,
    pub signature: [u8; 64],
    pub data: Vec<u8>,
}

pub(crate) fn handler<'info>(ctx: Context<'_, '_, '_, 'info, SwapToSo001<'info>>, params: SwapToSolV2Params) -> Result<()> {
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;
    let expire_at = params.expire_at as i64;
    require_gt!(expire_at, now, ErrorCode::Expired);
    let nonce = params.nonce;
    check_nonce(
        &ctx.accounts.signer,
        &ctx.accounts.nonce_data,
        &ctx.accounts.system_program,
        nonce, ctx.bumps.nonce_data,
    )?;










    let mut msg = Vec::with_capacity(168);
    msg.extend_from_slice(&ctx.accounts.config_data.load_field007()?.to_le_bytes());
    msg.extend_from_slice(crate::ID.as_ref());
    msg.extend_from_slice(b"swap");
    msg.extend_from_slice(&nonce.to_le_bytes());
    msg.extend_from_slice(ctx.accounts.token_holder_from_token_account.mint.as_ref());
    let amount = params.amount;
    msg.extend_from_slice(&amount.to_le_bytes());
    msg.extend_from_slice(ctx.accounts.wsol_mint.key().to_bytes().as_ref());
    let min_out = params.min_out;
    msg.extend_from_slice(&min_out.to_le_bytes());
    msg.extend_from_slice(ctx.accounts.user.key().to_bytes().as_ref());
    msg.extend_from_slice(&(expire_at as u32).to_le_bytes());
    let h = anchor_lang::solana_program::hash::hash(&msg);
    let recovered = anchor_lang::solana_program::secp256k1_recover::secp256k1_recover(&h.to_bytes(), params.flag, &params.signature).map_err(|_| ErrorCode::InvalidSignature)?;
    {
    let pks = ctx.accounts.config_data.load_secp256k1_pubkeys()?;
    require_gt!(pks.len() as u64, 0, ErrorCode::InvalidSignature);
    require!(pks.contains(&recovered.to_bytes()), ErrorCode::InvalidSignature);
    }
    let spend_before = ctx.accounts.token_holder_from_token_account.amount;
    let wsol_before = ctx.accounts.token_holder_wsol_account.amount;
    let holder_before = ctx.accounts.token_holder.lamports();

    {
        let signer_seeds: &[&[&[u8]]; 1] = &[&[
            b"token_holder",
            &[*ctx.accounts.config_data.load_bump_b3()?],
        ]];
        agg_router_swap(
            ctx.accounts.agg_router.key,
            ctx.accounts.token_holder.key,
            ctx.remaining_accounts,
            ctx.remaining_accounts.len() as u8,
            &params.data,
            signer_seeds,
        )?;
    }
    let holder_after = ctx.accounts.token_holder.lamports();









    require_gte!(holder_after, holder_before, ErrorCode::NumberMismatch);
    ctx.accounts.token_holder_from_token_account.reload()?;
    let spend_after = ctx.accounts.token_holder_from_token_account.amount;











    let spent = spend_before.checked_sub(spend_after).ok_or(error!(ErrorCode::MathOverflow).with_values((spend_before, spend_after)))?;
    check_partially_filled(spent, amount, *ctx.accounts.config_data.load_partially_filled_remaining_pct_base()?)?;
    ctx.accounts.token_holder_wsol_account.reload()?;
    let wsol_after = ctx.accounts.token_holder_wsol_account.amount;











    let received = wsol_after.checked_sub(wsol_before).ok_or(error!(ErrorCode::MathOverflow).with_values((wsol_before, wsol_after)))?;
    if ctx.accounts.token_holder.lamports() < received {
        let signer_seeds: &[&[&[u8]]] = &[&[b"token_holder", &[*ctx.accounts.config_data.load_bump_b3()?]]];
        let close_program = ctx.accounts.wsol_program.to_account_info();
        let close_accounts = token_interface::CloseAccount {
            account: ctx.accounts.token_holder_wsol_account.to_account_info(),
            destination: ctx.accounts.token_holder.to_account_info(),
            authority: ctx.accounts.token_holder.to_account_info(),
        };
        let cpi_close = CpiContext {
            program: close_program,
            accounts: close_accounts,
            signer_seeds,
            remaining_accounts: Vec::new(),
        };
        token_interface::close_account(cpi_close)?;
    }
    let out_amount = {
        let program = ctx.accounts.system_program.to_account_info();
        let accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.token_holder.to_account_info(),
            to: ctx.accounts.user.to_account_info(),
        };
        let signer_seeds: &[&[&[u8]]] = &[&[b"token_holder", &[*ctx.accounts.config_data.load_bump_b3()?]]];
        let transfer_ctx = CpiContext::new_with_signer(program, accounts, signer_seeds);
        let out_before = ctx.accounts.user.lamports();
        anchor_lang::system_program::transfer(transfer_ctx, received)?;
    let out_after = ctx.accounts.user.lamports();










    out_after.checked_sub(out_before).ok_or(error!(ErrorCode::MathOverflow).with_values((out_before, out_after)))? };


    require_gte!(out_amount, min_out, ErrorCode::ReturnAmountNotEnough);
    emit_cpi!(SwapEvent {
        nonce: nonce, mint_in: ctx.accounts.token_holder_from_token_account.mint, amount: amount,
        pubkey_opt: None, out_amount, recipient: *ctx.accounts.user.key });
    Ok(())
}
