use anchor_lang::prelude::*;

use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{TokenAccount, TokenInterface};
use crate::state::ConfigData;
use crate::ErrorCode;


#[derive(Accounts)]
pub struct EmergencyWithdraw<'info> {
    #[account(mut, constraint = config_data.addresses.contains(owner.key) @ ErrorCode::SenderNotAllowed)]
    pub owner: Signer<'info>,
    pub token_holder: UncheckedAccount<'info>,
    pub to_user: UncheckedAccount<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump_b2)]
    pub config_data: Account<'info, ConfigData>,
    #[account(mut)]
    pub token_holder_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(init_if_needed, payer = owner, associated_token::mint = token_mint, associated_token::authority = to_user)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_mint: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
