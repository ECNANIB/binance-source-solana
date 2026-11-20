use anchor_lang::prelude::*;

use anchor_spl::associated_token::{self, AssociatedToken};
use crate::ErrorCode;










#[derive(Accounts)]
pub struct BatchCreateTokenAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub(crate) fn handler<'info>(ctx: Context<'_, '_, '_, 'info, BatchCreateTokenAccounts<'info>>) -> Result<()> {
    let accs = ctx.remaining_accounts;
    let n = accs.len() / 4;
    for i in 0..n {
        let token_program = &accs[i * 4];
        let payer = &accs[i * 4 + 1];
        let owner_acct = &accs[i * 4 + 2];
        let ata = &accs[i * 4 + 3];
        if ata.lamports() != 0 {
            continue;
        }



























            let ix = associated_token::spl_associated_token_account::instruction::create_associated_token_account_idempotent(
                ctx.accounts.payer.key,
                payer.key,
                owner_acct.key,
                token_program.key,
            );
            anchor_lang::solana_program::program::invoke(
                &ix,
                &[
                    ctx.accounts.payer.to_account_info(),
                    ata.clone(),
                    payer.clone(),
                    owner_acct.clone(),
                    ctx.accounts.system_program.to_account_info(),
                    token_program.clone(),
                ],
            )?;

    }
    Ok(())
}




