use anchor_lang::prelude::*;

declare_id!("BN111AnCthcdPVNJ6jkir9TDaS7xqXT8EhetAmYpNqFt");

pub mod errors;
pub mod instructions;
pub mod state;

pub use errors::ErrorC002 as ErrorCode;
pub use instructions::*;
pub use state::*;



















#[program]
pub mod w3w_vault {
    use super::*;

    pub fn set_partially_filled_remaining_pct_base(
        ctx: Context<SetPartiallyFilledRemainingPctBase>,
        pct_base: u64,
    ) -> Result<()> {
        instructions::set_partially_filled_remaining_pct_base::handler(ctx, pct_base)
    }

    pub fn set_trade_token(ctx: Context<SetTradeTo003>, arg0: u64) -> Result<()> {
        instructions::set_trade_token::handler(ctx, arg0)
    }

    pub fn remove_trade_token(ctx: Context<RemoveTradeTo003>) -> Result<()> {
        instructions::set_trade_token::remove_handler(ctx)
    }

    pub fn set_secp256k1_pubkey(ctx: Context<SetSecp256k1Pubkey>, pubkey: [u8; 64], flag: bool) -> Result<()> {
        instructions::set_secp256k1_pubkey::handler(ctx, pubkey, flag)
    }

    pub fn upgrade_to_v2(ctx: Context<UpgradeToV2>, a: [u8; 72]) -> Result<()> {
        instructions::initialize::upgrade_handler(ctx, a)
    }

    #[instruction(discriminator = 0)]
    pub fn swap_to_token<'info>(ctx: Context<'_, '_, '_, 'info, SwapToToken<'info>>, params: SwapToTokenParams) -> Result<()> {
        instructions::swap_to_token::handler(ctx, params)
    }

    #[instruction(discriminator = 2)]
    pub fn sync_native(ctx: Context<SyncNat003>, amount: u64) -> Result<()> {
        instructions::sync_native::handler(ctx, amount)
    }

    #[instruction(discriminator = 3)]
    pub fn swap_to_sol<'info>(ctx: Context<'_, '_, '_, 'info, SwapTo003<'info>>, params: SwapToSolParams) -> Result<()> {
        instructions::swap_to_sol::handler(ctx, params)
    }

    #[instruction(discriminator = 4)]
    pub fn swap_to_token_v2<'info>(ctx: Context<'_, '_, '_, 'info, SwapToTokenV2<'info>>, params: SwapToTokenV2Params) -> Result<()> {
        instructions::swap_to_token_v2::handler(ctx, params)
    }

    #[instruction(discriminator = 5)]
    pub fn swap_to_sol_v2<'info>(ctx: Context<'_, '_, '_, 'info, SwapToSo001<'info>>, params: SwapToSolV2Params) -> Result<()> {
        instructions::swap_to_sol_v2::handler(ctx, params)
    }

    #[instruction(discriminator = 6)]
    pub fn batch_create_token_accounts<'info>(ctx: Context<'_, '_, '_, 'info, BatchCreateTokenAccounts<'info>>) -> Result<()> {
        instructions::batch_create_token_accounts::handler(ctx)
    }

    pub fn close_wsol(ctx: Context<CloseW003>) -> Result<()> {
        instructions::sync_native::close_handler(ctx)
    }

    pub fn close_nonce_account<'info>(ctx: Context<'_, '_, '_, 'info, CloseNonceAcco001<'info>>, params: CloseNonceAccountParams) -> Result<()> {
        instructions::close_nonce_account::handler(ctx, &params)
    }

    pub fn rebalance_out(ctx: Context<RebalanceOut>, kind: u8, amount: u64) -> Result<()> {
        instructions::rebalance_out_handler(ctx, kind, amount)
    }

    pub fn rebalance_out_sol(ctx: Context<RebalanceOutSol>, amount: u64) -> Result<()> {
        instructions::rebalance_out_sol_handler(ctx, amount)
    }

    pub fn initialize(ctx: Context<Initialize>, manager: Pubkey, owner: Pubkey, pct_base: u64) -> Result<()> {
        instructions::initialize::handler(ctx, manager, owner, pct_base)
    }

    pub fn config_addr(ctx: Context<ConfigAddr>, kind: u8, addr: Pubkey, flag: bool) -> Result<()> {
        instructions::initialize::config_addr_handler(ctx, kind, addr, flag)
    }

    pub fn emergency_withdraw(ctx: Context<EmergencyWithdraw>, data: Vec<u8>, kind: u8, amount: u64) -> Result<()> {
        use anchor_spl::token_interface::{self, TransferChecked};
        let signer_seeds: &[&[&[u8]]] = &[&[&data, &[ctx.accounts.config_data.bump_b3]]];









        let amount = if amount == 0 { ctx.accounts.token_holder_token_account.amount } else { amount };
        token_interface::transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.token_holder_token_account.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.token_holder.to_account_info(),
                    mint: ctx.accounts.token_mint.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
            kind,
        )?;
        Ok(())
    }

    pub fn pause(ctx: Context<Pause>) -> Result<()> {
        instructions::initialize::pause_handler(ctx)
    }

    pub fn resume(ctx: Context<Resume>) -> Result<()> {
        instructions::initialize::resume_handler(ctx)
    }
}
