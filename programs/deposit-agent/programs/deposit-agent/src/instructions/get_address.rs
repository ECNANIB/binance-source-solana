use crate::DepositAgentError;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct GetAddress {}

pub fn handler(ctx: Context<GetAddress>, id: u128) -> Result<()> {
    let (addr, _bump) = Pubkey::find_program_address(
        &[b"account_token_holder", &id.to_le_bytes()],



        &crate::ID,
    );
    msg!("deposit_addr: {}", addr);




    Err(DepositAgentError::Rejected.into())
}
