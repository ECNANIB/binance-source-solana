use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::associated_token::spl_associated_token_account::instruction::create_associated_token_account_idempotent;
use anchor_spl::associated_token::AssociatedToken;









#[derive(Accounts)]
pub struct BatchCreateTokenAccou001<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub(crate) fn batch_create_token_accounts<'info>(
    ctx: Context<'_, '_, '_, 'info, BatchCreateTokenAccou001<'info>>,
) -> Result<()> {
    let n = ctx.remaining_accounts.len() / 4;
    for i in 0..n {
        let token_program = &ctx.remaining_accounts[i * 4];
        let owner = &ctx.remaining_accounts[i * 4 + 1];
        let token_mint = &ctx.remaining_accounts[i * 4 + 2];
        let ata = &ctx.remaining_accounts[i * 4 + 3];
        if ata.lamports() == 0 {
            let ix = create_associated_token_account_idempotent(
                ctx.accounts.payer.key,
                owner.key,
                token_mint.key,
                token_program.key,
            );
            invoke(
                &ix,
                &[
                    ctx.accounts.payer.to_account_info(),
                    ata.clone(),
                    owner.clone(),
                    token_mint.clone(),
                    ctx.accounts.system_program.to_account_info(),
                    token_program.clone(),
                ],
            )?;
        }
    }
    Ok(())
}
