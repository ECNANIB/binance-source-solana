use anchor_lang::prelude::*;

use crate::errors::MemeshotVaultError;
use crate::state::{ConfigData, LazyConfigData, TokenConfigData};
use anchor_spl::token_interface::TokenAccount;

#[event]
pub struct SellToSelfFromAlphaEvent {
    pub amount_in: u64,
    pub from_mint: Pubkey,
    pub filled_amount: u64,
    pub to_mint: Pubkey,
    pub amount_out: u64,
    pub total_amount: u64,
    pub cloud_wallet_program: Pubkey,
}

#[derive(Accounts)]
#[event_cpi]
pub struct SellToSelfFromAlpha<'info> {
    #[account(
        mut,
        signer,
        constraint = config_data.load_senders()?.contains(signer.key) @ MemeshotVaultError::SenderNotAllowed
    )]
    pub signer: UncheckedAccount<'info>,
    #[account(
        seeds = [b"config_data"],
        bump = *config_data.load_bump()?,
        constraint = !*config_data.load_is_frozen()? @ MemeshotVaultError::Paused
    )]
    pub config_data: LazyAccount<'info, ConfigData>,
    #[account(
        seeds = [b"config_data", token_holder_to_token_account.mint.as_ref()],
        bump = to_token_config_data.bump
    )]
    pub to_token_config_data: Box<Account<'info, TokenConfigData>>,
    pub token_holder: UncheckedAccount<'info>,
    #[account(mut)]
    pub token_holder_from_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub token_holder_to_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        constraint = super::is_registered(
            &*config_data.load_rebalance_out_holder()?,
            &cloud_wallet_program.key()
        ) @ MemeshotVaultError::InvalidProgram
    )]
    pub cloud_wallet_program: UncheckedAccount<'info>,
    #[account(
        constraint = super::is_agg_router(&*config_data.load_agg_router()?, &agg_router.key())
            @ MemeshotVaultError::InvalidProgram
    )]
    pub agg_router: UncheckedAccount<'info>,
}


pub const SELL_TO_SELF_DISCRIMINATOR: u64 = 0xf90d_6515_7c5e_9cf4;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SellToSelfFromAlphaArgs {
    pub amount_in: u64,
    pub nonce: u64,
    pub total_amount: u64,
    pub filled_amount: u64,
    pub min_amount_out: u64,
    pub deadline: u32,
    pub pct_base: u32,
    pub kind: u8,
    pub flag: u8,
    pub route_data: Vec<u8>,
}

pub(crate) fn handler(
    ctx: Context<SellToSelfFromAlpha>,
    args: SellToSelfFromAlphaArgs,
) -> Result<()> {













    let clock = Clock::get()?;
    require_gte!(
        args.deadline as i64,
        clock.unix_timestamp,
        MemeshotVaultError::Expired
    );
    let from_before = ctx.accounts.token_holder_from_token_account.amount;
    let lamports_before = ctx.accounts.token_holder.lamports();
    let mut data: Vec<u8> = Vec::with_capacity(37);
    data.extend_from_slice(&SELL_TO_SELF_DISCRIMINATOR.to_le_bytes());
    data.extend_from_slice(&args.nonce.to_le_bytes());
    data.extend_from_slice(&args.total_amount.to_le_bytes());
    data.extend_from_slice(&args.filled_amount.to_le_bytes());
    data.extend_from_slice(&args.pct_base.to_le_bytes());
    data.extend_from_slice(&[args.kind]);
    super::invoke_route(
        ctx.accounts.cloud_wallet_program.key,
        ctx.accounts.token_holder.key,
        ctx.remaining_accounts,
        args.flag,
        ctx.remaining_accounts.len() as u8,
        &data,
        &[&[
            b"token_holder",
            &[*ctx
                .accounts
                .config_data
                .load_partially_filled_remaining_pct_base()?],
        ]],
    )?;
    let to_before = ctx.accounts.token_holder_to_token_account.amount;






    super::invoke_route(
        ctx.accounts.agg_router.key,
        ctx.accounts.token_holder.key,
        ctx.remaining_accounts,
        0,
        args.flag,
        &args.route_data,
        &[&[
            b"token_holder",
            &[*ctx
                .accounts
                .config_data
                .load_partially_filled_remaining_pct_base()?],
        ]],
    )?;
    ctx.accounts.token_holder_to_token_account.reload()?;
    let delta = ctx
        .accounts
        .token_holder_to_token_account
        .amount
        .checked_sub(to_before)
        .ok_or(MemeshotVaultError::MathOverflow)?;
    require_gte!(
        delta,
        args.min_amount_out,
        MemeshotVaultError::ReturnAmountNotEnough
    );
    ctx.accounts.token_holder_from_token_account.reload()?;





    let total = ctx
        .accounts
        .token_holder_from_token_account
        .amount
        .checked_sub(from_before)
        .and_then(|sold| sold.checked_add(args.filled_amount))
        .ok_or(MemeshotVaultError::MathOverflow)?;
    super::check_partially_filled(
        args.filled_amount,
        total,
        *ctx.accounts.config_data.load_expire_duration()?,
    )?;
    let lamports_after = ctx.accounts.token_holder.lamports();
    require_gte!(
        lamports_after,
        lamports_before,
        MemeshotVaultError::NumberMismatch
    );
    emit_cpi!(SellToSelfFromAlphaEvent {
        amount_in: args.amount_in,
        from_mint: ctx.accounts.token_holder_from_token_account.mint,
        filled_amount: args.filled_amount,
        to_mint: ctx.accounts.token_holder_to_token_account.mint,
        amount_out: delta,
        total_amount: args.total_amount,
        cloud_wallet_program: ctx.accounts.cloud_wallet_program.key(),
    });
    Ok(())
}
