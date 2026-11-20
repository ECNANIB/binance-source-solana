use anchor_lang::prelude::*;

use anchor_spl::associated_token::{self, AssociatedToken, Create};
use anchor_spl::token_interface::{self, CloseAccount, SyncNative as SyncNativeCpi, TokenAccount, TokenInterface};
use crate::state::ConfigData;
use crate::ErrorCode;


#[derive(Accounts)]
pub struct SyncNat003<'info> {
    #[account(mut, constraint = (config_data.addresses_2.contains(signer.key) || config_data.addresses_1.contains(signer.key)) @ ErrorCode::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub token_holder: UncheckedAccount<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump_b2, constraint = !config_data.paused @ ErrorCode::Paused)]
    pub config_data: Box<Account<'info, ConfigData>>,
    #[account(mut)]
    pub wsol_account: UncheckedAccount<'info>,
    pub wsol_mint: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
























pub(crate) fn handler(ctx: Context<SyncNat003>, amount: u64) -> Result<()> {
    crate::instructions::check_mint(ctx.accounts.wsol_mint.key)?;

    {
    let expected = Pubkey::find_program_address(&[

            ctx.accounts.token_holder.key.as_ref(),
            ctx.accounts.token_program.key.as_ref(),
            ctx.accounts.wsol_mint.key.as_ref(),
        ],
        ctx.accounts.associated_token_program.key).0;

    require_keys_eq!(ctx.accounts.wsol_account.key(), expected);
    }




    let bump = ctx.accounts.config_data.bump_b3;
    let seeds: &[&[u8]] = &[b"token_holder", &[bump]];
    let signer_seeds: &[&[&[u8]]] = &[seeds];




    if ctx.accounts.wsol_account.data_is_empty() {
    let create_ctx = CpiContext::new_with_signer(
        ctx.accounts.associated_token_program.to_account_info(),
        Create {
            payer: ctx.accounts.token_holder.to_account_info(),
            associated_token: ctx.accounts.wsol_account.to_account_info(),
            authority: ctx.accounts.token_holder.to_account_info(),
            mint: ctx.accounts.wsol_mint.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        },
        signer_seeds,
    );
    associated_token::create(create_ctx)?;
    }
    let transfer_ctx =
        CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.token_holder.to_account_info(),
                to: ctx.accounts.wsol_account.to_account_info(),
            },
            signer_seeds,
        );
    anchor_lang::system_program::transfer(transfer_ctx,
        amount)?;






    let token_program = ctx.accounts.token_program.to_account_info();
    match *token_program.key {
        crate::instructions::TOKEN_PROGRAM | crate::instructions::TOKEN_2022_PROGRAM => {}
        _ => return Err(ErrorCode::InvalidProgram.into()),
    }
    let cpi_accounts = SyncNativeCpi {
        account: ctx.accounts.wsol_account.to_account_info(),
    };
    let sync_ctx = CpiContext::new_with_signer(
        token_program,
        cpi_accounts,
        signer_seeds,
    );
    token_interface::sync_native(sync_ctx)?;
    Ok(())
}


pub(crate) fn close_handler(ctx: Context<super::CloseW003>) -> Result<()> {
    crate::instructions::check_mint(ctx.accounts.wsol_mint.key)?;

    let signer_seeds: &[&[&[u8]]] = &[&[b"token_holder", &[ctx.accounts.config_data.bump_b3]]];
    token_interface::close_account(CpiContext::new_with_signer(
        ctx.accounts.wsol_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.token_holder_wsol_account.to_account_info(),
            destination: ctx.accounts.token_holder.to_account_info(),
            authority: ctx.accounts.token_holder.to_account_info(),
        },
        signer_seeds,
    ))?;
    Ok(())
}
