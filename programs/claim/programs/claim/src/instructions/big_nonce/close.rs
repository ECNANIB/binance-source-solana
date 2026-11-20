use anchor_lang::prelude::*;

use crate::state::*;
use crate::ClaimError;

#[derive(Accounts)]
#[instruction(id: Vec<u8>, kind: u8, index: u64)]
pub struct BigNonceClose<'info> {
    #[account(
        mut,
        constraint = config_data.owner == *signer.key @ ClaimError::NotAllowed,
    )]
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"config_data"],
        bump = config_data.bump,
    )]
    pub config_data: Account<'info, ConfigData>,
    #[account(
        mut,
        close = signer,
        seeds = [id.as_slice(), index.to_le_bytes().as_ref()],
        bump = kind,
    )]
    pub nonce_data: AccountLoader<'info, BigNonceData>,
    #[account(
        mut,
        close = signer,
        seeds = [b"nonce_increase_data", nonce_data.key().to_bytes().as_ref()],
        bump = nonce_increase_data.bump,
    )]
    pub nonce_increase_data: Account<'info, NonceIncreaseData>,
    pub system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<BigNonceClose>, id: Vec<u8>, _kind: u8, _index: u64) -> Result<()> {
    super::check_id(&id)?;
    msg!("NOTICE: Closing nonce account. Make sure it is no longer needed.");
    Ok(())
}
