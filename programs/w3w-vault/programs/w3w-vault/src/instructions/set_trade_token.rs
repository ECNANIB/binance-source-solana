use anchor_lang::prelude::*;

use anchor_spl::token_interface::Mint;
use crate::state::{ConfigData, TokenConfigData};
use crate::ErrorCode;



#[derive(Accounts)]
pub struct SetTradeTo003<'info> {
    #[account(mut, constraint = config_data.addresses.contains(signer.key) @ ErrorCode::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump_b2)]
    pub config_data: Box<Account<'info, ConfigData>>,
    #[account(init_if_needed, payer = signer, space = 8 + 1476, seeds = [b"config_data", token_mint.key().as_ref()], bump)]
    pub token_config_data: Account<'info, TokenConfigData>,
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}













pub fn handler(ctx: Context<SetTradeTo003>, arg0: u64) -> Result<()> {
    let token_config_data = &mut ctx.accounts.token_config_data;
    token_config_data.bump = ctx.bumps.token_config_data;
    token_config_data.fiel00p = arg0;
    Ok(())
}

pub fn remove_handler(_ctx: Context<super::RemoveTradeTo003>) -> Result<()> {
    Ok(())
}
