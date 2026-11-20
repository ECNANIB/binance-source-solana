use anchor_lang::prelude::*;

use crate::state::ConfigData;
use crate::ErrorCode;



#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer = payer, space = 8 + 1476, seeds = [b"config_data"], bump)]
    pub config_data: Account<'info, ConfigData>,
    #[account(seeds = [b"token_holder"], bump)]
    pub token_holder: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct Pause<'info> {
    #[account(mut, constraint = config_data.addresses_2.contains(signer.key) @ ErrorCode::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [b"config_data"], bump = config_data.bump_b2)]
    pub config_data: Account<'info, ConfigData>,
}










pub(crate) fn handler(ctx: Context<Initialize>, manager: Pubkey, owner: Pubkey, pct_base: u64) -> Result<()> {
    ctx.accounts.config_data.addresses.push(manager);
    ctx.accounts.config_data.pubkey_60 = owner;
    ctx.accounts.config_data.partially_filled_remaining_pct_base = pct_base;
    ctx.accounts.config_data.bump_b2 = ctx.bumps.config_data;
    ctx.accounts.config_data.bump_b3 = ctx.bumps.token_holder;
    Ok(())
}





pub fn upgrade_handler(_ctx: Context<super::UpgradeToV2>, _a: [u8; 72]) -> Result<()> {
    Err(ErrorCode::SenderNotAllowed.into())
}













#[derive(Clone, Copy, PartialEq)]
pub enum ConfigAddrType {
    Addresses,
    Addresses1,
    Addresses2,
    Pubkey80,
    Pubkey60,
}
fn config_addr_slot(kind: u8) -> Result<ConfigAddrType> {
    match kind {
        1 => Ok(ConfigAddrType::Addresses),
        2 => Ok(ConfigAddrType::Addresses1),
        3 => Ok(ConfigAddrType::Addresses2),
        4 => Ok(ConfigAddrType::Pubkey80),
        5 => Ok(ConfigAddrType::Pubkey60),
        _ => Err(ErrorCode::InvalidConfigAddrType.into()),
    }
}
























pub(crate) fn config_addr_handler(
    ctx: Context<super::ConfigAddr>,
    kind: u8,
    addr: Pubkey,
    remove: bool,
) -> Result<()> {
    let config_data = &mut ctx.accounts.config_data;
    let kind = config_addr_slot(kind)?;
    if (kind == ConfigAddrType::Addresses) || (kind == ConfigAddrType::Addresses1) || (kind == ConfigAddrType::Addresses2) {
        let list = config_addr_list(config_data, kind);
        let found = list.contains(&addr);
        if found && !remove {
            return Err(ErrorCode::AddrExists.into());
        }
        if !found && remove {
            return Err(ErrorCode::AddrNotExists.into());
        }
        if found {
            list.retain(|p| *p != addr);
        } else {
            list.push(addr);
        }
        check_config_addr_limit(kind, list.len())?;
    } else {
        return set_config_addr_single(config_data, kind, addr, remove);
    }
    Ok(())
}





















pub fn pause_handler(ctx: Context<Pause>) -> Result<()> {
    ctx.accounts.config_data.paused = true;
    Ok(())
}

pub fn resume_handler(ctx: Context<super::Resume>) -> Result<()> {
    ctx.accounts.config_data.paused = false;
    Ok(())
}
fn set_config_addr_single(config_data: &mut Account<'_, ConfigData>, kind: ConfigAddrType, addr: Pubkey, remove: bool) -> Result<()> {
    let is60 = matches!(kind, ConfigAddrType::Pubkey60);
    let cur = match kind {
        ConfigAddrType::Pubkey80 => &config_data.pubkey_80,
        ConfigAddrType::Pubkey60 => &config_data.pubkey_60,
        _ => return Err(ErrorCode::InvalidConfigAddrType.into()),
    };
    if *cur == addr && !remove {
        return Err(ErrorCode::AddrExists.into());
    }
    let v = if remove { Pubkey::default() } else { addr };
    if is60 {
        config_data.pubkey_60 = v;
    } else {
        config_data.pubkey_80 = v;
    }
    Ok(())
}
fn config_addr_list<'a>(config_data: &'a mut Account<'_, ConfigData>, kind: ConfigAddrType) -> &'a mut Vec<Pubkey> {
    match kind {
        ConfigAddrType::Addresses => &mut config_data.addresses,
        ConfigAddrType::Addresses1 => &mut config_data.addresses_1,
        _ => &mut config_data.addresses_2,
    }
}
pub fn check_config_addr_limit(kind: ConfigAddrType, len: usize) -> core::result::Result<(), ErrorCode> {
    let limit = match kind {
        ConfigAddrType::Addresses => 3,
        ConfigAddrType::Addresses1 => 16,
        ConfigAddrType::Addresses2 => 4,
        ConfigAddrType::Pubkey80 | ConfigAddrType::Pubkey60 => return Err(ErrorCode::InvalidConfigAddrType),
    };
    if len > limit {
        return Err(ErrorCode::ConfigAddrsExtendsLimit);
    }
    Ok(())
}
