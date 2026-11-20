use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash;
use anchor_lang::solana_program::secp256k1_recover::secp256k1_recover;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{self, TokenAccount, TokenInterface, TransferChecked};
use crate::state::ConfigData;
use crate::DepositAgentError;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CollectToArgs {
    pub id: u128,
    pub amount: u64,
    pub deadline: u32,
    pub decimals: u8,
    pub recovery_id: u8,
    pub signature: [u8; 64],
}

#[event]
pub struct CollectToEvent {
    pub id: u128,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub to: Pubkey,
}

#[event_cpi]
#[derive(Accounts)]
#[instruction(args: CollectToArgs)]
pub struct CollectTo<'info> {
    #[account(mut, constraint = config_data.withdraw_tos.contains(&signer.key()) @ DepositAgentError::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump, constraint = !config_data.paused @ DepositAgentError::Paused)]
    pub config_data: Account<'info, ConfigData>,

    #[account(
        seeds = [crate::TOKEN_HOLDER_SEED, &args.id.to_le_bytes()],
        bump,
    )]
    pub token_holder: UncheckedAccount<'info>,

    #[account(mut)]
    pub token_holder_token_account: UncheckedAccount<'info>,

    #[account(constraint = config_data.program_addr.is_some_and(|c| c == *to.key) @ DepositAgentError::InvalidWithdrawTo)]
    pub to: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = token_mint,
        associated_token::authority = to,
    )]
    pub to_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_mint: UncheckedAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}





















fn check_deadline(deadline: u32) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    if (deadline as i64) < now {
        let code = DepositAgentError::Expired;
        return Err(Error::from(code).with_values((deadline, now)));
    }
    Ok(())
}
pub(crate) fn handler(ctx: Context<CollectTo>, args: CollectToArgs) -> Result<()> {

    check_deadline(args.deadline)?;
    let digest = {
        let mut message_data = Vec::with_capacity(142);
        message_data.extend_from_slice(&ctx.accounts.config_data.expiration.to_le_bytes());
        message_data.extend_from_slice(crate::ID.as_ref());
        message_data.extend_from_slice(b"collect_to");
        message_data.extend_from_slice(&args.id.to_le_bytes());
        message_data.extend_from_slice(ctx.accounts.token_mint.key.as_ref());
        message_data.extend_from_slice(&args.amount.to_le_bytes());
        message_data.extend_from_slice(ctx.accounts.to.key.as_ref());
        message_data.extend_from_slice(&args.deadline.to_le_bytes());
        hash::hash(&message_data) };
    let found = {
        let recovered = secp256k1_recover(&digest.to_bytes(), args.recovery_id, &args.signature).map_err(|_| DepositAgentError::InvalidSignature)?;
        ctx.accounts.config_data.secp256k1_pubkeys.contains(&recovered.to_bytes()) };
    require!(
        found,
        DepositAgentError::InvalidSignature
    );

    let seeds: &[&[u8]] = &[
        crate::TOKEN_HOLDER_SEED,
        &args.id.to_le_bytes(),
        &[ctx.bumps.token_holder],
    ];
    let signer_seeds: &[&[&[u8]]] = &[seeds];
    let cpi_ctx =
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.token_holder_token_account.to_account_info(),
                to: ctx.accounts.to_token_account.to_account_info(),
                authority: ctx.accounts.token_holder.to_account_info(),
                mint: ctx.accounts.token_mint.to_account_info(),
            },
            signer_seeds,
        );
    token_interface::transfer_checked(cpi_ctx, args.amount,
        args.decimals,
    )?;

    emit_cpi!(CollectToEvent {
        id: args.id,
        token_mint: ctx.accounts.token_mint.key(),
        amount: args.amount,
        to: ctx.accounts.to.key(),
    });
    Ok(())
    }
