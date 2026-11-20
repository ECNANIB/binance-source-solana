use crate::state::ConfigData;
use crate::DepositAgentError;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ConfigAddr<'info> {
    #[account(mut, constraint = config_data.owner == *owner.key @ DepositAgentError::SenderNotAllowed)]
    pub owner: Signer<'info>,
    #[account(mut, seeds = [b"config_data"], bump = config_data.bump)]
    pub config_data: Account<'info, ConfigData>,
    pub system_program: Program<'info, System>,
}








#[derive(Clone, Copy, PartialEq)]
pub(crate) enum ConfigAddrKind {
    DepositAddr,
    Senders,
    WithdrawTos,
}

pub(crate) fn which_list(kind: u8) -> Result<ConfigAddrKind> {
    match kind {
        1 => Ok(ConfigAddrKind::DepositAddr),
        2 => Ok(ConfigAddrKind::Senders),
        3 => Ok(ConfigAddrKind::WithdrawTos),
        _ => Err(DepositAgentError::InvalidConfigAddrType.into()),
    }
}

pub(crate) fn addr_list<'a>(
    config_data: &'a mut Account<ConfigData>,
    which: ConfigAddrKind,
) -> &'a mut Vec<Pubkey> {
    if let ConfigAddrKind::Senders = which {
        &mut config_data.senders
    } else {
        &mut config_data.withdraw_tos
    }
}

pub(crate) fn set_deposit_addr(
    config_data: &mut Account<ConfigData>,
    which: ConfigAddrKind,
    addr: Pubkey,
    remove: bool,
) -> Result<()> {
    match which {
        ConfigAddrKind::DepositAddr => {
            if config_data.deposit_addr.map(|a| a == addr) == Some(true) && !remove {
                return Err(DepositAgentError::AddrExists.into());
            }
            config_data.deposit_addr = Some(if remove { Pubkey::default() } else { addr });
            Ok(())
        }
        _ => Err(DepositAgentError::InvalidConfigAddrType.into()),
    }
}

pub(crate) fn check_config_addr_limit(
    which: ConfigAddrKind,
    len: usize,
) -> core::result::Result<(), DepositAgentError> {
    let limit = match which {
        ConfigAddrKind::DepositAddr => return Err(DepositAgentError::InvalidConfigAddrType),
        ConfigAddrKind::Senders => crate::state::ConfigData::MAX_SENDERS,
        ConfigAddrKind::WithdrawTos => crate::state::ConfigData::MAX_WITHDRAW_TOS,
    };
    if len > limit {
        return Err(DepositAgentError::ConfigAddrsExtendsLimit);
    }
    Ok(())
}
