use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use super::delta_window::{DeltaSnapshot, DeltaWindowSettled, TOKEN_DELTA_SEED};















#[derive(Accounts)]
pub struct BeginTokenDeltaWindow<'info> {
    pub authority: Signer<'info>,

    pub watched: UncheckedAccount<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    #[account(
        init,
        payer = rent_payer,
        space = 8 + 32 + 32 + 8 + 1,
        seeds = [TOKEN_DELTA_SEED, watched.key().as_ref()],
        bump
    )]
    pub snapshot: Account<'info, DeltaSnapshot>,
    #[account(mut)]
    pub rent_payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(address = anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instructions: UncheckedAccount<'info>,

}





pub(crate) fn begin_token_delta_window<'info>(
    ctx: Context<'_, '_, '_, 'info, BeginTokenDeltaWindow<'info>>,
) -> Result<()> { fn begin_token_delta_window_inner<'info>(ctx: Context<'_, '_, '_, 'info, BeginTokenDeltaWindow<'info>>) -> Result<()> {
    let accounts = ctx.accounts;
    let bump = ctx.bumps.snapshot;
    super::delta_window::require_top_level()?;
    let authority = accounts.authority.key();
    let mint = accounts.mint.key();
    let token_program = accounts.token_program.key();

    require_keys_eq!(*accounts.mint.to_account_info().owner, token_program, crate::RouterError::TokenProgramMismatch);


    let start_bala03j = if *accounts.watched.owner == anchor_lang::solana_program::system_program::ID && accounts.watched.data_is_empty() {






        require_keys_eq!(accounts.watched.key(), anchor_spl::associated_token::get_associated_token_address_with_program_id(&authority, &mint, &token_program), crate::RouterError::WatchedNotAssociatedTokenAccount);
        0
    } else {




        require_keys_eq!(*accounts.watched.owner, token_program, crate::RouterError::TokenProgramMismatch);
        let token = anchor_spl::token_interface::TokenAccount::try_deserialize(&mut &accounts.watched.try_borrow_data()?[..])?;
        require_keys_eq!(token.owner, authority, crate::RouterError::WatchedAuthorityMismatch);
        require_keys_eq!(token.mint, mint, crate::RouterError::WatchedMintMismatch);
        token.amount
    };
    super::delta_window::require_matching_settle(
        accounts.instructions.as_ref(),
        <crate::instruction::SettleTokenDeltaWindow as anchor_lang::Discriminator>::DISCRIMINATOR,
        &accounts.snapshot.key(),
    )?;
    let s = &mut accounts.snapshot;
    s.authority = authority;
    s.watched = mint;
    s.start_bala03j = start_bala03j;
    s.bump = bump;
    Ok(()) }
    begin_token_delta_window_inner(ctx) }



#[event_cpi]
#[derive(Accounts)]
pub struct SettleTokenDeltaWin00y<'info> {
    #[account(
        mut,
        close = rent_receiver,
        seeds = [TOKEN_DELTA_SEED, watched.key().as_ref()],
        bump = snapshot.bump
    )]
    pub snapshot: Account<'info, DeltaSnapshot>,

    pub authority: Signer<'info>,
    #[account(mut)]
    pub watched: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub recipient: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub rent_receiver: UncheckedAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
}







pub(crate) fn settle_token_delta_window<'info>(
    ctx: Context<'_, '_, '_, 'info, SettleTokenDeltaWin00y<'info>>,
    min_delta: u64,
) -> Result<()> {
    fn settle_token_delta_window_inner<'info>(ctx: Context<'_, '_, '_, 'info, SettleTokenDeltaWin00y<'info>>, min_delta: u64) -> Result<()> {
        let authority_bump = ctx.bumps.event_authority;




    require_keys_eq!(ctx.accounts.snapshot.authority, ctx.accounts.authority.key(), crate::RouterError::SnapshotAuthorityMismatch);





    require_keys_eq!(ctx.accounts.watched.owner, ctx.accounts.authority.key(), crate::RouterError::WatchedAuthorityMismatch);




    require_keys_eq!(ctx.accounts.mint.key(), ctx.accounts.snapshot.watched, crate::RouterError::SnapshotMintMismatch);
    require_keys_eq!(ctx.accounts.watched.mint, ctx.accounts.snapshot.watched, crate::RouterError::SnapshotMintMismatch);
    let token_program_key = ctx.accounts.token_program.key();
    require_keys_eq!(*ctx.accounts.mint.to_account_info().owner, token_program_key, crate::RouterError::TokenProgramMismatch);




    require_keys_eq!(*ctx.accounts.watched.to_account_info().owner, token_program_key, crate::RouterError::TokenProgramMismatch);




    require_keys_eq!(ctx.accounts.recipient.mint, ctx.accounts.snapshot.watched, crate::RouterError::InvalidDeltaRecipient);




    require_keys_neq!(ctx.accounts.recipient.key(), ctx.accounts.watched.key(), crate::RouterError::SelfForward);





    let now = ctx.accounts.watched.amount;
    let start_bala03j = ctx.accounts.snapshot.start_bala03j;
    let delta = now
        .checked_sub(start_bala03j)
        .ok_or(error!(crate::RouterError::BalanceDecreased))?;
    require_neq!(delta, 0, crate::RouterError::NothingToForward);
    require_gte!(delta, min_delta, crate::RouterError::DeltaBelowMinimum);


    let before = ctx.accounts.recipient.amount;
    anchor_spl::token_2022::transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token_2022::TransferChecked {
                from: ctx.accounts.watched.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.recipient.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        delta,
        ctx.accounts.mint.decimals,
    )?;


    ctx.accounts.recipient.reload()?;
    let received = ctx
        .accounts
        .recipient
        .amount
        .checked_sub(before)
        .ok_or(error!(crate::RouterError::InvalidDeltaRecipient))?;
    emit_cpi!(DeltaWindowSettled {
        authority: ctx.accounts.authority.key(), watched: ctx.accounts.watched.key(),
        mint: Some(ctx.accounts.snapshot.watched), start_bala03j: ctx.accounts.snapshot.start_bala03j,
        delta, recipient: ctx.accounts.recipient.owner, received,
    });
    Ok(()) }
    settle_token_delta_window_inner(ctx, min_delta) }
