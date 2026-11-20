use anchor_lang::prelude::*;

declare_id!("BN111erRrQy7bpcZBEH7i9anCsrzkSHRFwmJhXiyWitm");

pub mod error;
pub mod instructions;
pub mod state;

pub use error::*;
use instructions::*;









#[program]
pub mod deposit_agent {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        owner: Pubkey,
        deposit_addr: Pubkey,
        expire_duration: u64,
        pubkey: [u8; 64],
    ) -> Result<()> {
        instructions::initialize::handler(ctx, owner, deposit_addr, expire_duration, pubkey)
    }

    pub fn config_addr(
        ctx: Context<ConfigAddr>,
        kind: u8,
        addr: Pubkey,
        remove: bool,
    ) -> Result<()> {
        let config_data = &mut ctx.accounts.config_data;
        let which = which_list(kind)?;

















        if (which == ConfigAddrKind::Senders) | (which == ConfigAddrKind::WithdrawTos) {
            let list = addr_list(config_data, which);
            let found = list.contains(&addr);
            if !found && remove {
                return Err(DepositAgentError::AddrNotExists.into());
            }
            if found && !remove {
                return Err(DepositAgentError::AddrExists.into());
            }
            if found {
                list.retain(|a| *a != addr);
            } else {
                list.push(addr);
            }
            check_config_addr_limit(which, list.len())?;
        } else {
            return set_deposit_addr(config_data, which, addr, remove);
        }
        Ok(())
    }

    pub fn set_secp256k1_pubkey(
        ctx: Context<SetSecp256k1Pubkey>,
        pubkey: [u8; 64],
        remove: bool,
    ) -> Result<()> {
        instructions::set_secp256k1_pubkey::handler(ctx, pubkey, remove)
    }

    pub fn pause(ctx: Context<Pause>) -> Result<()> {
        instructions::pause::handler(ctx)
    }

    pub fn resume(ctx: Context<Resume>) -> Result<()> {
        instructions::resume::handler(ctx)
    }

    pub fn transfer_ownership(ctx: Context<TransferOwnership>) -> Result<()> {
        instructions::transfer_ownership::handler(ctx)
    }

    pub fn collect_to(ctx: Context<CollectTo>, args: CollectToArgs) -> Result<()> {
        instructions::collect_to::handler(ctx, args)
    }

    pub fn sync_native(ctx: Context<SyncNative>, id: u128, amount: u64) -> Result<()> {
        let bump = ctx.bumps.token_holder;
        check_wsol_mint(ctx.accounts.wsol_mint.key)?;
        let seeds: &[&[u8]] = &[b"account_token_holder", &id.to_le_bytes(), &[bump]];
        let signer_seeds: &[&[&[u8]]] = &[seeds];











        anchor_lang::system_program::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.token_holder.to_account_info(),
                    to: ctx.accounts.wsol_account.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
        )?;
        let cpi2 = {
            let token_program_info = ctx.accounts.token_program.to_account_info();
            match *token_program_info.key {
                anchor_spl::token::ID | anchor_spl::token_2022::ID => {}
                _ => return Err(DepositAgentError::InvalidProgram.into()),
            }
            CpiContext::new_with_signer(
                token_program_info,
                anchor_spl::token_interface::SyncNative {
                    account: ctx.accounts.wsol_account.to_account_info(),
                },
                signer_seeds,
            )
        };
        anchor_spl::token_interface::sync_native(cpi2)?;
        Ok(())
    }

    pub fn get_address(ctx: Context<GetAddress>, id: u128) -> Result<()> {
        instructions::get_address::handler(ctx, id)
    }
}
