use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};



#[derive(Accounts)]
pub struct SyncNative<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub token_account: AccountInfo<'info>,
    pub token_program: Interface<'info, anchor_spl::token_interface::TokenInterface>,
    pub system_program: Program<'info, System>,
}



pub(crate) fn sync_native<'info>(
    ctx: Context<'_, '_, '_, 'info, SyncNative<'info>>,
    amount: u64,
) -> Result<()> {
    let token_account_acc = &ctx.accounts.token_account; if amount > 0 {
        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: token_account_acc.to_account_info(),
                },
            ),
            amount,
        )?;
    }
    let account = token_account_acc.to_account_info();
    anchor_spl::token_2022::sync_native(CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        anchor_spl::token_2022::SyncNative { account },
    ))?;
    Ok(())
}
