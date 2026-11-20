use anchor_lang::prelude::*;

use super::delta_window::{DeltaSnapshot, DeltaWindowSettled, LAMPORT_DELTA_SEED};














#[derive(Accounts)]
pub struct BeginLamportDeltaWindow<'info> {
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = rent_payer,
        space = 8 + 32 + 32 + 8 + 1,
        seeds = [LAMPORT_DELTA_SEED, authority.key().as_ref()],
        bump
    )]
    pub snapshot: Account<'info, DeltaSnapshot>,
    #[account(mut)]
    pub rent_payer: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(address = anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instructions: UncheckedAccount<'info>,

}


pub(crate) fn begin_lamport_delta_window<'info>(
    ctx: Context<'_, '_, '_, 'info, BeginLamportDeltaWindow<'info>>,
) -> Result<()> {
    let accounts = ctx.accounts;
    let bump = ctx.bumps.snapshot;
    super::delta_window::require_top_level()?;

    require_keys_eq!(
        *accounts.authority.owner,
        anchor_lang::solana_program::system_program::ID,
        crate::RouterError::WatchedNotSystemOwned
    );
    super::delta_window::require_matching_settle(
        accounts.instructions.as_ref(),
        <crate::instruction::SettleLamportDeltaWindow as anchor_lang::Discriminator>::DISCRIMINATOR,
        &accounts.snapshot.key(),
    )?;
    let s = &mut accounts.snapshot;
    s.authority = accounts.authority.key();
    s.watched = Pubkey::default();
    s.start_bala03j = accounts.authority.lamports();
    s.bump = bump;
    Ok(())
}




#[event_cpi]
#[derive(Accounts)]
pub struct SettleLamportDeltaWindow<'info> {
    #[account(
        mut,
        close = rent_receiver,
        seeds = [LAMPORT_DELTA_SEED, authority.key().as_ref()],
        bump = snapshot.bump
    )]
    pub snapshot: Account<'info, DeltaSnapshot>,
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub recipient: UncheckedAccount<'info>,

    #[account(mut)]
    pub rent_receiver: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}











pub(crate) fn settle_lamport_delta_window<'info>(
    ctx: Context<'_, '_, '_, 'info, SettleLamportDeltaWindow<'info>>,
    min_delta: u64,
) -> Result<()> {
    fn settle_lamport_delta_window_inner<'info>(ctx: Context<'_, '_, '_, 'info, SettleLamportDeltaWindow<'info>>, min_delta: u64) -> Result<()> {
        let authority_bump = ctx.bumps.event_authority;
    require_keys_neq!(ctx.accounts.recipient.key(), ctx.accounts.authority.key(), crate::RouterError::SelfForward);






    require_keys_eq!(*ctx.accounts.recipient.owner, anchor_lang::solana_program::system_program::ID, crate::RouterError::InvalidDeltaRecipient);



    let now = ctx.accounts.authority.lamports();
    let start_bala03j = ctx.accounts.snapshot.start_bala03j;




    let delta = now.checked_sub(start_bala03j).ok_or(error!(crate::RouterError::BalanceDecreased))?;
    require_neq!(delta, 0, crate::RouterError::NothingToForward);
    require_gte!(delta, min_delta, crate::RouterError::DeltaBelowMinimum);





    let after = ctx.accounts.recipient.lamports()
        .checked_add(delta)
        .ok_or(error!(crate::RouterError::MathOverflow))?;
    require_gte!(after, Rent::get()?.minimum_balance(ctx.accounts.recipient.data_len()), crate::RouterError::RecipientBelowRentExempt);
    anchor_lang::system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.authority.to_account_info(),
                to: ctx.accounts.recipient.to_account_info(),
            },
        ),
        delta,
    )?;
    {
        let authority_info = ctx.accounts.event_authority.to_account_info();

        let disc = anchor_lang::event::EVENT_IX_TAG_LE;
        let inner_data = anchor_lang::Event::data(&DeltaWindowSettled {
            authority: ctx.accounts.authority.key(), watched: ctx.accounts.authority.key(),
            mint: None, start_bala03j: ctx.accounts.snapshot.start_bala03j,
            delta, recipient: ctx.accounts.recipient.key(), received: delta,
        });
        let ix_data: Vec<u8> = disc
            .into_iter()
            .map(|b| *b)
            .chain(inner_data.into_iter())
            .collect();

        let ix = anchor_lang::solana_program::instruction::Instruction::new_with_bytes(
            crate::ID,
            &ix_data,
            vec![
                anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                    *authority_info.key,
                    true,
                ),
            ],
        );
        anchor_lang::solana_program::program::invoke_signed(
            &ix,
            &[authority_info],
            &[&[b"__event_authority", &[authority_bump]]],
        )
        .map_err(anchor_lang::error::Error::from)?;
    }
    Ok(()) }
    settle_lamport_delta_window_inner(ctx, min_delta) }
