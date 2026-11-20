use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;
use anchor_lang::solana_program::secp256k1_recover::secp256k1_recover;
use anchor_lang::system_program::{transfer, Transfer};

use crate::state::*;
use crate::ClaimError;

#[event]
pub struct ClaimEvent {
    pub fee: u64,
    pub claim_id: u64,
    pub mint: Option<Pubkey>,
    pub amount: u64,
    pub user: Pubkey,
    pub deadline: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ClaimArgs {
    pub fee: u64,
    pub claim_id: u64,
    pub amount: u64,
    pub deadline: u64,
    pub signature: [u8; 64],
    pub recovery_id: u8,
}

#[derive(Accounts)]
#[instruction(args: ClaimArgs)]
#[event_cpi]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
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
    pub system_program: Program<'info, System>,
}

pub(crate) fn handler(ctx: Context<Claim>, args: ClaimArgs) -> Result<()> {











    require_gte!(args.deadline as i64, Clock::get()?.unix_timestamp, ClaimError::Expired);



    let mut invalid = false; if args.claim_id < 1 { invalid = true; } else if args.fee < 1 { invalid = true; } if invalid {
        return Err(error!(ClaimError::InvalidParameter));
    }

    ctx.accounts.nonce_data.load_mut()?.use_claim_id(args.claim_id)?;

    let mut message: Vec<u8> = Vec::with_capacity(101);
    message.extend_from_slice(crate::ID.as_ref());
    message.extend_from_slice(b"claim");
    message.extend_from_slice(&args.fee.to_le_bytes());
    message.extend_from_slice(&args.claim_id.to_le_bytes());
    message.extend_from_slice(&args.amount.to_le_bytes());
    message.extend_from_slice(&ctx.accounts.user.key.to_bytes());
    message.extend_from_slice(&args.deadline.to_le_bytes());
    let digest = hash(&message);

    let recovered = secp256k1_recover(&digest.to_bytes(), args.recovery_id, &args.signature)
        .map_err(|_| ClaimError::InvalidParameter)?;
    {
        let config_data = &ctx.accounts.config_data;
        let signers = config_data.signers.as_slice();
        let signer_bytes = recovered.to_bytes();


        require!(
            signers.iter().any(|s| s == &signer_bytes),
            ClaimError::InvalidSignature
        );
    }
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"token_holder",
        &[ctx.accounts.config_data.token_holder_bump],
    ]];
    let cpi_program = ctx.accounts.system_program.to_account_info();
    let cpi_accounts = Transfer {
        from: ctx.accounts.token_holder.to_account_info(),
        to: ctx.accounts.user.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
    transfer(cpi_ctx, args.amount)?;

    emit_cpi!(ClaimEvent {
        fee: args.fee,
        claim_id: args.claim_id,
        mint: None,
        amount: args.amount,
        user: *ctx.accounts.user.key,
        deadline: args.deadline,
    });

    Ok(())
}







fn param_error(filename: &'static str, line: u32) -> Error {
    Error::from(anchor_lang::error::AnchorError {
        error_name: ClaimError::InvalidParameter.name(),
        error_code_number: ClaimError::InvalidParameter.into(),
        error_msg: ClaimError::InvalidParameter.to_string(),
        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
            anchor_lang::error::Source { filename, line },
        )),
        compared_values: None,
    })
}









fn require_known(ok: bool, filename: &'static str, line: u32) -> Result<()> {
    if ok {
        return Ok(());
    }
    Err(Error::from(anchor_lang::error::AnchorError {
        error_name: ClaimError::InvalidSignature.name(),
        error_code_number: ClaimError::InvalidSignature.into(),
        error_msg: ClaimError::InvalidSignature.to_string(),
        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
            anchor_lang::error::Source { filename, line },
        )),
        compared_values: None,
    }))
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
