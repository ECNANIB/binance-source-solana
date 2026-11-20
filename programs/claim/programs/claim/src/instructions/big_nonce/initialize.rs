use anchor_lang::prelude::*;

use crate::state::*;

pub const BIG_NONCE_DATA_SPACE: usize = 10240;
pub const NONCE_INCREASE_DATA_SPACE: usize = 17;



#[derive(Accounts)]
#[instruction(args: BigNonceInitializeArgs)]
pub struct BigNonceInitialize<'info> {
    #[account(
        init,
        payer = payer,
        space = BIG_NONCE_DATA_SPACE,
        seeds = [args.id.as_slice(), args.index.to_le_bytes().as_ref()],
        bump,
    )]
    pub nonce_data: AccountLoader<'info, BigNonceData>,
    #[account(
        init,
        payer = payer,
        space = NONCE_INCREASE_DATA_SPACE,
        seeds = [b"nonce_increase_data", nonce_data.key().to_bytes().as_ref()],
        bump,
    )]
    pub nonce_increase_data: Account<'info, NonceIncreaseData>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<BigNonceInitialize>, args: BigNonceInitializeArgs) -> Result<()> {
    super::check_id(&args.id)?;
    let nonce_increase_data = &mut ctx.accounts.nonce_increase_data;
    nonce_increase_data.count = 0;
    nonce_increase_data.bump = ctx.bumps.nonce_increase_data;
    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BigNonceInitializeArgs {
    pub id: Vec<u8>,
    pub kind: u8,
    pub index: u64,
}
