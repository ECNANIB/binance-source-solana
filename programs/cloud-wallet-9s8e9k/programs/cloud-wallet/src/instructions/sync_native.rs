use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;

#[derive(Accounts)]
pub struct SyncNative<'info> {
    #[account(mut, constraint = config_data.managers.contains(signer.key) || config_data.rebalance_addrs.contains(signer.key) @ CloudWalletError::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(mut)]

    pub token_holder: UncheckedAccount<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump, constraint = !config_data.paused @ CloudWalletError::Paused)]
    pub config_data: Box<Account<'info, ConfigData>>,
    #[account(mut)]

    pub wsol_account: UncheckedAccount<'info>,

    pub wsol_mint: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}























pub(crate) fn sync_native(ctx: Context<SyncNative>, amount: u64) -> Result<()> {
    crate::instructions::check_mint(ctx.accounts.wsol_mint.key)?;
    {
        let (ata, _) = Pubkey::find_program_address(
            &[
                ctx.accounts.token_holder.key.as_ref(),
                ctx.accounts.token_program.key.as_ref(),
                ctx.accounts.wsol_mint.key.as_ref(),
            ],
            ctx.accounts.associated_token_program.key,
        );
        require_keys_eq!(ctx.accounts.wsol_account.key(), ata);
    }

    let signer_seeds: &[&[&[u8]]] = &[&[
        b"token_holder",
        &[ctx.accounts.config_data.token_holder_bump],
    ]];
    if ctx.accounts.wsol_account.data_is_empty() {
        let create_ctx = CpiContext::new_with_signer(
            ctx.accounts.associated_token_program.to_account_info(),
            anchor_spl::associated_token::Create {
                payer: ctx.accounts.token_holder.to_account_info(),
                associated_token: ctx.accounts.wsol_account.to_account_info(),
                authority: ctx.accounts.token_holder.to_account_info(),
                mint: ctx.accounts.wsol_mint.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            signer_seeds,
        );
        anchor_spl::associated_token::create(create_ctx)?;
    }
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.token_holder.to_account_info(),
            to: ctx.accounts.wsol_account.to_account_info(),
        },
        signer_seeds,
    );
    anchor_lang::system_program::transfer(cpi_ctx, amount)?;

    let token_program = ctx.accounts.token_program.to_account_info();
    match *token_program.key {
        anchor_spl::token::ID => {}
        anchor_spl::token_2022::ID => {}
        _ => return Err(CloudWalletError::InvalidProgram.into()),
    }
    let cpi_accounts = anchor_spl::token_interface::SyncNative {
        account: ctx.accounts.wsol_account.to_account_info(),
    };
    let sync_ctx = CpiContext::new_with_signer(token_program, cpi_accounts, signer_seeds);
    anchor_spl::token_interface::sync_native(sync_ctx)?;
    Ok(())
}
