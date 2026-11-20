use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::system_program;

macro_rules! require_msg {
    ($invariant:expr, $err:path $(,)?) => {
        if !($invariant) {
            return Err(anchor_lang::error::Error::from($err(file!(), line!())));
        }
    };
}

#[event]
pub struct SplitPaymentEvent {
    pub id: u64,
    pub platform_amount: u64,
    pub split_amount: u64,
    pub token: bool,
    pub platform: Pubkey,
    pub split_contract: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SplitPaymentParams {
    pub id: u64,
    pub platform_amount: u64,
    pub split_amount: u64,
}
#[event_cpi]
#[derive(Accounts)]
#[instruction(params: SplitPaymentParams)]
pub struct SplitPayment<'info> {
    #[account(
        mut,
        constraint = config_data.addrs.contains(signer.key) @ crate::VaultError::NotAllowed
    )]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub token_holder: UncheckedAccount<'info>,
    #[account(
        seeds = [crate::CONFIG_DATA_SEED],
        bump = config_data.data_bump,
        constraint = !config_data.paused @ crate::VaultError::Paused
    )]
    pub config_data: Account<'info, ConfigData>,
    #[account(
        init,
        payer = signer,
        space = 1,
        seeds = [crate::SPLIT_PAYMENT_ID_SEED, &params.id.to_le_bytes()],
        bump
    )]
    pub split_payment_id_account: Account<'info, EmptyData>,

    #[account(mut, constraint = crate::addr_ok(platform.key, &config_data.platform) @ crate::VaultError::NotAllowed)]
    pub platform: UncheckedAccount<'info>,

    #[account(mut, constraint = crate::addr_ok(split_contract.key, &config_data.split_contract) @ crate::VaultError::NotAllowed)]
    pub split_contract: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

fn invalid_parameter(filename: &'static str, line: u32) -> anchor_lang::error::AnchorError {
    anchor_lang::error::AnchorError {
        error_name: crate::VaultError::InvalidParameter.name(),
        error_code_number: crate::VaultError::InvalidParameter.into(),
        error_msg: crate::VaultError::InvalidParameter.to_string(),
        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
            anchor_lang::error::Source { filename, line },
        )),
        compared_values: None,
    }
}

pub(crate) fn split_payment_handler<'info>(ctx: Context<'_, '_, 'info, 'info, SplitPayment<'info>>, params: SplitPaymentParams) -> Result<()> {
    require_msg!(params.id != 0, invalid_parameter);
    let before = ctx.accounts.token_holder.lamports();
    let holder_seeds: &[&[&[u8]]] =
        &[&[crate::TOKEN_HOLDER_SEED, &[ctx.accounts.config_data.token_holder_bump]]];
    let cpi_program = ctx.accounts.system_program.to_account_info();
    let cpi_accounts = system_program::Transfer {
        from: ctx.accounts.token_holder.to_account_info(),
        to: ctx.accounts.platform.to_account_info(),
    };
    let cpi_p = CpiContext::new_with_signer(
        cpi_program.clone(),
        cpi_accounts,
        holder_seeds,
    );
    system_program::transfer(cpi_p, params.platform_amount)?;
    let cpi_s = CpiContext::new_with_signer(
        cpi_program,
        system_program::Transfer {
            from: ctx.accounts.token_holder.to_account_info(),
            to: ctx.accounts.split_contract.to_account_info(),
        },
        holder_seeds);
    system_program::transfer(cpi_s, params.split_amount)?;
    let after = ctx.accounts.token_holder.lamports();


    require_msg!(
        after
            == before - params.platform_amount - params.split_amount,
        invalid_parameter
    );
    emit_cpi!(SplitPaymentEvent {
        id: params.id,
        platform_amount: params.platform_amount,
        split_amount: params.split_amount,
        token: false,
        platform: ctx.accounts.platform.key(),
        split_contract: ctx.accounts.split_contract.key(),
    });
    Ok(())
}
