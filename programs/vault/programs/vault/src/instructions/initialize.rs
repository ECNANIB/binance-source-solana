use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{self, CloseAccount, TokenAccount, TokenInterface, TransferChecked};
use super::{CloseConfig, CloseWsol, ConfigAddr, TransferOwnership};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + 232,
        seeds = [crate::CONFIG_DATA_SEED],
        bump
    )]
    pub config_data: Account<'info, ConfigData>,

    #[account(
        seeds = [crate::TOKEN_HOLDER_SEED],
        bump
    )]
    pub token_holder: UncheckedAccount<'info>,

    #[account(
        seeds = [crate::TMP_TOKEN_HOLDER_SEED],
        bump
    )]
    pub tmp_token_holder: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Pause<'info> {
    #[account(
        mut,
        constraint = config_data.owner == *signer.key @ crate::VaultError::NotAllowed
    )]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [crate::CONFIG_DATA_SEED],
        bump = config_data.data_bump
    )]
    pub config_data: Account<'info, ConfigData>,
}

#[derive(Accounts)]
pub struct Resume<'info> {
    #[account(
        mut,
        constraint = config_data.owner == *signer.key @ crate::VaultError::NotAllowed
    )]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [crate::CONFIG_DATA_SEED],
        bump = config_data.data_bump
    )]
    pub config_data: Account<'info, ConfigData>,
}

pub(crate) fn initialize_handler(ctx: Context<Initialize>, owner: Pubkey) -> Result<()> {
    let c = &mut ctx.accounts.config_data;
    c.owner = owner;
    c.data_bump = ctx.bumps.config_data;
    c.token_holder_bump = ctx.bumps.token_holder;
    c.tmp_bump = ctx.bumps.tmp_token_holder;
    Ok(())
}

fn config_addr_index(kind: u8) -> Result<u8> {
    match kind {
        1 => Ok(0),
        2 => Ok(1),
        3 => Ok(2),
        _ => Err(crate::VaultError::InvalidParameter.into()),
    }
}

fn config_addr_zero(c: &mut ConfigData, idx: u8) -> Result<&mut Vec<Pubkey>> {
    if idx != 0 {
        return Err(crate::VaultError::InvalidParameter.into());
    }
    Ok(&mut c.addrs)
}

fn config_addr_store(
    c: &mut Account<'_, ConfigData>,
    idx: u8,
    addr: &Pubkey,
    remove: bool,
) -> Result<()> {
    let same = if idx == 1 { c.platform == *addr } else { c.split_contract == *addr };

    if same && !remove {
        return Err(crate::VaultError::InvalidParameter.into());
    }
    let newv = &(!remove).then(|| *addr).unwrap_or_default();
    if idx != 1 {
        c.split_contract = *newv;
    } else {
        c.platform = *newv;
    }

    Ok(())
}

fn config_addr_list(v: &mut Vec<Pubkey>, idx: u8, addr: Pubkey, remove: bool) -> Result<()> {
    let has = v.contains(&addr);
    if !has && remove {
            return Err(crate::VaultError::InvalidParameter.into());
    }
    if has && !remove {
            return Err(crate::VaultError::InvalidParameter.into());
    }
    if has {
        v.retain(|a| *a != addr);
    } else {
        v.push(addr);
    }
    config_addr_limit(idx, v.len())
}

fn config_addr_limit(idx: u8, len: usize) -> Result<()> {
    let limit = match idx {
        0 => 4,
        _ => return Err(crate::VaultError::InvalidParameter.into()),
    };
    if len > limit {
        return Err(crate::VaultError::InvalidParameter.into());
    }
    Ok(())
}

pub(crate) fn config_addr_handler(
    ctx: Context<ConfigAddr>,
    kind: u8,
    addr: Pubkey,
    remove: bool,
) -> Result<()> {
    let c = &mut ctx.accounts.config_data;
    let idx: u8 = config_addr_index(kind)?;
    match idx {
        1..=2 => config_addr_store(c, idx, &addr, remove),
        _ => {
            let v: &mut Vec<Pubkey> =
                config_addr_zero(c, idx)?;
            config_addr_list(v, idx, addr, remove)
        }
    }
}

pub(crate) fn close_config_handler(ctx: Context<CloseConfig>) -> Result<()> {

    Ok(())
}

pub(crate) fn pause_handler(ctx: Context<Pause>) -> Result<()> {
    ctx.accounts.config_data.paused = true;
    Ok(())
}

pub(crate) fn resume_handler(ctx: Context<Resume>) -> Result<()> {
    ctx.accounts.config_data.paused = false;
    Ok(())
}
fn check_wsol_mint(mint: &Pubkey) -> Result<()> {
    const WSOL_MINT: Pubkey = Pubkey::new_from_array([
        0x06, 0x9b, 0x88, 0x57, 0xfe, 0xab, 0x81, 0x84, 0xfb, 0x68, 0x7f, 0x63, 0x46, 0x18, 0xc0,
        0x35, 0xda, 0xc4, 0x39, 0xdc, 0x1a, 0xeb, 0x3b, 0x55, 0x98, 0xa0, 0xf0, 0x00, 0x00, 0x00,
        0x00, 0x01,
    ]);
    const ALT_MINT: Pubkey = Pubkey::new_from_array([
        0x83, 0x0d, 0xfc, 0x9f, 0xde, 0x5f, 0xe6, 0xb8, 0xaa, 0x7c, 0x04, 0xa4, 0x76, 0xe9, 0x1e,
        0x8a, 0xc6, 0xbb, 0x26, 0x4a, 0xad, 0x90, 0xfa, 0x19, 0xc9, 0xdf, 0x49, 0xd8, 0x5c, 0x3e,
        0x5b, 0x5e,
    ]);
    match *mint { WSOL_MINT | ALT_MINT => Ok(()), _ => Err(crate::VaultError::InvalidParameter.into()) }
}
pub(crate) fn close_wsol_handler(ctx: Context<CloseWsol>) -> Result<()> {
    check_wsol_mint(ctx.accounts.wsol_mint.key)?;
    if ctx.accounts.token_holder_wsol_account.amount == 0 {
        return Ok(());
    }
    let before = ctx.accounts.token_holder.lamports();
    let amount = ctx.accounts.token_holder_wsol_account.amount;
    {

    let signer_a: &[&[&[u8]]] = &[&[crate::TOKEN_HOLDER_SEED, &[ctx.accounts.config_data.token_holder_bump]]];
    let cpi_a = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.token_holder_wsol_account.to_account_info(),
            to: ctx.accounts.tmp_token_holder_wsol_account.to_account_info(),
            authority: ctx.accounts.token_holder.to_account_info(),
            mint: ctx.accounts.wsol_mint.to_account_info(),
        },
        signer_a,
    );
    token_interface::transfer_checked(cpi_a, amount, 9)?; }
    let signer_b: &[&[&[u8]]] =
        &[&[crate::TMP_TOKEN_HOLDER_SEED, &[ctx.accounts.config_data.tmp_bump]]];
    let cpi_b = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.tmp_token_holder_wsol_account.to_account_info(),
            destination: ctx.accounts.token_holder.to_account_info(),
            authority: ctx.accounts.tmp_token_holder.to_account_info(),
        },
        signer_b,
    );
    token_interface::close_account(cpi_b)?;
    let after = ctx.accounts.token_holder.lamports();
    let md = after.checked_sub(before).and_then(|d| d.checked_sub(amount));
    let extra = match md { Some(v) => v, None => return Err(crate::VaultError::MathOverflow.into()) };
    if extra != 0 {
        let signer_c: &[&[&[u8]]] =
            &[&[crate::TOKEN_HOLDER_SEED, &[ctx.accounts.config_data.token_holder_bump]]];

        let prog_c = ctx.accounts.system_program.to_account_info(); let acc_c = system_program::Transfer { from: ctx.accounts.token_holder.to_account_info(), to: ctx.accounts.signer.to_account_info() }; let cpi_c =
            CpiContext::new_with_signer(
                prog_c,
                acc_c,
                signer_c,
            );
        system_program::transfer(cpi_c, extra)?;

    }
    Ok(())
}

pub(crate) fn transfer_ownership_handler(ctx: Context<TransferOwnership>) -> Result<()> {
    ctx.accounts.config_data.owner = ctx.accounts.new_owner.key();
    Ok(())
}
