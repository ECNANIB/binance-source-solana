use anchor_lang::prelude::*;

use anchor_spl::associated_token::{self, AssociatedToken, Create};
use anchor_spl::token_interface::{self, CloseAccount, SyncNative as SyncNativeCpi, TokenAccount, TokenInterface};
use crate::state::ConfigData;
use crate::ErrorCode;


#[derive(Accounts)]
pub struct CloseW003<'info> {
    #[account(mut, constraint = (config_data.addresses_2.contains(signer.key) || config_data.addresses_1.contains(signer.key)) @ ErrorCode::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump_b2)]
    pub config_data: Box<Account<'info, ConfigData>>,
    #[account(mut)]
    pub token_holder: UncheckedAccount<'info>,
    #[account(mut, token::mint = wsol_mint)]
    pub token_holder_wsol_account: InterfaceAccount<'info, TokenAccount>,
    pub wsol_mint: UncheckedAccount<'info>,
    pub wsol_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
