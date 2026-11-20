use anchor_lang::prelude::*;

declare_id!("BN111TK638zTi7uSXCKkzXYPSRt3rKNSDBK6VsjS772b");

pub mod errors;
pub mod instructions;
pub mod state;

pub use errors::*;
pub use instructions::*;
pub use state::*;




























#[program]
pub mod cloud_wallet {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, owner: Pubkey, token_holder: Pubkey) -> Result<()> {
        instructions::initialize(ctx, owner, token_holder)
    }

    pub fn config_addr(
        ctx: Context<ConfigAddr>,
        addr_type: u8,
        addr: Pubkey,
        remove: bool,
    ) -> Result<()> {
        instructions::config_addr(ctx, addr_type, addr, remove)
    }

    pub fn nonce_initialize(ctx: Context<NonceInitialize>, count: u64) -> Result<()> {
        instructions::nonce_initialize(ctx, count)
    }

    pub fn nonce_increase(ctx: Context<NonceIncrease>) -> Result<()> {
        instructions::nonce_increase(ctx)
    }

    pub fn nonce_close(ctx: Context<NonceClose>) -> Result<()> {
        instructions::nonce_close(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, args: DepositArgs) -> Result<()> {
        instructions::deposit(ctx, args)
    }

    pub fn quick_withdraw(ctx: Context<QuickWithdraw>, args: QuickWithdrawArgs) -> Result<()> {
        instructions::quick_withdraw(ctx, args)
    }

    pub fn close_wsol(ctx: Context<CloseWsol>) -> Result<()> {
        instructions::close_wsol(ctx)
    }

    pub fn emergency_withdraw(
        ctx: Context<EmergencyWithdraw>,
        token_decimals: u8,
        amount: u64,
    ) -> Result<()> {
        instructions::emergency_withdraw(ctx, token_decimals, amount)
    }

    pub fn emergency_withdraw_sol(ctx: Context<EmergencyWithdrawSol>, amount: u64) -> Result<()> {
        instructions::emergency_withdraw_sol(ctx, amount)
    }

    pub fn pause(ctx: Context<Pause>) -> Result<()> {
        instructions::pause(ctx)
    }

    #[instruction(discriminator = 101)]
    pub fn resume(ctx: Context<Resume>) -> Result<()> {
        instructions::resume(ctx)
    }

    pub fn transfer_ownership(ctx: Context<TransferOwnership>) -> Result<()> {
        instructions::transfer_ownership(ctx)
    }

    pub fn set_secp256k1_pubkey(
        ctx: Context<SetSecp256k1Pubkey>,
        addr_type: u8,
        pubkey: [u8; 64],
        remove: bool,
    ) -> Result<()> {
        instructions::set_secp256k1_pubkey(ctx, addr_type, pubkey, remove)
    }

    pub fn set_partially_filled_remaining_pct_base(
        ctx: Context<SetPartiallyFilledRemainingPctBase>,
        base: u64,
    ) -> Result<()> {
        instructions::set_partially_filled_remaining_pct_base(ctx, base)
    }

    pub fn add_withdraw_allowlist(ctx: Context<AddWithdrawAllowlist>) -> Result<()> {
        instructions::add_withdraw_allowlist(ctx)
    }

    pub fn remove_withdraw_allowlist(ctx: Context<RemoveWithdrawAllowlist>) -> Result<()> {
        instructions::remove_withdraw_allowlist(ctx)
    }

    pub fn big_nonce_initialize(
        ctx: Context<BigNonceInitialize>,
        args: BigNonceInitializeArgs,
    ) -> Result<()> {
        instructions::big_nonce_initialize(ctx, args)
    }

    pub fn big_nonce_increase(
        ctx: Context<BigNonceIncrease>,
        args: BigNonceIncreaseArgs,
    ) -> Result<()> {
        instructions::big_nonce_increase(ctx, args)
    }

    pub fn big_nonce_close(ctx: Context<BigNonceClose>, args: BigNonceCloseArgs) -> Result<()> {
        instructions::big_nonce_close(ctx, args)
    }

    pub fn withdraw(ctx: Context<Withdraw>, args: WithdrawArgs) -> Result<()> {
        instructions::withdraw(ctx, args)
    }

    #[instruction(discriminator = 0)]
    pub fn swap(ctx: Context<Swap>, args: SwapArgs) -> Result<()> {
        instructions::swap(ctx, args)
    }

    #[instruction(discriminator = 1)]
    pub fn sync_native(ctx: Context<SyncNative>, amount: u64) -> Result<()> {
        instructions::sync_native(ctx, amount)
    }

    pub fn add_withdraw_token_allowlist(ctx: Context<AddWithdrawTokenAllowlist>) -> Result<()> {
        instructions::add_withdraw_token_allowlist(ctx)
    }

    pub fn remove_withdraw_token_allowlist(
        ctx: Context<RemoveWithdrawTokenAllowlist>,
    ) -> Result<()> {
        instructions::remove_withdraw_token_allowlist(ctx)
    }

    pub fn withdraw_v2(ctx: Context<WithdrawV2>, args: WithdrawArgs) -> Result<()> {
        instructions::withdraw_v2(ctx, args)
    }

    pub fn upgrade_to_v4(ctx: Context<UpgradeToV4>, pubkey: [u8; 64]) -> Result<()> {
        instructions::upgrade_to_v4(ctx, pubkey)
    }

    pub fn user_deposit(ctx: Context<UserDeposit>, args: UserDepositArgs) -> Result<()> {
        instructions::user_deposit(ctx, args)
    }

}
