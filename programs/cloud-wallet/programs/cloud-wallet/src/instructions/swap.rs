use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;

#[event]
pub struct SwapEvent {
    pub nonce: u64,
    pub from_mint: Pubkey,
    pub amount_in: u64,
    pub to_mint: Pubkey,
    pub amount_out: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SwapArgs {
    pub nonce: u64,
    pub amount: u64,
    pub min_amount_out: u64,
    pub deadline: u32,
    pub recovery_id: u8,
    pub signature: [u8; 64],
    pub data: Vec<u8>,
}

#[derive(Accounts)]
#[instruction(args: SwapArgs)]
#[event_cpi]
pub struct Swap<'info> {
    #[account(mut, constraint = config_data.rebalance_addrs.contains(signer.key) @ CloudWalletError::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump, constraint = !config_data.paused @ CloudWalletError::Paused)]
    pub config_data: Box<Account<'info, ConfigData>>,
    #[account(mut, seeds = [b"big_nonce_data", (args.nonce / BIG_NONCE_BUCKET).to_le_bytes().as_ref()], bump)]
    pub nonce_data: AccountLoader<'info, BigNonceData>,
    #[account(seeds = [b"token_holder"], bump = config_data.token_holder_bump)]

    pub token_holder: UncheckedAccount<'info>,
    #[account(mut)]
    pub token_holder_from_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, token::authority = token_holder)]
    pub token_holder_to_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(constraint = crate::instructions::check_router(&config_data, agg_router.key) @ CloudWalletError::InvalidProgram)]

    pub agg_router: UncheckedAccount<'info>,
}

































pub(crate) fn swap(ctx: Context<Swap>, args: SwapArgs) -> Result<()> {
    let deadline = args.deadline;
    require_gte!(
        deadline as i64,
        Clock::get()?.unix_timestamp,
        CloudWalletError::Expired
    );
    let nonce = { let mut nd = ctx.accounts.nonce_data.load_mut()?; let v = args.nonce; nd.use_nonce(v)?; v };
    let (amount, min_amount_out) = { let mut msg = Vec::with_capacity(136);
        msg.extend_from_slice(&ctx.accounts.config_data.chain_id.to_le_bytes());
        msg.extend_from_slice(crate::ID.as_ref());
        msg.extend_from_slice(b"swap");
        msg.extend_from_slice(&nonce.to_le_bytes());
        msg.extend_from_slice(ctx.accounts.token_holder_from_token_account.mint.as_ref());
        let amount = args.amount; msg.extend_from_slice(&amount.to_le_bytes());
        msg.extend_from_slice(ctx.accounts.token_holder_to_token_account.mint.to_bytes().as_ref());
        let min_amount_out = args.min_amount_out; msg.extend_from_slice(&min_amount_out.to_le_bytes());
        msg.extend_from_slice(&deadline.to_le_bytes());
        let recovered = { let hash = anchor_lang::solana_program::hash::hash(&msg);
            anchor_lang::solana_program::secp256k1_recover::secp256k1_recover(
                hash.to_bytes().as_ref(),
                args.recovery_id,
                &args.signature,
            )
            .map_err(|_| CloudWalletError::InvalidSignature)? };
        require!(
            ctx.accounts
                .config_data
                .secp256k1_pubkeys
                .contains(&recovered.to_bytes()),
            CloudWalletError::InvalidSignature
        );
        (amount, min_amount_out)
    };

    let signer_seeds: &[&[&[u8]]; 1] = &[&[b"token_holder", &[ctx.accounts.config_data.token_holder_bump]]];

    let amount_in_before = ctx
        .accounts
        .token_holder_from_token_account
        .amount;
    let amount_out_before = ctx
        .accounts
        .token_holder_to_token_account
        .amount;
    let lamports_before = ctx.accounts.token_holder.lamports();

    crate::instructions::invoke_router(
        ctx.accounts.agg_router.key,
        ctx.accounts.token_holder.key,
        ctx.remaining_accounts,
        ctx.remaining_accounts.len() as u8,
        &args.data,
        signer_seeds,
    )?;

    let lamports_after = ctx.accounts.token_holder.lamports();
    require_gte!(
        lamports_after,
        lamports_before,
        CloudWalletError::NumberMismatch
    );
    reload_token_account(&mut ctx.accounts.token_holder_from_token_account)?;
    let amount_in = amount_in_before
        .checked_sub(
            ctx.accounts
                .token_holder_from_token_account
                .amount,
        )
        .ok_or(CloudWalletError::MathOverflow)?;
    crate::instructions::check_partially_filled(
        amount_in,
        amount,
        ctx.accounts.config_data.partially_filled_remaining_pct_base,
    )?;
    match {
        reload_token_account(&mut ctx.accounts.token_holder_to_token_account)?;
        let amount_out_after = ctx.accounts.token_holder_to_token_account.amount;
        amount_out_after
    }.checked_sub(amount_out_before)
        .ok_or(CloudWalletError::MathOverflow)?
    {
        v if v < min_amount_out => {
            Err(error!(CloudWalletError::ReturnAmountNotEnough).with_values((v, min_amount_out)))
        }
        amount_out => {
            emit_cpi!(SwapEvent {
                amount_in: amount,
                nonce,
                from_mint: ctx.accounts.token_holder_from_token_account.mint,
                to_mint: ctx.accounts.token_holder_to_token_account.mint,
                amount_out,
            });
            Ok(())
        }
    }
}

pub fn invoke_router<'info>(
    router: &Pubkey,
    token_holder: &Pubkey,
    accounts: &[AccountInfo<'info>],
    account_len: u8,
    data: &[u8],
    signer_seeds: &[&[&[u8]]; 1],
) -> Result<()> {
    let mut metas: Vec<anchor_lang::solana_program::instruction::AccountMeta> =
        Vec::with_capacity(account_len as usize);
    let mut seen: std::collections::HashMap<&'info Pubkey, AccountInfo<'info>> =
        std::collections::HashMap::new();
    for i in 0..account_len {
        let acc = accounts
            .get(i as usize)
            .ok_or(CloudWalletError::AddrNotExists)?;
        metas.push(anchor_lang::solana_program::instruction::AccountMeta {
            pubkey: *acc.key,
            is_signer: acc.is_signer || *acc.key == *token_holder,
            is_writable: acc.is_writable,
        });
        if seen.contains_key(&acc.key) {
            continue;
        }
        seen.insert(acc.key, acc.clone());
    }
    let infos: Vec<AccountInfo<'info>> = seen.values().cloned().collect();
    anchor_lang::solana_program::program::invoke_signed(
        &anchor_lang::solana_program::instruction::Instruction {
            program_id: *router,
            accounts: metas,
            data: data.to_vec(),
        },
        &infos,
        signer_seeds,
    )?;
    Ok(())
}

fn reload_token_account(account: &mut InterfaceAccount<TokenAccount>) -> Result<()> {
    account.reload()
}
