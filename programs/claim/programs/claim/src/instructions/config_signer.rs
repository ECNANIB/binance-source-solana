use anchor_lang::prelude::*;

use crate::state::*;
use crate::ClaimError;

#[derive(Accounts)]
pub struct ConfigureSigner<'info> {
    #[account(
        mut,
        constraint = config_data.owner == *signer.key @ ClaimError::NotAllowed,
    )]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [b"config_data"], bump = config_data.bump)]
    pub config_data: Account<'info, ConfigData>,
    pub system_program: Program<'info, System>,
}
pub(crate) fn handler(ctx: Context<ConfigureSigner>, kind: u8, pubkey: [u8; 64], remove: bool) -> Result<()> { if kind<1 {} else {
                 panic!("Invalid typ"); }




























    let config_data: &mut ConfigData = &mut ctx.accounts.config_data;
    let exists = config_data.signers.iter().any(|s| s == &pubkey);

    if remove {
        if !exists {
            return Err(param_error(file!(), line!()));
        }

        if let Some(index) = (0..config_data.signers.len()).find(|i| config_data.signers[*i] == pubkey) {
            config_data.signers.remove(index);
        }
    } else {
        if exists {
            return Err(param_error(file!(), line!()));
        }
        config_data.signers.push(pubkey);
        if config_data.signers.len() > 4 {
            return Err(ClaimError::ExceedsLimit.into());
        }
    }

    Ok(())
}

fn param_error(filename: &'static str, line: u32) -> Error {
    Error::from(anchor_lang::error::AnchorError {
        error_name: ClaimError::InvalidParameter.name(),
        error_code_number: ClaimError::InvalidParameter.into(),
        error_msg: ClaimError::InvalidParameter.to_string(),
        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
            anchor_lang::error::Source { filename, line },
        )),
        compared_values: None,
    })
}
