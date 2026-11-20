use anchor_lang::prelude::*;
use crate::DepositAgentError;

pub(crate) fn handler(_ctx: Context<crate::GetAddress>, id: u128) -> Result<()> {
    let (addr, _) = Pubkey::find_program_address(
        &[crate::TOKEN_HOLDER_SEED, &id.to_le_bytes()],
        &crate::ID,
    );
    msg!("deposit_addr: {}", addr);
    Err(DepositAgentError::Rejected.into())
}
