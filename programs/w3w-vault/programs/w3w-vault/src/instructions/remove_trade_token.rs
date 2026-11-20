use anchor_lang::prelude::*;

use anchor_spl::token_interface::Mint;
use crate::state::{ConfigData, TokenConfigData};
use crate::ErrorCode;


#[derive(Accounts)]
pub struct RemoveTradeTo003<'info> {
    #[account(mut, constraint = config_data.addresses.contains(signer.key) @ ErrorCode::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump_b2)]
    pub config_data: Box<Account<'info, ConfigData>>,
    #[account(mut, close = signer, seeds = [b"config_data", token_mint.key().as_ref()], bump = token_config_data.bump)]
    pub token_config_data: Account<'info, TokenConfigData>,
    pub token_mint: InterfaceAccount<'info, Mint>,
}
