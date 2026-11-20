use crate::state::ConfigData;
use anchor_lang::prelude::*;





#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer = payer, space = ConfigData::SPACE, seeds = [b"config_data"], bump)]
    pub config_data: Account<'info, ConfigData>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn handler(
    ctx: Context<Initialize>,
    owner: Pubkey,
    deposit_addr: Pubkey,
    expire_duration: u64,
    pubkey: [u8; 64],
) -> Result<()> {
    let c = &mut ctx.accounts.config_data;
    c.bump = ctx.bumps.config_data;
    c.expire_duration = expire_duration;
    c.owner = owner;
    c.deposit_addr = Some(deposit_addr);



    c.signer_pubkeys = vec![pubkey];
    Ok(())
}
