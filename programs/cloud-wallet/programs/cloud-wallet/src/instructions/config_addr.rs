use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;

#[derive(Accounts)]
pub struct ConfigAddr<'info> {
    #[account(
        mut,
        constraint = ({
            let k = owner.key;
            config_data.owner != Pubkey::default() && config_data.owner == *k
        }) @ CloudWalletError::SenderNotAllowed
    )]
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
    let kind = addr_kind(addr_type)?;
    match kind {
        ConfigAddrKind::Managers
        | ConfigAddrKind::WithdrawAddrs
        | ConfigAddrKind::RebalanceAddrs => {
            let list = addr_list(config_data, kind)?;
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
            check_limit(kind, list.len())?;
        }
        _ => {
            return set_addr_slot(config_data, kind, addr, remove);
        }
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum ConfigAddrKind {
    TokenHolder,
    Managers,
    WithdrawAddrs,
    AggRouter,
    RebalanceAddrs,
}

fn addr_kind(addr_type: u8) -> Result<ConfigAddrKind> {
    match addr_type {
        1 => Ok(ConfigAddrKind::TokenHolder),
        2 => Ok(ConfigAddrKind::Managers),
        3 => Ok(ConfigAddrKind::WithdrawAddrs),
        4 => Ok(ConfigAddrKind::AggRouter),
        5 => Ok(ConfigAddrKind::RebalanceAddrs),
        _ => Err(CloudWalletError::InvalidConfigAddrType.into()),
    }
}

fn addr_list(config_data: &mut ConfigData, kind: ConfigAddrKind) -> Result<&mut Vec<Pubkey>> {
    match kind {
        ConfigAddrKind::Managers => Ok(&mut config_data.managers),
        ConfigAddrKind::WithdrawAddrs => Ok(&mut config_data.withdraw_addrs),
        ConfigAddrKind::RebalanceAddrs => Ok(&mut config_data.rebalance_addrs),
        _ => Err(CloudWalletError::InvalidConfigAddrType.into()),
    }
}

fn set_addr_slot(
    config_data: &mut Account<ConfigData>,
    kind: ConfigAddrKind,
    addr: Pubkey,
    remove: bool,
) -> Result<()> {
    let r: core::result::Result<(), CloudWalletError> = 'b: {
        let found = *match kind {
            ConfigAddrKind::TokenHolder => &config_data.token_holder,
            ConfigAddrKind::AggRouter => &config_data.agg_router,
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
        *match kind {
            ConfigAddrKind::TokenHolder => &mut config_data.token_holder,
            ConfigAddrKind::AggRouter => &mut config_data.agg_router,
            _ => break 'b Err(CloudWalletError::InvalidConfigAddrType),
        } = value;
        Ok(())
    };
    r?;
    Ok(())
}

fn check_limit(kind: ConfigAddrKind, len: usize) -> core::result::Result<(), CloudWalletError> {
    let limit = match kind {
        ConfigAddrKind::Managers => 4,
        ConfigAddrKind::WithdrawAddrs => 3,
        ConfigAddrKind::RebalanceAddrs => 16,
        ConfigAddrKind::TokenHolder | ConfigAddrKind::AggRouter => {
            return Err(CloudWalletError::InvalidConfigAddrType)
        }
    };
    if len > limit {
        return Err(CloudWalletError::ConfigAddrsExtendsLimit);
    }
    Ok(())
}
