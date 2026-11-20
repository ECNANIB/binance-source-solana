use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;

#[event]
pub struct WithdrawEvent {
    pub nonce: u64,
    pub amount: u64,
    pub token_mint: Pubkey,
    pub user: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct WithdrawArgs {
    pub nonce: u64,
    pub amount: u64,
    pub withdraw_id: u32,
    pub token_decimals: u8,
    pub recovery_id: u8,
    pub signature: [u8; 64],
}

#[derive(Accounts)]
#[instruction(args: WithdrawArgs)]
#[event_cpi]
pub struct Withdraw<'info> {
    #[account(mut, constraint = config_data.rebalance_addrs.contains(&signer.key()) @ CloudWalletError::SenderNotAllowed)]
    pub signer: Signer<'info>,

    pub token_holder: UncheckedAccount<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump, constraint = !config_data.paused @ CloudWalletError::Paused)]
    pub config_data: Account<'info, ConfigData>,
    #[account(mut, seeds = [b"big_nonce_data", (args.nonce / BIG_NONCE_BUCKET).to_le_bytes().as_ref()], bump)]
    pub nonce_data: AccountLoader<'info, BigNonceData>,
    #[account(mut)]

    pub token_holder_token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(seeds = [b"withdraw_allowlist", token_mint.key().as_ref()], bump)]
    pub withdraw_allowlist: Account<'info, EmptyData>,
    #[account(seeds = [b"withdraw_config_flag", token_mint.key().as_ref()], bump)]

    pub withdraw_config_flag: UncheckedAccount<'info>,
    #[account(seeds = [b"withdraw_config", token_mint.key().as_ref(), user_token_account.owner.as_ref()], bump)]

    pub withdraw_config: UncheckedAccount<'info>,

    pub token_mint: UncheckedAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
}











































pub(crate) fn withdraw(ctx: Context<Withdraw>, args: WithdrawArgs) -> Result<()> {
    let config_data = &mut ctx.accounts.config_data;
    require_gte!(
        args.withdraw_id as i64,
        Clock::get()?.unix_timestamp,
        CloudWalletError::Expired
    );
    {
        let flag = ctx.accounts.withdraw_config_flag.data.borrow();
        if !flag.is_empty() && flag[0] == 5u8 {
            let config = ctx.accounts.withdraw_config.data.borrow();
            if config.first().copied() != Some(5u8) {
                return Err(error!(CloudWalletError::Rejected).with_pubkeys((
                    ctx.accounts.token_mint.key(),
                    (*ctx.accounts.user_token_account).owner,
                )));
            }
        }
    }
    ctx.accounts.nonce_data.load_mut()?.use_nonce(args.nonce)?;
    {
        let mut msg = Vec::with_capacity(132);
        msg.extend_from_slice(&config_data.chain_id.to_le_bytes());
        msg.extend_from_slice(crate::ID.as_ref());
        msg.extend_from_slice(b"withdraw");
        msg.extend_from_slice(&args.nonce.to_le_bytes());
        msg.extend_from_slice(ctx.accounts.token_mint.key().as_ref());
        msg.extend_from_slice(&args.amount.to_le_bytes());
        msg.extend_from_slice((*ctx.accounts.user_token_account).owner.to_bytes().as_ref());
        msg.extend_from_slice(&args.withdraw_id.to_le_bytes());
        let hash = anchor_lang::solana_program::hash::hash(&msg);
        let recovered = anchor_lang::solana_program::secp256k1_recover::secp256k1_recover(
            hash.to_bytes().as_ref(),
            args.recovery_id,
            &args.signature,
        )
        .map_err(|_| CloudWalletError::InvalidSignature)?;
        require!(
            config_data
                .secp256k1_pubkeys
                .contains(&recovered.to_bytes()),
            CloudWalletError::InvalidSignature
        );
    }
    let signer_seeds: &[&[&[u8]]; 1] = &[&[b"token_holder", &[config_data.token_holder_bump]]];
    let prog = ctx.accounts.token_program.to_account_info();
    let accts = anchor_spl::token_interface::TransferChecked {
        from: ctx.accounts.token_holder_token_account.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.token_holder.to_account_info(),
        mint: ctx.accounts.token_mint.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(prog, accts, signer_seeds);
    anchor_spl::token_interface::transfer_checked(cpi_ctx, args.amount, args.token_decimals)?;

    emit_cpi!(WithdrawEvent {
        nonce: args.nonce,
        amount: args.amount,
        token_mint: ctx.accounts.token_mint.key(),
        user: (*ctx.accounts.user_token_account).owner,
    });

    Ok(())
}
