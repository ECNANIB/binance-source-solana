use anchor_lang::prelude::*;

use crate::state::ConfigData;
use crate::DepositAgentError;

pub const MAX_SECP256K1_PUBKEYS: usize = 11;

#[derive(Accounts)]
pub struct SetSecp256k1Pubkey<'info> {
    #[account(
        mut,
        constraint = config_data.owner == *signer.key @ DepositAgentError::SenderNotAllowed,
    )]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"config_data"],
        bump = config_data.bump,
    )]
    pub config_data: Account<'info, ConfigData>,
    pub system_program: Program<'info, System>,
}





pub(crate) fn handler(
    ctx: Context<SetSecp256k1Pubkey>,
    pubkey: [u8; 64],
    remove: bool,
) -> Result<()> {
    let config_account = &mut ctx.accounts.config_data;
    let found = config_account.secp256k1_pubkeys.iter().any(|p| *p == pubkey);
    if remove {
        require!(found, DepositAgentError::AddrNotExists);
        for index in 0..config_account.secp256k1_pubkeys.len() {
            if config_account.secp256k1_pubkeys[index] == pubkey {
                config_account.secp256k1_pubkeys.remove(index);
                break;
            }
        }
    } else {
        require!(!found, DepositAgentError::AddrExists);
        config_account.secp256k1_pubkeys.push(pubkey);
        if config_account.secp256k1_pubkeys.len() > MAX_SECP256K1_PUBKEYS {
            return Err(DepositAgentError::ConfigAddrsExtendsLimit.into());
        }
    }
    return Ok(());
}
