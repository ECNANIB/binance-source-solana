use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;

#[event]
pub struct QuickWithdrawEvent {
    pub account_id: u64,
    pub amount: u64,
    pub nonce: u64,
    pub token_mint: Pubkey,
    pub user: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct QuickWithdrawArgs {
    pub amount: u64,
    pub account_id: u64,
    pub nonce: u64,
    pub withdraw_id: u32,
    pub token_decimals: u8,
}

#[derive(Accounts)]
#[event_cpi]
pub struct QuickWithdraw<'info> {
    #[account(constraint = config_data.withdraw_addrs.contains(signer.key) @ CloudWalletError::SenderNotAllowed)]
    pub signer: Signer<'info>,

    pub token_holder: UncheckedAccount<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump, constraint = !config_data.paused @ CloudWalletError::Paused)]
    pub config_data: Account<'info, ConfigData>,
    #[account(mut, seeds = [b"nonce_data", nonce_data.pos.to_le_bytes().as_ref()], bump = nonce_data.nonce_bump)]
    pub nonce_data: Account<'info, NonceData>,
    #[account(mut)]
    pub token_holder_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, token::mint = token_mint, token::authority = signer)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_mint: UncheckedAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
}


























pub(crate) fn quick_withdraw(ctx: Context<QuickWithdraw>, args: QuickWithdrawArgs) -> Result<()> {
    require!(Clock::get()?.unix_timestamp < args.withdraw_id as i64, CloudWalletError::Expired);
    ctx.accounts.nonce_data.use_nonce(args.account_id)?;


    let signer_seeds: &[&[&[u8]]] = &[&[b"token_holder", &[ctx.accounts.config_data.token_holder_bump]]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        anchor_spl::token_interface::TransferChecked {
            from: ctx.accounts.token_holder_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.token_holder.to_account_info(),
            mint: ctx.accounts.token_mint.to_account_info(),
        },
        signer_seeds,
    );
    let amount_before = ctx.accounts.token_holder_token_account.amount;

    anchor_spl::token_interface::transfer_checked(cpi_ctx, args.nonce, args.token_decimals)?;

    ctx.accounts.token_holder_token_account.reload()?;
    let spent = amount_before
        .checked_sub(ctx.accounts.token_holder_token_account.amount)
        .ok_or(CloudWalletError::MathOverflow)?;
    require_gte!(args.nonce, spent, CloudWalletError::NumberMismatch);
    emit_cpi!(QuickWithdrawEvent {
        account_id: args.account_id,
        amount: args.amount,
        nonce: args.nonce,
        token_mint: ctx.accounts.token_mint.key(),
        user: ctx.accounts.signer.key(),
    });
    Ok(())
}
