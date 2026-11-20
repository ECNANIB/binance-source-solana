use anchor_lang::prelude::*;
use anchor_spl::associated_token::{get_associated_token_address_with_program_id, AssociatedToken};
use anchor_spl::token::Token;
use anchor_spl::token_interface::{TokenAccount, TokenInterface};

use crate::error::ReferralSwapError;
use crate::state::ConfigData;

#[event_cpi]
#[derive(Accounts)]
pub struct Rebalanc021<'info> {
    #[account(mut,
        constraint = config_data.send021.contains(signer.key) @ ReferralSwapError::NotAllowed)]
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"config_data"],
        bump = config_data.bump,
        constraint = !config_data.pau021 @ ReferralSwapError::Paused
    )]
    pub config_data: Box<Account<'info, ConfigData>>,

    #[account(
        mut,
        seeds = [b"token_holder"],
        bump = config_data.token_holder_b021
    )]
    pub token_holder: UncheckedAccount<'info>,
    #[account(mut)]
    pub from_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub to_token_account: UncheckedAccount<'info>,

    pub token_mint: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = config_data.platform == *platform.key @ ReferralSwapError::NotAllowed
    )]
    pub platform: UncheckedAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}






























pub(crate) fn handler(ctx: Context<Rebalanc021>, amount: u64, decimals: u8) -> Result<()> {
    require!(amount > 0, ReferralSwapError::InvalidParameter);

    let seeds: &[&[&[u8]]] =
        &[&[b"token_holder", &[ctx.accounts.config_data.token_holder_b021]]];


    let mint = ctx.accounts.token_mint.key();


    if mint != ctx.accounts.system_program.key() {
    let expected = get_associated_token_address_with_program_id(
        ctx.accounts.platform.key,
        &mint,
        ctx.accounts.token_program.key,
    );












    require!(
        ctx.accounts.to_token_account.key() == expected,
        ReferralSwapError::InvalidParameter
    );


    let cpi_create = CpiContext::new(
        ctx.accounts.associated_token_program.to_account_info(),
        anchor_spl::associated_token::Create {
            payer: ctx.accounts.signer.to_account_info(),
            associated_token: ctx.accounts.to_token_account.to_account_info(),
            authority: ctx.accounts.platform.to_account_info(),
            mint: ctx.accounts.token_mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        },
    );
    anchor_spl::associated_token::create_idempotent(cpi_create)?;

    let cpi_transfer = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        anchor_spl::token_2022::TransferChecked {
            from: ctx.accounts.from_token_account.to_account_info(),
            to: ctx.accounts.to_token_account.to_account_info(),
            authority: ctx.accounts.token_holder.to_account_info(),
            mint: ctx.accounts.token_mint.to_account_info(),
        },
        seeds,
    );
    anchor_spl::token_2022::transfer_checked(cpi_transfer, amount, decimals)?;
    } else {
    anchor_lang::system_program::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.token_holder.to_account_info(),
                to: ctx.accounts.platform.to_account_info(),
            },
            seeds,
        ),
        amount,
    )?;
    }

    emit_cpi!(crate::instructions::swap::SwapEvent {
        from_mint: mint,
        amount_in: amount,
        to_mint: mint,
        amount_out: amount,
    });

    Ok(())
}






