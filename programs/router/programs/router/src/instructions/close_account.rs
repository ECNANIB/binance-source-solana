use anchor_lang::prelude::*;

use anchor_spl::token_interface::{close_account, CloseAccount};

#[derive(Accounts)]
pub struct BatchCloseTokenAccounts<'info> {

    #[account(mut)]
    pub destination: AccountInfo<'info>,
    pub owner: Signer<'info>,
}











pub(crate) fn batch_close_token_accounts<'info>(
    ctx: Context<'_, '_, '_, 'info, BatchCloseTokenAccounts<'info>>,
) -> Result<()> {
    let ra = ctx.remaining_accounts;
    let accounts = &ctx.accounts;
    for i in 0..ra.len() / 2 {
        let token_program = &ra[i * 2];
        let token_account = &ra[i * 2 + 1];
        if token_account.lamports() == 0 {
            continue;
        }
        let cpi_accounts = CloseAccount {
            account: token_account.clone(),
            destination: accounts.destination.to_account_info(),
            authority: accounts.owner.to_account_info(),
        };
        let cpi = CpiContext::new(token_program.clone(), cpi_accounts);
        close_account(cpi)?;
    }
    Ok(())
}
