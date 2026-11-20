use anchor_lang::prelude::*;

use crate::errors::MemeshotVaultError;
use crate::state::ConfigData;
use crate::{SENDERS_LIMIT, WITHDRAW_TOS_LIMIT};

#[derive(Accounts)]
pub struct ConfigAddr<'info> {
    #[account(
        mut,
        constraint = super::is_registered(&config_data.owner, owner.key) @ MemeshotVaultError::SenderNotAllowed
    )]
    pub owner: Signer<'info>,
    #[account(mut, seeds = [b"config_data"], bump = config_data.bump)]
    pub config_data: Account<'info, ConfigData>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum ConfigAddrType {
    Senders,
    WithdrawTos,
    CloudWalletProgram,
    RebalanceOutHolder,
    AggRouter,
}

fn config_addr_type(kind: u8) -> Result<ConfigAddrType> {
    match kind {
        1 => Ok(ConfigAddrType::Senders),
        2 => Ok(ConfigAddrType::WithdrawTos),
        3 => Ok(ConfigAddrType::CloudWalletProgram),
        4 => Ok(ConfigAddrType::RebalanceOutHolder),
        5 => Ok(ConfigAddrType::AggRouter),
        _ => Err(MemeshotVaultError::InvalidConfigAddrType.into()),
    }
}

pub(crate) fn handler(
    ctx: Context<ConfigAddr>,
    kind: u8,
    addr: Pubkey,
    remove: bool,
) -> Result<()> {
    let config_data = &mut ctx.accounts.config_data;
    let kind = config_addr_type(kind)?;
    if ConfigAddrType::CloudWalletProgram > kind {
        match kind {
            ConfigAddrType::Senders | ConfigAddrType::WithdrawTos => {
                let list = addrs_mut(config_data, kind as usize);
                let found = list.contains(&addr);
                if !found && remove {
                    return Err(MemeshotVaultError::AddrNotExists.into());
                }
                if found && !remove {
                    return Err(MemeshotVaultError::AddrExists.into());
                }
                if found {
                    list.retain(|x| x != &addr);
                } else {
                    list.push(addr);
                }
                addr_limit(kind, list.len())?;
            }
            _ => return Err(MemeshotVaultError::InvalidConfigAddrType.into()),
        }
    } else {
        let r: core::result::Result<(), MemeshotVaultError> = 'other: {
            let current = match kind {
                ConfigAddrType::CloudWalletProgram => &config_data.cloud_wallet_program,
                ConfigAddrType::RebalanceOutHolder => &config_data.rebalance_out_holder,
                ConfigAddrType::AggRouter => &config_data.agg_router,
                _ => break 'other Err(MemeshotVaultError::InvalidConfigAddrType),
            };
            if *current == addr && !remove {
                break 'other Err(MemeshotVaultError::AddrExists);
            }
            let value = if remove == false {
                addr
            } else {
                Pubkey::default()
            };
            match kind {
                ConfigAddrType::CloudWalletProgram => config_data.cloud_wallet_program = value,
                ConfigAddrType::RebalanceOutHolder => config_data.rebalance_out_holder = value,
                ConfigAddrType::AggRouter => config_data.agg_router = value,
                _ => break 'other Err(MemeshotVaultError::InvalidConfigAddrType),
            }
            Ok(())
        };
        r?;
    }
    Ok(())
}

fn addrs_mut<'a, 'info>(
    config_data: &'a mut Account<'info, ConfigData>,
    index: usize,
) -> &'a mut Vec<Pubkey> {
    if index % 2 == 0 {
        &mut config_data.senders
    } else {
        &mut config_data.withdraw_tos
    }
}

fn addr_limit(kind: ConfigAddrType, len: usize) -> core::result::Result<(), MemeshotVaultError> {
    let limit = match kind {
        ConfigAddrType::Senders => SENDERS_LIMIT,
        ConfigAddrType::WithdrawTos => WITHDRAW_TOS_LIMIT,
        ConfigAddrType::CloudWalletProgram
        | ConfigAddrType::RebalanceOutHolder
        | ConfigAddrType::AggRouter => return Err(MemeshotVaultError::InvalidConfigAddrType),
    };
    if len > limit {
        return Err(MemeshotVaultError::ConfigAddrsExtendsLimit);
    }
    Ok(())
}
