use anchor_lang::prelude::*;

use crate::state::ConfigData;
use crate::ErrorCode;





macro_rules! keys_eq_else { ($a:expr, $b:expr, $e:expr $(,)?) => { if !($a == $b) { return Err(anchor_lang::error!($e).with_pubkeys(($a, $b))); } }; }

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CloseNonceAccountParams {
    pub nonces: Vec<u64>,
    pub nonce_data_bump: Vec<u8>,
}


#[derive(Accounts)]
pub struct CloseNonceAcco001<'info> {
    #[account(mut, constraint = (config_data.addresses.contains(signer.key) || config_data.addresses_1.contains(signer.key)) @ ErrorCode::SenderNotAllowed)]
    pub signer: Signer<'info>,
    #[account(seeds = [b"config_data"], bump = config_data.bump_b2)]
    pub config_data: Box<Account<'info, ConfigData>>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn handler<'info>(
    ctx: Context<'_, '_, '_, 'info, CloseNonceAcco001<'info>>,
    params: &CloseNonceAccountParams,

) -> Result<()> {
    let nonces = &params.nonces; let nonce_data_bump = &params.nonce_data_bump; let remaining_accounts = &ctx.remaining_accounts;
    let count = nonces.len();

    require_eq!(count, nonce_data_bump.len(), ErrorCode::NumberMismatch);




    require_eq!(count, ctx.remaining_accounts.len(), ErrorCode::NumberMismatch);





    for (i, nonce) in nonces.iter().enumerate() {

        keys_eq_else!(
            Pubkey::create_program_address(
                &[
                    b"nonce_data",
                    &nonce.to_le_bytes(),
                    &[nonce_data_bump[i]],
                ],
                &crate::ID,
            )
            .map_err(|_| ErrorCode::WrongNonceTable)?,
            remaining_accounts[i].key(),
            ErrorCode::WrongNonceTable
        );
        let account = remaining_accounts[i].to_account_info();
        let credit = ctx.accounts.signer.lamports()
            .checked_add(account.lamports())
            .ok_or(ErrorCode::MathOverflow)?;
        **ctx.accounts.signer.lamports.borrow_mut() = credit;


        **account.lamports.borrow_mut() = 0;

    }
    Ok(())
}
