use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;

#[derive(Accounts)]
pub struct ConfigAddr<'info> {
    #[account(mut, constraint = ({ let k = owner.key; config_data.owner != Pubkey::default() && config_data.owner == *k }) @ CloudWalletError::SenderNotAllowed)]
    pub owner: Signer<'info>,
    #[account(mut, seeds = [b"config_data"], bump = config_data.bump)]
    pub config_data: Account<'info, ConfigData>,
    pub system_program: Program<'info, System>,
}
pub(crate) fn config_addr(
    ctx: Context<ConfigAddr>,
    addr_type: u8,
    addr: Pubkey,
    remove: bool,
) -> Result<()> {
    let config_data = &mut ctx.accounts.config_data;
    let idx = addr_index(addr_type)?;
    match idx {
        AddrKind::Managers | AddrKind::WithdrawAddrs | AddrKind::RebalanceAddrs => {
            let list = addr_list(config_data, idx)?;
            let found = list.iter().any(|x| *x == addr);
            if !found && remove {
                return Err(CloudWalletError::AddrNotExists.into());
            }
            if found && !remove {
                return Err(CloudWalletError::AddrExists.into());
            }
            if remove {
                list.retain(|x| *x != addr);
            } else {
                list.push(addr);
            }
            check_limit(idx, list.len())?;
        }
        _ => {
            return set_addr_slot(config_data, idx, addr, remove);
        }
    }
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum AddrKind {
    TokenHolder = 0,
    Managers = 1,
    WithdrawAddrs = 2,
    AggRouter = 3,
    RebalanceAddrs = 4,
}

fn addr_index(addr_type: u8) -> Result<AddrKind> {
    match addr_type {
        1 => Ok(AddrKind::TokenHolder),
        2 => Ok(AddrKind::Managers),
        3 => Ok(AddrKind::WithdrawAddrs),
        4 => Ok(AddrKind::AggRouter),
        5 => Ok(AddrKind::RebalanceAddrs),
        _ => Err(CloudWalletError::InvalidConfigAddrType.into()),
    }
}

fn addr_list(config_data: &mut ConfigData, idx: AddrKind) -> Result<&mut Vec<Pubkey>> {
    match idx {
        AddrKind::Managers => Ok(&mut config_data.managers),
        AddrKind::WithdrawAddrs => Ok(&mut config_data.withdraw_addrs),
        AddrKind::RebalanceAddrs => Ok(&mut config_data.rebalance_addrs),
        _ => Err(CloudWalletError::InvalidConfigAddrType.into()),
    }
}

fn set_addr_slot(
    config_data: &mut Account<ConfigData>,
    idx: AddrKind,
    addr: Pubkey,
    remove: bool,
) -> Result<()> {
    let r: core::result::Result<(), CloudWalletError> = 'b: {
        let found = *match idx {
            AddrKind::TokenHolder => &config_data.token_holder,
            AddrKind::AggRouter => &config_data.agg_router,
            _ => break 'b Err(CloudWalletError::InvalidConfigAddrType),
        } == addr;
        if !remove && found {
            break 'b Err(CloudWalletError::AddrExists);
        }
        let value = if remove == false {
            addr
        } else {
            Pubkey::default()
        };
        *match idx {
            AddrKind::TokenHolder => &mut config_data.token_holder,
            AddrKind::AggRouter => &mut config_data.agg_router,
            _ => break 'b Err(CloudWalletError::InvalidConfigAddrType),
        } = value;
        Ok(())
    };
    r?;
    Ok(())
}

fn check_limit(idx: AddrKind, len: usize) -> core::result::Result<(), CloudWalletError> {
    let limit = match idx {
        AddrKind::Managers => 4,
        AddrKind::RebalanceAddrs => 16,
        AddrKind::WithdrawAddrs => 3,
        AddrKind::TokenHolder | AddrKind::AggRouter => {
            return Err(CloudWalletError::InvalidConfigAddrType)
        }
    };
    if len > limit {
        return Err(CloudWalletError::ConfigAddrsExtendsLimit);
    }
    Ok(())
}
