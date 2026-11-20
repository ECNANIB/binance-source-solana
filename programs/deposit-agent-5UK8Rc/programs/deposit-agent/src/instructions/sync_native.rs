use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{self, SyncNative, TokenAccount, TokenInterface};

use crate::state::ConfigData;
use crate::DepositAgentError;
use utils::is_wsol_mint;
#[derive(Accounts)]
#[instruction(id: u128, amount: u64)]
pub struct SyncNativeInstruction<'info> {
    #[account(mut, constraint = config_data.withdraw_tos.contains(signer.key) @ DepositAgentError::SenderNotAllowed)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [crate::TOKEN_HOLDER_SEED, &id.to_le_bytes()],
        bump,
    )]
    pub token_holder: UncheckedAccount<'info>,
    #[account(seeds = [crate::CONFIG_DATA_SEED], bump = config_data.bump, constraint = !config_data.paused @ DepositAgentError::Paused)]
    pub config_data: Box<Account<'info, ConfigData>>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = wsol_mint,
        associated_token::authority = token_holder,
    )]
    pub wsol_account: InterfaceAccount<'info, TokenAccount>,

    pub wsol_mint: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
pub(crate) fn check_wsol_mint(key: &Pubkey) -> Result<()> {
    if !is_wsol_mint(key) {
        return Err(DepositAgentError::InvalidProgram.into());
    }
    Ok(())
}

pub(crate) fn handler(ctx: Context<SyncNativeInstruction>, id: u128, amount: u64, bump: u8) -> Result<()> {
    let seeds: &[&[u8]] = &[
        crate::TOKEN_HOLDER_SEED,
        &id.to_le_bytes(),
        &[bump],
    ];
    let signer_seeds: &[&[&[u8]]] = &[seeds];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.token_holder.to_account_info(),
            to: ctx.accounts.wsol_account.to_account_info(),
        },
        signer_seeds,
    );
    anchor_lang::system_program::transfer(cpi_ctx, amount)?;
    let token_program_info = ctx.accounts.token_program.to_account_info();
    match *token_program_info.key {
        anchor_spl::token::ID | anchor_spl::token_2022::ID => {}
        _ => return Err(DepositAgentError::InvalidProgram.into()),
    }
    let cpi_accounts = SyncNative { account: ctx.accounts.wsol_account.to_account_info() };
    token_interface::sync_native(CpiContext::new_with_signer(
        token_program_info,
        cpi_accounts,
        signer_seeds,
    ))?;
    Ok(())
}
