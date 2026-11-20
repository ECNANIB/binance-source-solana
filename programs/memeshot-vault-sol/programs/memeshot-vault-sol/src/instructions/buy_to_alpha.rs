use anchor_lang::prelude::*;

use crate::errors::MemeshotVaultError;
use crate::state::{ConfigData, LazyConfigData, TokenConfigData};
use anchor_spl::token_interface::TokenAccount;

#[event]
pub struct BuyToAlphaEvent {
    pub amount_in: u64,
    pub from_mint: Pubkey,
    pub total_amount: u64,
    pub to_mint: Pubkey,
    pub amount_out: u64,
    pub filled_amount: u64,
    pub cloud_wallet_program: Pubkey,
}

#[derive(Accounts)]
#[instruction(args: BuyToAlphaArgs)]
#[event_cpi]
pub struct BuyToAlpha<'info> {
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
        seeds = [b"config_data", token_holder_from_token_account.mint.as_ref()],
        bump = from_token_config_data.bump,
        constraint = from_token_config_data.max_trade_amount >= args.total_amount @ MemeshotVaultError::TradeAmountExceedsLimit
    )]
    pub from_token_config_data: Box<Account<'info, TokenConfigData>>,
    pub token_holder: UncheckedAccount<'info>,
    #[account(mut)]
    pub token_holder_from_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub token_holder_to_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(address = *config_data.load_rebalance_out_holder()?)]
    pub cloud_wallet_program: UncheckedAccount<'info>,
    #[account(
        constraint = super::is_agg_router(&*config_data.load_agg_router()?, &agg_router.key())
            @ MemeshotVaultError::InvalidProgram
    )]
    pub agg_router: UncheckedAccount<'info>,
}


pub const DEPOSIT_DISCRIMINATOR: [u8; 8] = [0xf2, 0x23, 0xc6, 0x89, 0x52, 0xe1, 0xf2, 0xb6];

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BuyToAlphaArgs {
    pub amount_in: u64,
    pub filled_amount: u64,
    pub total_amount: u64,
    pub min_amount_out: u64,
    pub deadline: u32,
    pub kind: u8,
    pub flag: u8,
    pub route_data: Vec<u8>,
}


























pub(crate) fn handler(ctx: Context<BuyToAlpha>, args: BuyToAlphaArgs) -> Result<()> {
    let clock = Clock::get()?;
    require_gt!(
        args.deadline as i64,
        clock.unix_timestamp,
        MemeshotVaultError::Expired
    );


    let signer_seeds: &[&[&[u8]]; 1] = &[&[
        b"token_holder",
        &[*ctx.accounts.config_data.load_partially_filled_remaining_pct_base()?],
    ]];
    let from_before = ctx.accounts.token_holder_from_token_account.amount;
    let to_before = ctx.accounts.token_holder_to_token_account.amount;
    let lamports_before = ctx.accounts.token_holder.lamports();



    super::invoke_route(
        ctx.accounts.agg_router.key,
        ctx.accounts.token_holder.key,
        ctx.remaining_accounts,
        0,
        args.flag,
        &args.route_data,
        signer_seeds,
    )?;
    let lamports_after_route = ctx.accounts.token_holder.lamports();
    require_gte!(
        lamports_after_route,
        lamports_before,
        MemeshotVaultError::NumberMismatch
    );
    ctx.accounts.token_holder_from_token_account.reload()?;








    let spent = from_before
        .checked_sub(ctx.accounts.token_holder_from_token_account.amount)
        .ok_or(MemeshotVaultError::MathOverflow)?;
    super::check_partially_filled(
        spent,
        args.total_amount,
        *ctx.accounts.config_data.load_expire_duration()?,
    )?;
    ctx.accounts.token_holder_to_token_account.reload()?;
    let delta = ctx
        .accounts
        .token_holder_to_token_account
        .amount
        .checked_sub(to_before)
        .ok_or(MemeshotVaultError::MathOverflow)?;
    let to_info =
        super::account_at(
            ctx.remaining_accounts,
            args.flag + 3,
        )?;
    let (_, out_before) = super::read_token_account(to_info)?;
    let lamports_mid = ctx.accounts.token_holder.lamports();




    let digest = {
        let mut preimage: Vec<u8> = Vec::with_capacity(44);
        preimage.extend_from_slice(ctx.accounts.token_holder_from_token_account.mint.as_ref());
        preimage.extend_from_slice(&args.total_amount.to_le_bytes());
        preimage.extend_from_slice(&args.deadline.to_le_bytes());
        anchor_lang::solana_program::keccak::hash(&preimage)
    };
    {
        let mut cpi_data: Vec<u8> = Vec::with_capacity(65);
        let data: &[u8];
        cpi_data.extend_from_slice(&DEPOSIT_DISCRIMINATOR);
        cpi_data.extend_from_slice(&args.amount_in.to_le_bytes());
        cpi_data.extend_from_slice(&args.filled_amount.to_le_bytes());
        cpi_data.extend_from_slice(&delta.to_le_bytes());
        {
            let kind = [args.kind];
            cpi_data.extend_from_slice(&kind);
            cpi_data.extend_from_slice(digest.as_ref());
            data = &cpi_data;
        }
        super::invoke_route(
            ctx.accounts.cloud_wallet_program.key,
            ctx.accounts.token_holder.key,
            ctx.remaining_accounts,
            args.flag,
            ctx.remaining_accounts.len() as u8,
            data,
            signer_seeds,
        )?;
    }
    let lamports_after_deposit = ctx.accounts.token_holder.lamports();
    require_gte!(
        lamports_after_deposit,
        lamports_mid,
        MemeshotVaultError::NumberMismatch
    );
    let (_, out_after) = super::read_token_account(to_info)?;




    let amount_out = out_after
        .checked_sub(out_before)
        .ok_or(MemeshotVaultError::MathOverflow)?;
    require_gte!(
        amount_out,
        args.min_amount_out,
        MemeshotVaultError::ReturnAmountNotEnough
    );
    emit_cpi!(BuyToAlphaEvent {
        amount_in: args.amount_in,
        from_mint: ctx.accounts.token_holder_from_token_account.mint,
        total_amount: args.total_amount,
        to_mint: ctx.accounts.token_holder_to_token_account.mint,
        amount_out,
        filled_amount: args.filled_amount,
        cloud_wallet_program: ctx.accounts.cloud_wallet_program.key(),
    });
    Ok(())
}
