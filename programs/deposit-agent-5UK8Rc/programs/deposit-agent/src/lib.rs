use anchor_lang::prelude::*;
declare_id!("5UK8RcSv4DiDpHSEsBu8y9qnqzizdyDsNbNpSRdc4Ko");
pub mod error;
pub use error::*;
pub mod instructions;
pub mod state;
pub use instructions::*;
pub(crate) use instructions::{__client_accounts_collect_to, __cpi_client_accounts_collect_to, __client_accounts_config_addr, __cpi_client_accounts_config_addr, __client_accounts_initialize, __cpi_client_accounts_initialize, __client_accounts_set_secp256k1_pubkey, __cpi_client_accounts_set_secp256k1_pubkey, __client_accounts_sync_native_instruction, __cpi_client_accounts_sync_native_instruction, __client_accounts_pause, __cpi_client_accounts_pause, __client_accounts_resume, __cpi_client_accounts_resume, __client_accounts_transfer_ownership, __cpi_client_accounts_transfer_ownership};
pub use state::*;
pub const MAX_CONFIG_ADDRS: usize = 16;
pub const MAX_OPERATORS: usize = 4;
pub const CONFIG_DATA_SEED: &[u8] = b"config_data";
pub const TOKEN_HOLDER_SEED: &[u8] = b"account_token_holder";

#[derive(Accounts)]
pub struct GetAddress {}



#[program]
pub mod deposit_agent {
    use super::*;
    use anchor_lang::solana_program::hash;
    use anchor_lang::solana_program::secp256k1_recover::secp256k1_recover;
    use anchor_spl::token_interface::{self, SyncNative, TransferChecked};
    use crate::instructions::sync_native::check_wsol_mint;

    pub fn initialize(
        ctx: Context<Initialize>,
        owner: Pubkey,
        program_addr: Pubkey,
        expiration: u64,
        secp256k1_pubkey: [u8; 64],
    ) -> Result<()> {
        let config_data = &mut ctx.accounts.config_data;
        config_data.bump = ctx.bumps.config_data;
        config_data.expiration = expiration;
        config_data.owner = owner;
        config_data.program_addr = Some(program_addr);
        config_data.secp256k1_pubkeys = vec![secp256k1_pubkey];
        Ok(())
    }

    pub fn config_addr(
        ctx: Context<ConfigAddr>,
        kind: u8,
        addr: Pubkey,
        remove: bool,
    ) -> Result<()> {
        instructions::config_addr::handler(ctx, kind, addr, remove)
    }

    pub fn set_secp256k1_pubkey(
        ctx: Context<SetSecp256k1Pubkey>,
        pubkey: [u8; 64],
        remove: bool,
    ) -> Result<()> {
        instructions::set_secp256k1_pubkey::handler(ctx, pubkey, remove)
    }

    pub fn pause(ctx: Context<Pause>) -> Result<()> {
        ctx.accounts.config_data.paused = true;
        Ok(())
    }

    pub fn resume(ctx: Context<Resume>) -> Result<()> {
        ctx.accounts.config_data.paused = false;
        Ok(())
    }

    pub fn transfer_ownership(ctx: Context<TransferOwnership>) -> Result<()> {
        ctx.accounts.config_data.owner = ctx.accounts.new_owner.key();
        Ok(())
    }

    pub fn collect_to(ctx: Context<CollectTo>, args: CollectToArgs) -> Result<()> {
        instructions::collect_to::handler(ctx, args)
    }

    pub fn sync_native(ctx: Context<SyncNativeInstruction>, id: u128, amount: u64) -> Result<()> {
        let bump = ctx.bumps.token_holder;
        check_wsol_mint(ctx.accounts.wsol_mint.key)?;
        instructions::sync_native::handler(ctx, id, amount, bump)
    }

    pub fn get_address(ctx: Context<GetAddress>, id: u128) -> Result<()> {
        instructions::get_address::handler(ctx, id)
    }
}
