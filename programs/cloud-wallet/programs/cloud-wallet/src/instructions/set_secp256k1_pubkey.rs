use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;

enum AddrKind { Secp, Deposit, Withdraw }
impl ConfigData {
    fn addr_list(&mut self, kind: AddrKind) -> &mut Vec<[u8; 64]> {
        match kind {
            AddrKind::Secp => &mut self.secp256k1_pubkeys,
            AddrKind::Deposit => &mut self.deposit_secp256k1_pubkeys,
            AddrKind::Withdraw => &mut self.withdraw_secp256k1_pubkeys,
        }
    }
}
fn addr_kind(addr_type: u8) -> AddrKind { match addr_type { 0 => AddrKind::Secp, 1 => AddrKind::Deposit, 2 => AddrKind::Withdraw, _ =>
                 panic!("Invalid typ")
} }



#[derive(Accounts)]
pub struct SetSecp256k1Pubkey<'info> {
    #[account(mut, constraint = config_data.owner == *signer.key @ CloudWalletError::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [b"config_data"], bump = config_data.bump)]
    pub config_data: Account<'info, ConfigData>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn set_secp256k1_pubkey(
    ctx: Context<SetSecp256k1Pubkey>,
    addr_type: u8,
    pubkey: [u8; 64],
    remove: bool,
) -> Result<()> {
    let kind = addr_kind(addr_type);
    let config_data = &mut *ctx.accounts.config_data;
    let list = config_data.addr_list(kind);
    let found = list.contains(&pubkey);
    if remove {













        require!(found, CloudWalletError::AddrNotExists);
        for i in 0..list.len() {
            if list[i] == pubkey {
                list.remove(i);
                break;
            }
        }
    } else {
        require!(!found, CloudWalletError::AddrExists);
        list.push(pubkey);
        if list.len() > 11 {
            return Err(CloudWalletError::ConfigAddrsExtendsLimit.into());
        }
    }
    Ok(())
}
