use anchor_lang::prelude::*;

use crate::state::ConfigData;
use crate::{DepositAgentError, MAX_CONFIG_ADDRS, MAX_OPERATORS};

#[derive(Accounts)]
pub struct ConfigAddr<'info> {
    #[account(
        mut,
        constraint = config_data.owner == *owner.key @ DepositAgentError::SenderNotAllowed,
    )]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"config_data"],
        bump = config_data.bump,
    )]
    pub config_data: Account<'info, ConfigData>,
    pub system_program: Program<'info, System>,
}
pub(crate) enum ConfigAddrKind {
    ProgramAddr,
    WithdrawTo,
    Operator,
}

pub fn config_addr_kind(kind: u8) -> Result<ConfigAddrKind> {
    match kind {
        1 => Ok(ConfigAddrKind::ProgramAddr),
        2 => Ok(ConfigAddrKind::WithdrawTo),
        3 => Ok(ConfigAddrKind::Operator),
        _ => Err(DepositAgentError::InvalidConfigAddrType.into()),
    }
}























































pub(crate) fn handler(
    ctx: Context<ConfigAddr>,
    kind: u8,
    addr: Pubkey,
    remove: bool,
) -> Result<()> {
    let config_data = &mut ctx.accounts.config_data;
    let kind = config_addr_kind(kind)?;
    if !matches!(kind, ConfigAddrKind::WithdrawTo | ConfigAddrKind::Operator) {
        return Ok(set_program_addr(config_data, addr, remove)?);
    }
    let config_data: &mut ConfigData = config_data;
    let addrs = match kind {
        ConfigAddrKind::WithdrawTo => &mut config_data.withdraw_tos,
        _ => &mut config_data.operators,
    };
    let found = addrs.contains(&addr);
    if !found && remove {
        return Err(DepositAgentError::AddrNotExists.into());
    }
    if found && !remove {
        return Err(DepositAgentError::AddrExists.into());
    }
    if found {
        addrs.retain(|a| *a != addr);
    } else {
        addrs.push(addr);
    }
    check_limit(kind, addrs.len())?;
    Ok(())
}

fn check_limit(kind: ConfigAddrKind, len: usize) -> core::result::Result<(), DepositAgentError> {
    let limit = match kind {
        ConfigAddrKind::ProgramAddr => return Err(DepositAgentError::InvalidConfigAddrType),
        ConfigAddrKind::WithdrawTo => MAX_CONFIG_ADDRS,
        ConfigAddrKind::Operator => MAX_OPERATORS,
    };
    if len > limit {
        return Err(DepositAgentError::ConfigAddrsExtendsLimit);
    }
    Ok(())
}

fn set_program_addr(config_data: &mut Account<ConfigData>, addr: Pubkey, remove: bool) -> Result<()> {
    if config_data.program_addr.is_some_and(|c| c == addr) && !remove {
        return Err(DepositAgentError::AddrExists.into());
    }
    config_data.program_addr = Some(if remove { Pubkey::default() } else { addr });
    Ok(())
}
