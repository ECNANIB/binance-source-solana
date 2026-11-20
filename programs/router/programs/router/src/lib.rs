use anchor_lang::prelude::*;
declare_id!("B3111yJCeHBcA1bizdJjUFPALfhAfSRnAbJzGUtnt56A");
pub mod constants;
pub mod error;
pub mod instructions;

pub use constants::*;
pub use error::*;
pub use instructions::*;







#[program]
pub mod router {
    use super::*;

    #[instruction(discriminator = 1)]
    pub fn swap<'info>(
        ctx: Context<'_, '_, '_, 'info, Swap<'info>>,
        data: Vec<u8>,
    ) -> Result<()> {
        instructions::swap_with_fee_log::swap(ctx, data)
    }

    #[instruction(discriminator = 3)]
    pub fn swap_with_fee_log<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapWithFeeLog<'info>>,
        amount: u64,
        fee_bps: u16,
        decimals: u8,
        data: Vec<u8>,
    ) -> Result<()> {
        instructions::swap_with_fee_log::swap_with_fee_log(ctx, amount, fee_bps, decimals, &data)
    }

    #[instruction(discriminator = 4)]
    pub fn swap_with_sol_fee_log<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapWithSolFeeLog<'info>>,
        amount: u64,
        fee_bps: u16,
        data: Vec<u8>,
    ) -> Result<()> {
        instructions::swap_with_sol_fee_log::swap_with_sol_fee_log(ctx, amount, fee_bps, &data)
    }

    #[instruction(discriminator = 5)]
    pub fn take_fee_log<'info>(
        ctx: Context<'_, '_, '_, 'info, TakeFeeLog<'info>>,
        amount: u64,
        fee_bps: u16,
        decimals: u8,
    ) -> Result<()> {
        instructions::take_fee_log::take_fee_log(ctx, amount, fee_bps, decimals)
    }

    #[instruction(discriminator = 6)]
    pub fn take_sol_fee_log<'info>(
        ctx: Context<'_, '_, '_, 'info, TakeSolFeeLog<'info>>,
        amount: u64,
        fee_bps: u16,
    ) -> Result<()> {
        instructions::take_sol_fee_log::take_sol_fee_log(ctx, amount, fee_bps)
    }

    #[instruction(discriminator = 7)]
    pub fn batch_create_token_accounts<'info>(
        ctx: Context<'_, '_, '_, 'info, BatchCreateTokenAccounts<'info>>,
    ) -> Result<()> {
        instructions::batch_create_token_accounts::batch_create_token_accounts(ctx)
    }

    #[instruction(discriminator = 8)]
    pub fn sync_native<'info>(
        ctx: Context<'_, '_, '_, 'info, SyncNative<'info>>,
        amount: u64,
    ) -> Result<()> {
        instructions::sync_native::sync_native(ctx, amount)
    }

    #[instruction(discriminator = 9)]
    pub fn batch_close_token_accounts<'info>(
        ctx: Context<'_, '_, '_, 'info, BatchCloseTokenAccounts<'info>>,
    ) -> Result<()> {
        instructions::close_account::batch_close_token_accounts(ctx)
    }
}
