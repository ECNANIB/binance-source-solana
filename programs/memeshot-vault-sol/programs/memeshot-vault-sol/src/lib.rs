use anchor_lang::prelude::*;
declare_id!("BN111JnbLtbmQqqiCh7h2pDKhAhMx4wi77Mj7jJFbyp8");
pub mod errors;
pub mod instructions;
pub mod state;

pub use errors::*;
pub use instructions::*;
pub use state::*;




pub const SENDERS_LIMIT: usize = 16;

pub const WITHDRAW_TOS_LIMIT: usize = 4;


pub const CONFIG_DATA_SPACE: usize = 8
    + 1 + 1 + 1
    + 32 * 3
    + (4 + 32 * SENDERS_LIMIT)
    + (4 + 32 * WITHDRAW_TOS_LIMIT)
    + 32
    + 8;

#[program]
pub mod memeshot_vault_sol {
    use super::*;

    pub fn buy_to_alpha(ctx: Context<BuyToAlpha>, args: BuyToAlphaArgs) -> Result<()> {
        instructions::buy_to_alpha::handler(ctx, args)
    }

    pub fn sell_to_self_from_alpha(
        ctx: Context<SellToSelfFromAlpha>,
        args: SellToSelfFromAlphaArgs,
    ) -> Result<()> {
        instructions::sell_to_self_from_alpha::handler(ctx, args)
    }

    pub fn sync_native(ctx: Context<SyncNative>, amount: u64) -> Result<()> {
        instructions::sync_native::handler(ctx, amount)
    }

    pub fn close_wsol(ctx: Context<CloseWsol>) -> Result<()> {
        instructions::close_wsol::handler(ctx)
    }

    pub fn rebalance_out(ctx: Context<RebalanceOut>, decimals: u8, amount: u64) -> Result<()> {
        instructions::rebalance_out::handler(ctx, decimals, amount)
    }

    pub fn rebalance_out_sol(ctx: Context<RebalanceOutSol>, amount: u64) -> Result<()> {
        instructions::rebalance_out_sol::handler(ctx, amount)
    }

    pub fn initialize(
        ctx: Context<Initialize>,
        owner: Pubkey,
        agg_router: Pubkey,
        cloud_wallet_program: Pubkey,
        expire_duration: u64,
    ) -> Result<()> {
        instructions::initialize::handler(
            ctx,
            owner,
            agg_router,
            cloud_wallet_program,
            expire_duration,
        )
    }

    pub fn increase_config_data_space(
        ctx: Context<IncreaseConfigDataSpace>,
        space: u64,
    ) -> Result<()> {
        instructions::increase_config_data_space::handler(ctx, space)
    }

    pub fn config_addr(
        ctx: Context<ConfigAddr>,
        kind: u8,
        addr: Pubkey,
        remove: bool,
    ) -> Result<()> {
        instructions::config_addr::handler(ctx, kind, addr, remove)
    }

    pub fn set_partially_filled_remaining_pct_base(
        ctx: Context<SetPartiallyFilledRemainingPctBase>,
        pct_base: u64,
    ) -> Result<()> {
        instructions::set_partially_filled_remaining_pct_base::handler(ctx, pct_base)
    }

    pub fn emergency_withdraw(
        ctx: Context<EmergencyWithdraw>,
        route_data: Vec<u8>,
        decimals: u8,
        amount: u64,
    ) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[
            &route_data,
            &[ctx.accounts.config_data.partially_filled_remaining_pct_base],
        ]];
        let amount = if amount == 0 {
            ctx.accounts.token_holder_token_account.amount
        } else {
            amount
        };
        anchor_spl::token_interface::transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token_interface::TransferChecked {
                    from: ctx.accounts.token_holder_token_account.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.token_holder.to_account_info(),
                    mint: ctx.accounts.token_mint.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
            decimals,
        )
    }

    pub fn transfer_ownership(ctx: Context<TransferOwnership>) -> Result<()> {
        instructions::transfer_ownership::handler(ctx)
    }

    pub fn pause(ctx: Context<Pause>) -> Result<()> {
        instructions::pause::handler(ctx)
    }

    pub fn resume(ctx: Context<Resume>) -> Result<()> {
        instructions::resume::handler(ctx)
    }

    pub fn set_trade_token(ctx: Context<SetTradeToken>, trade_amount_limit: u64) -> Result<()> {
        instructions::set_trade_token::handler(ctx, trade_amount_limit)
    }

    pub fn remove_trade_token(ctx: Context<RemoveTradeToken>) -> Result<()> {
        instructions::remove_trade_token::handler(ctx)
    }
}
