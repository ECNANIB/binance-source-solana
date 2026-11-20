use crate::state::ConfigData;
use crate::DepositAgentError;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SetSecp256k1Pubkey<'info> {
    #[account(mut, constraint = config_data.owner == *signer.key @ DepositAgentError::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [b"config_data"], bump = config_data.bump)]
    pub config_data: Account<'info, ConfigData>,
    pub system_program: Program<'info, System>,
}













pub(crate) fn handler(
    ctx: Context<SetSecp256k1Pubkey>,
    pubkey: [u8; 64],
    remove: bool,
) -> Result<()> {
    let c = &mut ctx.accounts.config_data;
    let found = c.signer_pubkeys.contains(&pubkey);
    if remove {


        require!(found, DepositAgentError::AddrNotExists);
        for index in 0..c.signer_pubkeys.len() {
            if c.signer_pubkeys[index] == pubkey {
                c.signer_pubkeys.remove(index);
                break;
            }
        }
    } else {
        require!(!found, DepositAgentError::AddrExists);
        c.signer_pubkeys.push(pubkey);
        if c.signer_pubkeys.len() > ConfigData::MAX_PUBKEYS {
            return Err(DepositAgentError::ConfigAddrsExtendsLimit.into());
        }
    }
    Ok(())
}
