use anchor_lang::prelude::*;

use crate::state::*;
use crate::ClaimError;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BigNonceIncreaseArgs {
    pub id: Vec<u8>,
    pub kind: u8,
    pub count: u64,
    pub index: u64,
}






#[derive(Accounts)]
#[instruction(args: BigNonceIncreaseArgs)]
pub struct BigNonceIncrease<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [args.id.as_slice(), args.count.to_le_bytes().as_ref()],
        bump = args.kind,
        realloc = args.index as usize,
        realloc::payer = signer,
        realloc::zero = false,
    )]
    pub nonce_data: AccountLoader<'info, BigNonceData>,
    #[account(
        mut,
        seeds = [b"nonce_increase_data", nonce_data.key().to_bytes().as_ref()],
        bump = nonce_increase_data.bump,
        constraint = nonce_increase_data.count < args.index @ ClaimError::InvalidParameter,
    )]
    pub nonce_increase_data: Account<'info, NonceIncreaseData>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<BigNonceIncrease>, args: BigNonceIncreaseArgs) -> Result<()> {
    super::check_id(&args.id)?;
    let nonce_increase_data = &mut ctx.accounts.nonce_increase_data;
    nonce_increase_data.count = args.index;
    Ok(())
}
