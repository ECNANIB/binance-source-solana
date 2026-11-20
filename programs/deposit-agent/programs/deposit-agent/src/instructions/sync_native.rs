use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    self, SyncNative as SyncNativeCpi, TokenAccount, TokenInterface,
};

use crate::state::ConfigData;
use crate::DepositAgentError;

#[derive(Accounts)]
#[instruction(id: u128, amount: u64)]
pub struct SyncNative<'info> {
    #[account(mut, constraint = config_data.senders.contains(signer.key) @ DepositAgentError::SenderNotAllowed)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"account_token_holder", &id.to_le_bytes()],
        bump,
    )]
    pub token_holder: UncheckedAccount<'info>,






    #[account(seeds = [b"config_data"], bump = config_data.bump, constraint = !config_data.paused @ DepositAgentError::Paused)]
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
    const WSOL_MINT: Pubkey =
        anchor_lang::solana_program::pubkey!("So11111111111111111111111111111111111111112");
    const ALT_WSOL_MINT: Pubkey =
        anchor_lang::solana_program::pubkey!("9pan9bMn5HatX4EJdBwg9VgCa7Uz5HL8N1m5D3NdXejP");
    match *key {
        WSOL_MINT | ALT_WSOL_MINT => Ok(()),
        _ => Err(DepositAgentError::InvalidProgram.into()),
    }
}
