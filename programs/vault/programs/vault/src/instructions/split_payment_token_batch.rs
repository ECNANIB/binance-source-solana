use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, CreateAccount, Transfer};
use anchor_lang::AccountDeserialize;
use anchor_spl::associated_token::{self, AssociatedToken, Create};
use anchor_spl::token_interface::{self, TokenAccount, TransferChecked};









#[event]
pub struct SplitPaymentTokenBatchEvent {
    pub nonces: Vec<u64>,
    pub amounts: Vec<u64>,
    pub fees: Vec<u64>,
    pub mints: Vec<Pubkey>,
    pub platform: Pubkey,
    pub split_contract: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SplitPaymentTokenBatchArgs {
    pub nonces: Vec<u64>,
    pub amounts: Vec<u64>,
    pub fees: Vec<u64>,
    pub decimals: Vec<u8>,
}

#[event_cpi]
#[derive(Accounts)]
#[instruction(args: SplitPaymentTokenBatchArgs)]
pub struct SplitPaymentTokenBatch<'info> {
    #[account(
        mut,
        constraint = config_data.addrs.contains(signer.key) @ crate::VaultError::NotAllowed
    )]
    pub signer: Signer<'info>,

    pub token_holder: UncheckedAccount<'info>,
    #[account(
        seeds = [crate::CONFIG_DATA_SEED],
        bump = config_data.data_bump,
        constraint = !config_data.paused @ crate::VaultError::Paused
    )]
    pub config_data: Account<'info, ConfigData>,

    #[account(mut, constraint = config_data.platform == *platform.key @ crate::VaultError::NotAllowed)]
    pub platform: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = config_data.split_contract == *split_contract.key @ crate::VaultError::NotAllowed
    )]
    pub split_contract: UncheckedAccount<'info>,

    pub associated_token_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}











pub(crate) fn split_payment_token_batch_handler<'info>(
    ctx: Context<'_, '_, 'info, 'info, SplitPaymentTokenBatch<'info>>,
    args: SplitPaymentTokenBatchArgs,
) -> Result<()> { fn split_payment_token_batch_inner<'info>(ctx: Context<'_, '_, 'info, 'info, SplitPaymentTokenBatch<'info>>, args: &SplitPaymentTokenBatchArgs) -> Result<()> {
    let n = ctx.remaining_accounts.len();
    (n != 0 && n % 6 == 0).then_some(()).ok_or_else(|| anchor_lang::error!(crate::VaultError::InvalidParameter))?;
    let holder_seeds: &[&[&[u8]]] =

        &[&[crate::TOKEN_HOLDER_SEED, &[ctx.accounts.config_data.token_holder_bump]]];
    let mut mints: Vec<Pubkey> = Vec::with_capacity(n / 6);





    let mut k = 0usize;
    let mut item = 1usize;
    while k < n {
        let i = item.wrapping_sub(1); item = item.wrapping_add(1);
        mark_nonce(&ctx, unsafe { ctx.remaining_accounts.get_unchecked(k) }, &args.nonces, i)?;






        let src_ta =                     ctx.remaining_accounts[k.wrapping_add(1)].clone();

        let tokprog =       ctx.remaining_accounts[k.wrapping_add(2)].clone();

        let dst_ta =                 ctx.remaining_accounts[k.wrapping_add(3)].clone();

        let mint =       ctx.remaining_accounts[k.wrapping_add(4)].clone();

        let fee_ta =                       ctx.remaining_accounts[k.wrapping_add(5)].clone();
        mints.push(*mint.key);
        if mint.key == ctx.accounts.system_program.key {
            let before = ctx.accounts.token_holder.lamports();

            let cpi_a = CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.token_holder.to_account_info(),
                    to: ctx.accounts.platform.to_account_info(),
                },
                holder_seeds,
            );
            system_program::transfer(cpi_a,
                                           args.amounts[i],
            )?;
            let cpi_b = CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.token_holder.to_account_info(),
                    to: ctx.accounts.split_contract.to_account_info(),
                },
                holder_seeds,
            );
            system_program::transfer(cpi_b,         args.fees[i],
            )?;
            let after = ctx.accounts.token_holder.lamports();

            fn e137() -> Error { anchor_lang::error!(crate::VaultError::InvalidParameter) } if !(after ==

                       before - args.amounts[i] - args.fees[i]) { return Err(e137()); }
        } else {
            let before = token_balance(&src_ta)?;
            let token_program = tokprog.clone();
            ensure_token_account(&ctx.accounts, &dst_ta, &ctx.accounts.platform, &mint, &tokprog)?;












            let cpi_accounts = TransferChecked {
                from: src_ta.clone(),
                to: dst_ta,
                authority: ctx.accounts.token_holder.to_account_info(),
                mint: mint.clone(),
            }; let cpi_c = CpiContext::new_with_signer(tokprog.clone(), cpi_accounts, holder_seeds);




            token_interface::transfer_checked(cpi_c,
                             args.amounts[i],
                          args.decimals[i])?;











            ensure_token_account(&ctx.accounts, &fee_ta, &ctx.accounts.split_contract, &mint, &tokprog)?;
            let cpi_d = CpiContext::new_with_signer(
                token_program,
                TransferChecked { from: src_ta.clone(), to: fee_ta, authority: ctx.accounts.token_holder.to_account_info(), mint: mint },
                holder_seeds,
            );


            token_interface::transfer_checked(cpi_d,
                                      args.fees[i], args.decimals[i])?;





            let after = token_balance(&src_ta)?; fn e195() -> Error { anchor_lang::error!(crate::VaultError::InvalidParameter) } if !(after ==

                       before - args.amounts[i] - args.fees[i]) { return Err(e195()); }






        }

        k = k.wrapping_add(6);
    }
    emit_cpi!(SplitPaymentTokenBatchEvent { nonces: { let mut v = Vec::new(); let ne = !args.nonces.is_empty(); if ne { v = args.nonces.clone(); } v }, amounts: { let mut v = Vec::new(); let ne = !args.amounts.is_empty(); if ne { v = args.amounts.clone(); } v }, fees: { let mut v = Vec::new(); let ne = !args.fees.is_empty(); if ne { v = args.fees.clone(); } v }, mints, platform: ctx.accounts.config_data.platform, split_contract: ctx.accounts.config_data.split_contract }); Ok(())
    } split_payment_token_batch_inner(ctx, &args)
}





fn mark_nonce<'info>(
    ctx: &Context<'_, '_, 'info, 'info, SplitPaymentTokenBatch<'info>>,

    acc: &AccountInfo<'info>,
    nonces: &Vec<u64>,
    i: usize,
) -> Result<()> { let marker_acc = acc.clone();
    require!(marker_acc.data_is_empty(), crate::VaultError::InvalidParameter);
    let nonce =                            nonces[i];
    require!(nonce != 0, crate::VaultError::InvalidParameter);
    let s: &[&[u8]] = &[crate::SPLIT_PAYMENT_ID_SEED, &nonce.to_le_bytes()];
    let (pda, bump) = Pubkey::find_program_address(s, &crate::ID);
    require!(&pda == marker_acc.key, crate::VaultError::InvalidParameter);

    let min = Rent::get()?.minimum_balance(1);
    let have = marker_acc.lamports();
    let need = if min > have {

        min - marker_acc.lamports()
    } else { 0 };



    let cpi_e = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        CreateAccount {
            from: ctx.accounts.signer.to_account_info(),
            to: marker_acc.clone(),
        },
    );
    let sseeds: &[&[&[u8]]] = &[&[crate::SPLIT_PAYMENT_ID_SEED, &nonce.to_le_bytes(), &[bump]]]; let cpi_e = CpiContext { signer_seeds: sseeds, ..cpi_e };
    system_program::create_account(cpi_e, need, 1, &crate::ID)?;
    marker_acc.resize(1)?;


    marker_acc.data.borrow_mut()[0] = 5u8;
    Ok(())
}

fn ensure_token_account<'info>(
    accounts: &SplitPaymentTokenBatch<'info>,
    ata: &AccountInfo<'info>,
    authority: &UncheckedAccount<'info>,
    mint: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
) -> Result<()> {
    if ata.try_borrow_mut_data()?.len() < 1 {
        let cpi_f = CpiContext::new(
            accounts.associated_token_program.to_account_info(),
            Create {
                payer: accounts.signer.to_account_info(),
                associated_token: ata.clone(),
                authority: authority.to_account_info(),
                mint: mint.clone(),
                system_program: accounts.system_program.to_account_info(),
                token_program: token_program.clone(),
            },
        );
        return associated_token::create_idempotent(cpi_f);

    }
    let data = ata.try_borrow_mut_data()?;
    let acct = TokenAccount::try_deserialize_unchecked(&mut &data[..])?;

    if acct.mint != *mint.key || acct.owner != *authority.key { Err(crate::VaultError::InvalidParameter.into()) } else { Ok(()) }

}

fn token_balance(info: &AccountInfo) -> Result<u64> {
    let data = info.try_borrow_mut_data()?;
    let acct = TokenAccount::try_deserialize_unchecked(&mut &data[..])?;
    Ok(acct.amount)
}
