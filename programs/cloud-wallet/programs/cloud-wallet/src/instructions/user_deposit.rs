use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;

#[event]
pub struct UserDepositEvent {
    pub deposit_id: u64,
    pub account_id: u64,
    pub token_mint: Pubkey,
    pub amount: u64,
}













#[derive(Accounts)]
#[instruction(args: UserDepositArgs)]
#[event_cpi]
pub struct UserDeposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(seeds = [b"config_data"], bump = config.bump, constraint = !config.paused @ CloudWalletError::Paused)]
    pub config: Box<Account<'info, ConfigData>>,
    #[account(init, payer = signer, space = EMPTY_DATA_SPACE, seeds = [b"user_deposit_id", args.deposit_id.to_le_bytes().as_ref()], bump)]
    pub deposit_id_account: Account<'info, EmptyData>,
    #[account(seeds = [b"token_holder"], bump = config.token_holder_bump)]

    pub token_holder: UncheckedAccount<'info>,
    #[account(mut, token::mint = token_mint, token::authority = token_holder)]
    pub token_holder_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_mint: UncheckedAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}




























pub(crate) fn user_deposit(ctx: Context<UserDeposit>, args: UserDepositArgs) -> Result<()> {
    {
        let mut msg = Vec::with_capacity(113);
        msg.extend_from_slice(&ctx.accounts.config.chain_id.to_le_bytes());
        msg.extend_from_slice(crate::ID.as_ref());
        msg.extend_from_slice(b"user_deposit");
        msg.extend_from_slice(&args.deposit_id.to_le_bytes());
        msg.extend_from_slice(&args.account_id.to_le_bytes());
        msg.extend_from_slice(ctx.accounts.token_mint.key().to_bytes().as_ref());
        msg.extend_from_slice(&args.amount.to_le_bytes());
        msg.extend_from_slice(&args.token_decimals.to_le_bytes());
        msg.extend_from_slice(&args.expired_at.to_le_bytes());
        let recovered;
        let h32 = anchor_lang::solana_program::hash::hash(&msg);
        recovered = anchor_lang::solana_program::secp256k1_recover::secp256k1_recover(
            h32.to_bytes().as_ref(),
            args.recovery_id,
            &args.signature,
        )
        .map_err(|_| CloudWalletError::InvalidSignature)?;
        let found = {
            let keys = ctx.accounts.config.withdraw_secp256k1_pubkeys.as_slice();
            let rb = recovered.to_bytes();
            keys.iter().any(|k| *k == rb)
        };
        require!(found, CloudWalletError::InvalidSignature);
    }

    let token_decimals = args.token_decimals;
    let amount = args.amount;
    let account_id = args.account_id;
    let deposit_id = args.deposit_id;
    require_gt!(
        args.expired_at as i64,
        Clock::get()?.unix_timestamp,
        CloudWalletError::Expired
    );
    require_gt!(amount, 0, CloudWalletError::NumberMismatch);
    require_gt!(account_id, 0, CloudWalletError::NumberMismatch);
    require_gt!(deposit_id, 0, CloudWalletError::NumberMismatch);
    require_gt!(token_decimals, 0, CloudWalletError::NumberMismatch);
    let program = ctx.accounts.token_program.to_account_info();
    let accounts = anchor_spl::token_interface::TransferChecked {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.token_holder_token_account.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
        mint: ctx.accounts.token_mint.to_account_info(),
    };
    let cpi = CpiContext::new(program, accounts);

    let before = ctx.accounts.token_holder_token_account.amount;
    anchor_spl::token_interface::transfer_checked(
        cpi,
        args.amount, token_decimals,
    )?;
    ctx.accounts.token_holder_token_account.reload()?;
    let received = ctx
        .accounts
        .token_holder_token_account
        .amount
        .checked_sub(before)
        .ok_or(CloudWalletError::MathOverflow)?;
    require_gt!(received, 0, CloudWalletError::NumberMismatch);
    emit_cpi!(UserDepositEvent {
        deposit_id: args.deposit_id,
        account_id: args.account_id,
        token_mint: ctx.accounts.token_mint.key(),
        amount: received,
    });
    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UserDepositArgs {
    pub deposit_id: u64,
    pub account_id: u64,
    pub amount: u64,
    pub token_decimals: u8,
    pub expired_at: u32,
    pub recovery_id: u8,
    pub signature: [u8; 64],
}
