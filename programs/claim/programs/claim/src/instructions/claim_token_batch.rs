use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;
use anchor_lang::solana_program::secp256k1_recover::secp256k1_recover;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::associated_token::{self, AssociatedToken, Create};
use anchor_spl::token_interface::{self, TokenAccount, TransferChecked};

use crate::state::*;
use crate::ClaimError;












#[event]
pub struct ClaimTokenBatchEvent {
    pub fee: u64,
    pub claim_id: u64,
    pub mints: Vec<Pubkey>,
    pub amounts: Vec<u64>,
    pub user: Pubkey,
    pub deadline: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ClaimTokenBatchArgs {
    pub fee: u64,
    pub claim_id: u64,
    pub amounts: Vec<u64>,
    pub decimals: Vec<u8>,
    pub deadline: u64,
    pub signature: [u8; 64],
    pub recovery_id: u8,
}

#[derive(Accounts)]
#[instruction(args: ClaimTokenBatchArgs)]
#[event_cpi]
pub struct ClaimTokenBatch<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub token_holder: UncheckedAccount<'info>,
    #[account(
        seeds = [b"config_data"],
        bump = config_data.bump,
        constraint = !config_data.pause @ ClaimError::Paused,
    )]
    pub config_data: Account<'info, ConfigData>,
    #[account(
        mut,
        seeds = [b"big_nonce_data", (args.claim_id / 1146752).to_le_bytes().as_ref()],
        bump,
    )]
    pub nonce_data: AccountLoader<'info, BigNonceData>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
fn batch_digest(args: &ClaimTokenBatchArgs, signer: &Pubkey, mints: &Vec<Pubkey>)
    -> anchor_lang::solana_program::hash::Hash {
    let mut message: Vec<u8> = Vec::with_capacity(169);
    message.extend_from_slice(crate::ID.as_ref());
    message.extend_from_slice(b"claim_token_batch");
    message.extend_from_slice(&args.fee.to_le_bytes());
    message.extend_from_slice(&args.claim_id.to_le_bytes());
    let amounts = args.amounts.as_slice();
    let amt_hash = || -> anchor_lang::solana_program::hash::Hash {
    let mut amt_blob = Vec::with_capacity(amounts.len() * 8);
    for amount in amounts.iter() {
        amt_blob.extend_from_slice(&amount.to_le_bytes());
    }
    hash(&amt_blob)
    }; message.extend_from_slice(amt_hash().as_ref());
    let mints = mints.as_slice();
    let mnt_hash = || -> anchor_lang::solana_program::hash::Hash {
    let mut mnt_blob = Vec::with_capacity(mints.len() * 32);
    let mut it = mints.iter();
    while let Some(mint) = it.next() {
        mnt_blob.extend_from_slice(&mint.to_bytes());
    }
    hash(&mnt_blob) }; message.extend_from_slice(mnt_hash().as_ref());
    message.extend_from_slice(signer.as_ref());
    message.extend_from_slice(&args.deadline.to_le_bytes());
    hash(&message)
}










pub(crate) fn handler<'info>(
    ctx: Context<'_, '_, 'info, 'info, ClaimTokenBatch<'info>>,
    args: &ClaimTokenBatchArgs,
) -> Result<()> {
    let deadline = args.deadline;
    require_gte!(deadline as i64, Clock::get()?.unix_timestamp, ClaimError::Expired);

    let claim_id = args.claim_id;

    if claim_id == 0 || args.fee == 0 {
        return Err(error!(ClaimError::InvalidParameter));
    }


    ctx.accounts
        .nonce_data.load_mut()?.use_claim_id(claim_id)?;

    let signer_seeds: &[&[&[u8]]] = &[&[b"token_holder", &[ctx.accounts.config_data.token_holder_bump]]];





    let mut mints: Vec<Pubkey> =
        Vec::with_capacity(ctx.remaining_accounts.len() / 4);

    let mut i = 1usize;
    let mut k = 0usize;
    while k < ctx.remaining_accounts.len() {
        let mint_info = ctx.remaining_accounts[k].clone();
        mints.push(*mint_info.key);
        let token_program = ctx.remaining_accounts[k + 1].clone();

        let token_holder_token_account = ctx.remaining_accounts[k + 2].clone();

        let recipient_ata_info = ctx.remaining_accounts[k + 3].clone(); let idx = i.wrapping_sub(1); i = i.wrapping_add(1); k = k.wrapping_add(4);

        if mint_info.key == ctx.accounts.system_program.key {






            let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.system_program.to_account_info(), Transfer { from: ctx.accounts.token_holder.to_account_info(), to: ctx.accounts.signer.to_account_info() }, signer_seeds); transfer(cpi_ctx,
                             args.amounts[idx],
            )?;
        } else {

            fn ensure_ata<'info>(ctx: &Context<'_, '_, 'info, 'info, ClaimTokenBatch<'info>>, mint_info: &AccountInfo<'info>, token_program: &AccountInfo<'info>, recipient_ata_info: &AccountInfo<'info>) -> Result<()> { let signer = ctx.accounts.signer.to_account_info();

            return if recipient_ata_info.try_borrow_mut_data()?.len() < 1 {
                associated_token::create_idempotent(CpiContext::new(
                    ctx.accounts.associated_token_program.to_account_info(), Create {
                        payer: ctx.accounts.signer.to_account_info(), associated_token: recipient_ata_info.to_account_info(),
                        authority: signer.to_account_info(), mint: mint_info.to_account_info(),
                        system_program: ctx.accounts.system_program.to_account_info(), token_program: token_program.to_account_info(),
                    },
                ))
            } else {
                let data = recipient_ata_info.try_borrow_mut_data()?; let state = TokenAccount::try_deserialize_unchecked(&mut &data[..])?;
                if state.mint != *mint_info.key || state.owner != *signer.key { Err(crate::bare_error(ClaimError::InvalidParameter)) } else { Ok(()) }
            }; } ensure_ata(&ctx, &mint_info, &token_program, &recipient_ata_info)?;
            let cpi_accounts = TransferChecked { from: token_holder_token_account.to_account_info(), to: recipient_ata_info.to_account_info(), authority: ctx.accounts.token_holder.to_account_info(), mint: mint_info.to_account_info() }; let cpi_ctx = CpiContext::new_with_signer(token_program, cpi_accounts, signer_seeds); token_interface::transfer_checked(cpi_ctx,
               args.amounts[idx],
                      args.decimals[idx],
            )?;
        }

    }
    {
        let digest = batch_digest(&args, &ctx.accounts.signer.key(), &mints);
        let recovered = secp256k1_recover(digest.as_ref(), args.recovery_id, &args.signature)
            .map_err(|_| ClaimError::InvalidParameter)?;

        if !ctx.accounts.config_data.signers.contains(&recovered.to_bytes()) {
            return Err(error!(ClaimError::InvalidSignature));
        }
    }
    emit_cpi!(ClaimTokenBatchEvent {
        fee: args.fee,
        claim_id,
        amounts: args.amounts.clone(),
        mints,
        user: ctx.accounts.signer.key(),
        deadline,
    });

    Ok(())

}



fn require_params(claim_id: u64, fee: u64, filename: &'static str, line: u32) -> Result<()> {
    if claim_id == 0 || fee == 0 {
        return Err(Error::from(anchor_lang::error::AnchorError {
            error_name: ClaimError::InvalidParameter.name(),
            error_code_number: ClaimError::InvalidParameter.into(),
            error_msg: ClaimError::InvalidParameter.to_string(),
            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                anchor_lang::error::Source { filename, line },
            )),
            compared_values: None,
        }));
    }
    Ok(())
}

fn expired_error(filename: &'static str, line: u32) -> Error {
    Error::from(anchor_lang::error::AnchorError {
        error_name: ClaimError::Expired.name(),
        error_code_number: ClaimError::Expired.into(),
        error_msg: ClaimError::Expired.to_string(),
        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
            anchor_lang::error::Source { filename, line },
        )),
        compared_values: None,
    })
}
