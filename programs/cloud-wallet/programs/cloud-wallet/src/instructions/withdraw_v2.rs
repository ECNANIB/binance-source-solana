use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;
use crate::instructions::withdraw::{WithdrawArgs, WithdrawEvent};














#[derive(Accounts)]
#[instruction(args: WithdrawArgs)]
#[event_cpi]
pub struct WithdrawV2<'info> {
    #[account(mut, constraint = config_data.rebalance_addrs.contains(&signer.key()) @ CloudWalletError::SenderNotAllowed)]
    pub signer: Signer<'info>,

    pub token_holder: UncheckedAccount<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump, constraint = !config_data.paused @ CloudWalletError::Paused)]
    pub config_data: Account<'info, ConfigData>,
    #[account(init, payer = signer, space = EMPTY_DATA_SPACE, seeds = [b"withdraw_v2_nonce_data", args.nonce.to_le_bytes().as_ref()], bump)]
    pub nonce_data: Account<'info, EmptyData>,
    #[account(mut)]

    pub token_holder_token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_mint: UncheckedAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}





















pub(crate) fn withdraw_v2(ctx: Context<WithdrawV2>, args: WithdrawArgs) -> Result<()> {
    require_gte!(
        args.withdraw_id as i64,
        Clock::get()?.unix_timestamp,
        CloudWalletError::Expired
    );
    let mut msg = Vec::with_capacity(135);
    msg.extend_from_slice(&ctx.accounts.config_data.chain_id.to_le_bytes());
    msg.extend_from_slice(crate::ID.as_ref());
    msg.extend_from_slice(b"withdraw_v2");
    msg.extend_from_slice(&args.nonce.to_le_bytes());
    msg.extend_from_slice(ctx.accounts.token_mint.key().as_ref());
    msg.extend_from_slice(&args.amount.to_le_bytes());
    msg.extend_from_slice((*ctx.accounts.user_token_account).owner.to_bytes().as_ref());
    msg.extend_from_slice(&args.withdraw_id.to_le_bytes());
    let ok = {
        let h32 = anchor_lang::solana_program::hash::hash(&msg);
        let recovered = anchor_lang::solana_program::secp256k1_recover::secp256k1_recover(
            h32.to_bytes().as_ref(),
            args.recovery_id,
            &args.signature,
        ).map_err(|_| CloudWalletError::InvalidSignature)?;
        ctx.accounts.config_data.deposit_secp256k1_pubkeys.contains(&recovered.to_bytes()) };
    require!(ok, CloudWalletError::InvalidSignature);
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
    anchor_spl::token_interface::transfer_checked(cpi_ctx, args.amount, args.token_decimals)?;
    emit_cpi!(WithdrawEvent {
        nonce: args.nonce,
        amount: args.amount,
        token_mint: ctx.accounts.token_mint.key(),
        user: (*ctx.accounts.user_token_account).owner,
    });
    Ok(())
}
