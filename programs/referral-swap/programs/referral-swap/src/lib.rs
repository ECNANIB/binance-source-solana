use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("9VdG8gJNUo5r7MtGb4NHVP8tpnvoGZWJcByTGDXXtbY1");










#[program]
pub mod referral_swap {
    use super::*;

    #[instruction(discriminator = 1)]
    pub fn initialize(ctx: Context<Initial021>, owner: Pubkey) -> Result<()> {
        instructions::initialize::handler(ctx, owner)
    }

    #[instruction(discriminator = 2)]

    pub fn config_addr(
        ctx: Context<ConfigA021>,
        kind: u8,
        addr: Pubkey,
        remove: bool,
    ) -> Result<()> {
        instructions::config_addr::handler(ctx, kind, addr, remove)
    }






    #[instruction(discriminator = 3)]
    pub fn swap<'info>(
        ctx: Context<'_, '_, '_, 'info, S021<'info>>,
        params: SwapPar021,
    ) -> Result<()> {
        instructions::swap::handler(ctx, params)
    }

    #[instruction(discriminator = 4)]

    pub fn rebalance_to(ctx: Context<Rebalanc021>, amount: u64, decimals: u8) -> Result<()> {
        instructions::rebalance_to::handler(ctx, amount, decimals)
    }

    #[instruction(discriminator = 5)]
    pub fn pause(ctx: Context<Pa021>) -> Result<()> {
        instructions::pause::handler(ctx)
    }

    #[instruction(discriminator = 6)]
    pub fn resume(ctx: Context<Pa021>) -> Result<()> {
        instructions::resume::handler(ctx)
    }

    #[instruction(discriminator = 7)]
    pub fn close_wsol(ctx: Context<CloseW021>) -> Result<()> {
        instructions::close_wsol::handler(ctx)
    }

    #[instruction(discriminator = 8)]
    pub fn transfer_ownership(ctx: Context<TransferOwners021>) -> Result<()> {
        instructions::transfer_ownership::handler(ctx)
    }
}
