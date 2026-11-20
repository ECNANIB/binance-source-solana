use anchor_lang::prelude::*;

declare_id!("nbEkL7pwLvMdDToFY8bsoTUyFJ5Y9NtJvW2wMHMWzwH");

pub mod error;
pub mod instructions;
pub mod constants;

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
        fee_amount: u64,
        commission_rate: u16,
        wrap_sol: u8,
        data: Vec<u8>,
    ) -> Result<()> {
        instructions::swap_with_fee_log::swap_with_fee_log(
            ctx,
            fee_amount,
            commission_rate,
            wrap_sol,
            &data,
        )
    }

    #[instruction(discriminator = 4)]
    pub fn swap_with_sol_fee_log<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapWithSolFeeLog<'info>>,
        fee_amount: u64,
        commission_rate: u16,
        data: Vec<u8>,
    ) -> Result<()> {
        instructions::swap_with_sol_fee_log::swap_with_sol_fee_log(
            ctx,
            fee_amount,
            commission_rate,
            &data,
        )
    }

    #[instruction(discriminator = 5)]
    pub fn take_fee_log<'info>(
        ctx: Context<'_, '_, '_, 'info, TakeFeeLog<'info>>,
        fee_amount: u64,
        commission_rate: u16,
        decimals: u8,
    ) -> Result<()> {
        instructions::take_fee_log::take_fee_log(ctx, fee_amount, commission_rate, decimals)
    }

    #[instruction(discriminator = 6)]
    pub fn take_sol_fee_log<'info>(
        ctx: Context<'_, '_, '_, 'info, TakeSolFeeLog<'info>>,
        fee_amount: u64,
        commission_rate: u16,
    ) -> Result<()> {
        instructions::take_sol_fee_log::take_sol_fee_log(ctx, fee_amount, commission_rate)
    }

    #[instruction(discriminator = 7)]
    pub fn batch_create_token_accounts<'info>(
        ctx: Context<'_, '_, '_, 'info, BatchCreateTokenAccou001<'info>>,
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

    #[instruction(discriminator = 10)]
    pub fn swap_with_positive_slippage<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapWithPositiveSlippage<'info>>,
        original_amount: u64,
        commission_rate: u16,
        wrap_sol: bool,
        fee_amount: u64,
        data: Vec<u8>,
    ) -> Result<()> {
        instructions::swap_with_positive_slippage::swap_with_positive_slippage(
            ctx,
            original_amount,
            commission_rate,
            wrap_sol,
            fee_amount,
            &data,
        )
    }

    #[instruction(discriminator = 11)]
    pub fn swap_with_sol_positive_slippage<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapWithSolPositiveSlippage<'info>>,
        original_amount: u64,
        commission_rate: u16,
        wrap_sol: bool,
        fee_amount: u64,
        data: Vec<u8>,
    ) -> Result<()> {
        instructions::swap_with_sol_positive_slippage::swap_with_sol_positive_slippage(
            ctx,
            original_amount,
            commission_rate,
            wrap_sol,
            fee_amount,
            &data,
        )
    }

    #[instruction(discriminator = 13)]
    pub fn swap_with_positive_slippage_v2<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapWithPositiveSlippageV2<'info>>,
        original_amount: u64,
        commission_rate: u32,
        fee_amount: u64,
        slippage_bps: u16,
        data: Vec<u8>,
    ) -> Result<()> {
        instructions::swap_with_positive_slippage_v2::swap_with_positive_slippage_v2(
            ctx,
            original_amount,
            commission_rate,
            fee_amount,
            slippage_bps,
            &data,
        )
    }

    #[instruction(discriminator = 14)]
    pub fn swap_with_sol_positive_slippage_v2<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapWithSolPositiveSlippageV2<'info>>,
        original_amount: u64,
        commission_rate: u32,
        fee_amount: u64,
        slippage_bps: u16,
        data: Vec<u8>,
    ) -> Result<()> {
        instructions::swap_with_sol_positive_slippage_v2::swap_with_sol_positive_slippage_v2(
            ctx,
            original_amount,
            commission_rate,
            fee_amount,
            slippage_bps,
            &data,
        )
    }

    #[instruction(discriminator = 15)]
    pub fn swap_with_positive_slippage_v3<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapWithPositiveSlippage<'info>>,
        order_id: u64,
        original_amount: u64,
        commission_rate: u16,
        wrap_sol: bool,
        fee_amount: u64,
        data: Vec<u8>,
    ) -> Result<()> {
        let _ = order_id;
        instructions::swap_with_positive_slippage::swap_with_positive_slippage(
            ctx,
            original_amount,
            commission_rate,
            wrap_sol,
            fee_amount,
            &data,
        )
    }

    #[instruction(discriminator = 16)]
    pub fn swap_with_sol_positive_slippage_v3<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapWithSolPositiveSlippage<'info>>,
        order_id: u64,
        original_amount: u64,
        commission_rate: u16,
        wrap_sol: bool,
        fee_amount: u64,
        data: Vec<u8>,
    ) -> Result<()> {
        instructions::swap_with_sol_positive_slippage::swap_with_sol_positive_slippage(
            ctx,
            original_amount,
            commission_rate,
            wrap_sol,
            fee_amount,
            &data,
        )
    }

    #[instruction(discriminator = 17)]
    pub fn swap_with_positive_slippage_v4<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapWithPositiveSlippageV2<'info>>,
        order_id: u64,
        original_amount: u64,
        commission_rate: u32,
        fee_amount: u64,
        slippage_bps: u16,
        data: Vec<u8>,
    ) -> Result<()> {
        instructions::swap_with_positive_slippage_v2::swap_with_positive_slippage_v2(
            ctx,
            original_amount,
            commission_rate,
            fee_amount,
            slippage_bps,
            &data,
        )
    }

    #[instruction(discriminator = 18)]
    pub fn swap_with_sol_positive_slippage_v4<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapWithSolPositiveSlippageV2<'info>>,
        order_id: u64,
        original_amount: u64,
        commission_rate: u32,
        fee_amount: u64,
        slippage_bps: u16,
        data: Vec<u8>,
    ) -> Result<()> {
        instructions::swap_with_sol_positive_slippage_v2::swap_with_sol_positive_slippage_v2(
            ctx,
            original_amount,
            commission_rate,
            fee_amount,
            slippage_bps,
            &data,
        )
    }

    #[instruction(discriminator = 12)]
    pub fn swap_with_dynamic_input<'info>(
        ctx: Context<'_, '_, 'info, 'info, SwapWithDynamicInput>,
        keep_amount: u64,
        min_out_amount: u64,
        data: Vec<u8>,
    ) -> Result<()> {
        instructions::swap_with_dynamic_input::swap_with_dynamic_input_inner(
            ctx,
            keep_amount,
            min_out_amount,
            &data,
        )
    }

    #[instruction(discriminator = 19)]
    pub fn begin_token_delta_window<'info>(
        ctx: Context<'_, '_, '_, 'info, BeginTokenDeltaWindow<'info>>,
    ) -> Result<()> {
        instructions::token_delta_window::begin_token_delta_window(ctx)
    }

    #[instruction(discriminator = 20)]
    pub fn settle_token_delta_window<'info>(
        ctx: Context<'_, '_, '_, 'info, SettleTokenDeltaWin00y<'info>>,
        min_delta: u64,
    ) -> Result<()> {
        instructions::token_delta_window::settle_token_delta_window(ctx, min_delta)
    }

    #[instruction(discriminator = 21)]
    pub fn begin_lamport_delta_window<'info>(
        ctx: Context<'_, '_, '_, 'info, BeginLamportDeltaWindow<'info>>,
    ) -> Result<()> {
        instructions::lamport_delta_window::begin_lamport_delta_window(ctx)
    }

    #[instruction(discriminator = 22)]
    pub fn settle_lamport_delta_window<'info>(
        ctx: Context<'_, '_, '_, 'info, SettleLamportDeltaWindow<'info>>,
        min_delta: u64,
    ) -> Result<()> {
        instructions::lamport_delta_window::settle_lamport_delta_window(ctx, min_delta)
    }
}
