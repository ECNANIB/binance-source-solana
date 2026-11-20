use anchor_lang::prelude::*;
pub mod error; pub use error::*;
pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("nbEursuNQy9R8XeQH7yQYuKvVupqorCTmXJsvWskQGy");

pub const CONFIG_DATA_SEED: &[u8] = b"config_data";

pub const TOKEN_HOLDER_SEED: &[u8] = b"token_holder";

pub const TMP_TOKEN_HOLDER_SEED: &[u8] = b"tmp_token_holder";

pub const SPLIT_PAYMENT_ID_SEED: &[u8] = b"split_payment_id";


#[program]
pub mod vault {
    use super::*;

    pub fn split_payment<'info>(ctx: Context<'_, '_, 'info, 'info, SplitPayment<'info>>, params: SplitPaymentParams) -> Result<()> {
        instructions::split_payment::split_payment_handler(ctx, params)
    }

    pub fn config_addr(ctx: Context<ConfigAddr>, kind: u8, addr: Pubkey, remove: bool) -> Result<()> {
        instructions::initialize::config_addr_handler(ctx, kind, addr, remove)
    }

    pub fn close_config(ctx: Context<CloseConfig>) -> Result<()> {
        instructions::initialize::close_config_handler(ctx)
    }

    pub fn initialize(ctx: Context<Initialize>, owner: Pubkey) -> Result<()> {
        instructions::initialize::initialize_handler(ctx, owner)
    }

    pub fn pause(ctx: Context<Pause>) -> Result<()> {
        instructions::initialize::pause_handler(ctx)
    }

    pub fn resume(ctx: Context<Resume>) -> Result<()> {
        instructions::initialize::resume_handler(ctx)
    }

    pub fn close_wsol(ctx: Context<CloseWsol>) -> Result<()> {
        instructions::initialize::close_wsol_handler(ctx)
    }

    pub fn transfer_ownership(ctx: Context<TransferOwnership>) -> Result<()> {
        instructions::initialize::transfer_ownership_handler(ctx)
    }

    pub fn split_payment_token_batch<'info>(
        ctx: Context<'_, '_, 'info, 'info, SplitPaymentTokenBatch<'info>>,
        args: SplitPaymentTokenBatchArgs,
    ) -> Result<()> {
        instructions::split_payment_token_batch::split_payment_token_batch_handler(ctx, args)
    }
}

fn addr_ok(key: &Pubkey, expected: &Pubkey) -> bool {
    *key != Pubkey::default() && *key == *expected
}
