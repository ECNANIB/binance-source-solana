use anchor_lang::prelude::*;

pub mod error;










pub mod instructions;
pub mod state;

pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("NBAgSorjYjpnWXfJ7SwdhNUrzoXGJ9ysGqoYihrsNgp");

#[program]
pub mod claim {
    use super::*;

    pub fn is_claim_id_used(ctx: Context<IsClaimIdUsed>, claim_ids: Vec<u64>) -> Result<()> {
        instructions::is_claim_id_used::handler(ctx, claim_ids)
    }

    pub fn initialize(ctx: Context<Initialize>, owner: Pubkey) -> Result<()> {
        instructions::initialize::handler(ctx, owner)
    }

    pub fn config_signer(
        ctx: Context<ConfigureSigner>,
        kind: u8,
        pubkey: [u8; 64],
        remove: bool,
    ) -> Result<()> {
        instructions::config_signer::handler(ctx, kind, pubkey, remove)
    }

    pub fn pause(ctx: Context<Pause>) -> Result<()> {
        ctx.accounts.config_data.pause = true;
        Ok(())
    }

    pub fn resume(ctx: Context<Resume>) -> Result<()> {
        ctx.accounts.config_data.pause = false;
        Ok(())
    }

    pub fn big_nonce_initialize(
        ctx: Context<BigNonceInitialize>,
        args: BigNonceInitializeArgs,
    ) -> Result<()> {
        instructions::big_nonce::initialize::handler(ctx, args)
    }

    pub fn big_nonce_increase(
        ctx: Context<BigNonceIncrease>,
        args: BigNonceIncreaseArgs,
    ) -> Result<()> {
        instructions::big_nonce::increase::handler(ctx, args)
    }

    pub fn big_nonce_close(
        ctx: Context<BigNonceClose>,
        id: Vec<u8>,
        kind: u8,
        index: u64,
    ) -> Result<()> {
        instructions::big_nonce::close::handler(ctx, id, kind, index)
    }

    pub fn claim(ctx: Context<Claim>, args: ClaimArgs) -> Result<()> {
        instructions::claim::handler(ctx, args)
    }

    pub fn transfer_ownership(ctx: Context<TransferOwnership>) -> Result<()> {
        ctx.accounts.config_data.owner = ctx.accounts.new_owner.key();
        Ok(())
    }

    pub fn claim_token_batch<'info>(
        ctx: Context<'_, '_, 'info, 'info, ClaimTokenBatch<'info>>,
        args: ClaimTokenBatchArgs,
    ) -> Result<()> {
        instructions::claim_token_batch::handler(ctx, &args)
    }
}







pub(crate) fn source_error(error_code: ClaimError, filename: &'static str, line: u32) -> Error {
    Error::from(anchor_lang::error::AnchorError {
        error_name: error_code.name(),
        error_code_number: error_code.into(),
        error_msg: error_code.to_string(),
        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
            anchor_lang::error::Source { filename, line },
        )),
        compared_values: None,
    })
}






pub(crate) fn bare_error(error_code: ClaimError) -> Error {
    Error::from(anchor_lang::error::AnchorError {
        error_name: error_code.name(),
        error_code_number: error_code.into(),
        error_msg: error_code.to_string(),
        error_origin: None,
        compared_values: None,
    })
}










pub(crate) fn nonce_error(error_code: ClaimError) -> Error {
    Error::from(anchor_lang::error::AnchorError {
        error_name: error_code.name(),
        error_code_number: error_code.into(),
        error_msg: error_code.to_string(),
        error_origin: None,
        compared_values: None,
    })
}
