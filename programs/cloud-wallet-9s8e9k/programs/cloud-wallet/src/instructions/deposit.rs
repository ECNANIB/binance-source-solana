use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;

#[event]
pub struct DepositEvent {
    pub account_id: u64,
    pub deposit_id: u64,
    pub amount: u64,
    pub token_mint: Pubkey,
    pub token_decimals: u8,
    pub user: Pubkey,
}

#[derive(Accounts)]
#[event_cpi]
pub struct Deposit<'info> {
    pub signer: Signer<'info>,
    #[account(seeds = [b"config_data"], bump = config.bump, constraint = !config.paused @ CloudWalletError::Paused)]
    pub config: Box<Account<'info, ConfigData>>,
    #[account(seeds = [b"token_holder"], bump = config.token_holder_bump)]

    pub token_holder: UncheckedAccount<'info>,
    #[account(mut, token::mint = token_mint, token::authority = token_holder)]
    pub token_holder_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_mint: UncheckedAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DepositArgs {
    pub account_id: u64,
    pub deposit_id: u64,
    pub amount: u64,
    pub token_decimals: u8,
    pub user: Pubkey,
}
























pub(crate) fn deposit(ctx: Context<Deposit>, args: DepositArgs) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        anchor_spl::token_interface::TransferChecked {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.token_holder_token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
            mint: ctx.accounts.token_mint.to_account_info(),
        },
    );
    let before = ctx.accounts.token_holder_token_account.amount;
    anchor_spl::token_interface::transfer_checked(cpi_ctx, args.amount, args.token_decimals)?;
    ctx.accounts.token_holder_token_account.reload()?;
    let amount = ctx
        .accounts
        .token_holder_token_account
        .amount
        .checked_sub(before)
        .ok_or(CloudWalletError::MathOverflow)?;
    require_gt!(amount, 0, CloudWalletError::NumberMismatch);
    emit_cpi!(DepositEvent {
        account_id: args.account_id,
        deposit_id: args.deposit_id,
        amount,
        token_mint: ctx.accounts.token_mint.key(),
        token_decimals: args.token_decimals,
        user: args.user,
    });
    Ok(())
}
