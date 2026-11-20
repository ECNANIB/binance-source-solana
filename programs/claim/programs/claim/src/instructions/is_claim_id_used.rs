use anchor_lang::prelude::*;

use crate::state::*;
use crate::ClaimError;









#[derive(Accounts)]
#[instruction(claim_ids: Vec<u64>)]
pub struct IsClaimIdUsed<'info> {
    #[account(seeds = [b"big_nonce_data", &(claim_ids[0] / 1146752).to_le_bytes()], bump)]
    pub nonce_data: AccountLoader<'info, BigNonceData>,
}

pub(crate) fn handler(ctx: Context<IsClaimIdUsed>, claim_ids: Vec<u64>) -> Result<()> {

    let ids = claim_ids.as_slice();
    let nonce_data = ctx.accounts.nonce_data.load()?;
    let nonce_data_index = claim_ids[0] / 1146752;

    let mut it = ids.iter(); while let Some(claim_id) = it.next() {
        if !(nonce_data_index == *claim_id / 1146752) {
            return Err(error!(ClaimError::NonceOutOfTable));
        }
        let word = ((*claim_id >> 7) % 8959) as usize;
        let bit = *claim_id & 127;
        msg!("{}:{}", claim_id, (nonce_data.table[word] >> bit) & 1 != 0);
    }

    Ok(())
}
