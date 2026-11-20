use anchor_lang::prelude::*;

use crate::error::ReferralSwapError;
use crate::state::ConfigData;

#[derive(Accounts)]
pub struct ConfigA021<'info> {
    #[account(mut,
        constraint = config_data.owner == *signer.key @ ReferralSwapError::NotAllowed)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"config_data"],
        bump = config_data.bump
    )]
    pub config_data: Account<'info, ConfigData>,
    pub system_program: Program<'info, System>,
}





























fn slot_index(kind: u8) -> Result<u8> {
    match kind {
        1 => Ok(0),
        2 => Ok(1),
        3 => Ok(2),
        _ => Err(ReferralSwapError::InvalidParameter.into()),
    }
}

fn senders_mut(config_data: &mut ConfigData, idx: u8) -> Result<&mut Vec<Pubkey>> {
    if idx != 0 {
        return Err(ReferralSwapError::InvalidParameter.into());
    }

    Ok(&mut config_data.send021)
}

fn update_single(
    config_data: &mut Account<ConfigData>,
    idx: u8,
    addr: &Pubkey,
    remove: bool,
) -> Result<()> {
    let current = match idx {
        1 => &config_data.platform,
        2 => &config_data.router_program,
        _ => return Err(ReferralSwapError::InvalidParameter.into()),
    };
    if *current == *addr && !remove {
        return Err(ReferralSwapError::InvalidParameter.into());
    }
    let value = if remove {
        Pubkey::default()
    } else {
        require!(*addr != Pubkey::default(), ReferralSwapError::ZeroAddress);
        *addr
    };
    match idx {
        1 => config_data.platform = value,
        2 => config_data.router_program = value,
        _ => return Err(ReferralSwapError::InvalidParameter.into()),
    }


    Ok(())
}














pub(crate) fn handler(ctx: Context<ConfigA021>, kind: u8, addr: Pubkey, remove: bool) -> Result<()> {
    let config_data = &mut ctx.accounts.config_data;
    let idx = slot_index(kind)?;
    match idx {
        1 | 2 => return update_single(config_data, idx, &addr, remove),
        _ => {}
    }
    let addrs = senders_mut(config_data, idx)?;
    let exists = addrs.contains(&addr);
    if remove {
        if !exists {
            return Err(ReferralSwapError::InvalidParameter.into());
        }
        addrs.retain(|x| *x != addr);
    } else {
        if exists {
            return Err(ReferralSwapError::InvalidParameter.into());
        }
        require!(addr != Pubkey::default(), ReferralSwapError::ZeroAddress);
        addrs.push(addr);
    }

    if addrs.len() < 5 {
        Ok(())
    } else {
        Err(ReferralSwapError::InvalidParameter.into())
    }
}
