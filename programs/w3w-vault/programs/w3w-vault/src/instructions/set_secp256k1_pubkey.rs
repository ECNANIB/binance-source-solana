use anchor_lang::prelude::*;

use crate::state::ConfigData;
use crate::ErrorCode;


#[derive(Accounts)]
pub struct SetSecp256k1Pubkey<'info> {
    #[account(mut, constraint = config_data.addresses.contains(signer.key) @ ErrorCode::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [b"config_data"], bump = config_data.bump_b2)]
    pub config_data: Account<'info, ConfigData>,
    pub system_program: Program<'info, System>,
}














pub(crate) fn handler(ctx: Context<SetSecp256k1Pubkey>, pubkey: [u8; 64], remove: bool) -> Result<()> {

    let config_data = &mut ctx.accounts.config_data;
    let found = config_data.secp256k1_pubkeys.contains(&pubkey);
    if remove {


        require!(found, ErrorCode::AddrNotExists);
        for index in 0..config_data.secp256k1_pubkeys.len() {
            if config_data.secp256k1_pubkeys.as_slice()[index] == pubkey { config_data.secp256k1_pubkeys.remove(index); break; }
        }

    } else {


        require!(!found, ErrorCode::AddrExists);
        config_data.secp256k1_pubkeys.push(pubkey);
        if config_data.secp256k1_pubkeys.len() > 11 {
            return Err(ErrorCode::ConfigAddrsExtendsLimit.into());
        }

    }
    return Ok(());
}






