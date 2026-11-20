use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct CloseWsol<'info> {
    #[account(
        mut,
        constraint = config_data.addrs.contains(signer.key) @ crate::VaultError::NotAllowed
    )]
    pub signer: Signer<'info>,
    #[account(
        seeds = [crate::CONFIG_DATA_SEED],
        bump = config_data.data_bump,
        constraint = !config_data.paused @ crate::VaultError::Paused
    )]
    pub config_data: Box<Account<'info, ConfigData>>,

    #[account(mut)]
    pub token_holder: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = wsol_mint,
        associated_token::authority = token_holder,
        associated_token::token_program = token_program
    )]
    pub token_holder_wsol_account: InterfaceAccount<'info, TokenAccount>,

    pub tmp_token_holder: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = wsol_mint,
        associated_token::authority = tmp_token_holder,
        associated_token::token_program = token_program
    )]
    pub tmp_token_holder_wsol_account: InterfaceAccount<'info, TokenAccount>,

    pub wsol_mint: UncheckedAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
