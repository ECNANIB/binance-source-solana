use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token_interface::{transfer_checked, TokenInterface, TransferChecked};






#[event_cpi]
#[derive(Accounts)]
pub struct SwapWithPositiveSlippageV2<'info> {

    #[account(constraint = crate::is_allowed_router(router.key) @ crate::RouterError::InvalidRouter)]
    pub router: UncheckedAccount<'info>,

    pub user: Signer<'info>,


    #[account(mut)]
    pub user_from_token_account: UncheckedAccount<'info>,

    pub from_token_mint: UncheckedAccount<'info>,
    pub from_token_program: Interface<'info, TokenInterface>,







    #[account(mut)]
    pub user_to_token_account: UncheckedAccount<'info>,

    pub to_token_mint: UncheckedAccount<'info>,
    pub to_token_program: Interface<'info, TokenInterface>,

    #[account(mut)]
    pub fee_token_account: UncheckedAccount<'info>,

    #[account(mut)]
    pub ps_token_account: UncheckedAccount<'info>,

    pub commission_token_account: UncheckedAccount<'info>,
}





































































fn psv2_require_flag(info: &AccountInfo) -> Result<()> {
    require!(info.is_writable, crate::RouterError::InvalidFeeTokenAccount);
    Ok(())
}





fn fee_token_account_psv2(info: &AccountInfo) -> Result<(Pubkey, Pubkey, u64)> {
    require!(
        *info.owner == anchor_spl::token::ID || *info.owner == anchor_spl::token_2022::ID,
        crate::RouterError::InvalidFeeTokenAccount
    );
    let data = info.try_borrow_data()?;
    let a = anchor_spl::token_interface::TokenAccount::try_deserialize(&mut &data[..])
        .map_err(|_| error!(crate::RouterError::InvalidFeeTokenAccount))?;
    Ok((a.mint, a.owner, a.amount))
}

fn token_amount(account: &AccountInfo) -> Result<u8> {
    if account.owner != &anchor_spl::token::spl_token::ID && account.owner != &anchor_spl::token_2022::spl_token_2022::ID {


        return Err(error!(crate::RouterError::InvalidFeeTokenAccount));
    }
    let data = account.try_borrow_data()?;



    if !(data.len() > 44) {



        return Err(error!(crate::RouterError::InvalidFeeTokenAccount));
    }


    Ok(data[44])
}









fn psv2_split_transfer<'info>(
    token_program: &Interface<'info, TokenInterface>, authority: &AccountInfo<'info>,
    from: &AccountInfo<'info>, to: &AccountInfo<'info>, fee_to: Option<&AccountInfo<'info>>,
    mint: &AccountInfo<'info>, decimals: u8, total: u64, fee: u64,
) -> Result<()> {
        if total == 0 {
            return Ok(());
}
        let net_amount = total - fee;
        let cpi1;
        if net_amount != 0 {
            cpi1 = CpiContext::new(
                token_program.to_account_info(),
                TransferChecked {
                    authority: authority.clone(),
                    from: from.clone(),
                    to: to.clone(),
                    mint: mint.clone(),
                },
            );
            transfer_checked(cpi1, net_amount, decimals)?;
        }
        if let Some(fee_to) = fee_to {
            if fee != 0 {
                transfer_checked(
                    CpiContext::new(
                        token_program.to_account_info(),
                        TransferChecked {
                            authority: authority.clone(),
                            from: from.clone(),
                            to: fee_to.clone(),
                            mint: mint.clone(),
                        },
                    ),
                    fee,
                    decimals,
                )?;
            }
        }
        Ok(())
}

pub fn swap_with_positive_slippage_v2<'info>(
    ctx: Context<'_, '_, '_, 'info, SwapWithPositiveSlippageV2<'info>>,
    original_amount: u64, commission_rate: u32, fee_amount: u64,
    slippage_bps: u16, data: &[u8],
) -> Result<()> {


    let accts = &ctx.accounts;

    require!(commission_rate < 100_000_000, crate::RouterError::InvalidFee);
    require!(original_amount != 0, crate::RouterError::InvalidOriginalAmt);
    require!((slippage_bps & 0x3fff) < 10_000u16, crate::RouterError::InvalidCommissionRate);
    let bps: u16 = slippage_bps & 0x3fff;
    let flag14: bool = (slippage_bps & 0x4000) != 0;
    let from_mint = accts.from_token_mint.key();
    let from_decimals = token_amount(&accts.from_token_mint)?;
    let to_mint = accts.to_token_mint.key();
    let to_decimals = token_amount(&accts.to_token_mint)?;









    require_keys_eq!(*accts.from_token_mint.owner, *accts.from_token_program.key, crate::RouterError::InvalidFeeTokenAccount);




    require_keys_eq!(*accts.to_token_mint.owner, *accts.to_token_program.key, crate::RouterError::InvalidFeeTokenAccount);

    psv2_require_flag(&accts.user_from_token_account)?;
    psv2_require_flag(&accts.user_to_token_account)?;
    let (m1, o1, before_from) = fee_token_account_psv2(&accts.user_from_token_account)?;






    require_keys_eq!(m1, from_mint, crate::RouterError::InvalidFeeTokenAccount);
    require_keys_eq!(o1, accts.user.key(), crate::RouterError::InvalidFeeTokenAccount);
    let (m2, o2, before_to) = fee_token_account_psv2(&accts.user_to_token_account)?;





    require_keys_eq!(m2, to_mint, crate::RouterError::InvalidFeeTokenAccount);
    require_keys_eq!(o2, accts.user.key(), crate::RouterError::InvalidFeeTokenAccount);
    let expected: Pubkey = if flag14 { from_mint } else { to_mint };
    psv2_require_flag(&accts.fee_token_account)?;
    let (m3, o3, _a3) = fee_token_account_psv2(&accts.fee_token_account)?;











    require_keys_eq!(m3, expected, crate::RouterError::InvalidFeeTokenAccount);
    psv2_require_flag(&accts.ps_token_account)?;
    let (m4, o4, _a4) = fee_token_account_psv2(&accts.ps_token_account)?;



    require_keys_eq!(m4, to_mint, crate::RouterError::InvalidFeeTokenAccount);
    crate::validate_fee_receiver(&o4)?;
    let notake = (commission_rate == 0) | (bps == 0);
    let has_commission = *accts.commission_token_account.key != anchor_lang::system_program::ID;
    let commission_key: Pubkey = if !notake {










        require!(has_commission, crate::RouterError::InvalidCommissionAccount);



        require!(accts.commission_token_account.is_writable, crate::RouterError::InvalidCommissionAccount);
        let (m5, o5, _a5) = fee_token_account_psv2(&accts.commission_token_account).map_err(|_| {



            error!(crate::RouterError::InvalidCommissionAccount) })?;
        require_keys_eq!(m5, expected, crate::RouterError::InvalidCommissionAccount);
        o5
    } else {
        require!(!has_commission, crate::RouterError::InvalidCommissionAccount);
        Pubkey::default()
    };
    let mut fee1: u64 = 0;
    let mut c1: u64 = 0;
    if commission_rate != 0 && flag14 {
        let q1: u128 = (commission_rate as u128) * (original_amount as u128) / 100_000_000u128;
        fee1 = q1 as u64;
        if bps != 0 && fee1 != 0 { c1 = (q1 * (bps as u128) / 10_000u128) as u64; }
        let authority_info = accts.user.to_account_info();
        psv2_split_transfer(
            &accts.from_token_program, &authority_info,
            &*accts.user_from_token_account, &*accts.fee_token_account,
            if c1 > 0 { Some(&*accts.commission_token_account) } else { None },
            &*accts.from_token_mint, from_decimals, fee1, c1,
        )?;
    }

    super::route(accts.router.key, ctx.remaining_accounts, 0, ctx.remaining_accounts.len(), data)?;
    let (_, _, after_from) = fee_token_account_psv2(&accts.user_from_token_account)?;






























    require_gte!(before_from, after_from, crate::RouterError::InvalidOriginalAmt);
    let spent = before_from - after_from;
    require_gt!(spent, 0u64, crate::RouterError::InvalidOriginalAmt);
    require_gte!(original_amount, spent, crate::RouterError::InvalidOriginalAmt);
    let (_, _, after_to) = fee_token_account_psv2(&accts.user_to_token_account)?;


    require_gte!(after_to, before_to, crate::RouterError::InvalidOriginalAmt);
    let received = after_to - before_to; let mut out: u64 = received;
    let mut c2: u64 = 0;
    let mut fee2: u64 = 0;
    if commission_rate != 0 && !flag14 {
        let q2: u128 = (received as u128) * (commission_rate as u128) / 100_000_000u128;
        fee2 = q2 as u64;
        if bps != 0 && fee2 != 0 { c2 = (q2 * (bps as u128) / 10_000u128) as u64; }

        psv2_split_transfer(
            &accts.to_token_program, &accts.user.to_account_info(),
            &*accts.user_to_token_account, &*accts.fee_token_account,
            if c2 > 0 { Some(&*accts.commission_token_account) } else { None },
            &*accts.to_token_mint, to_decimals, fee2, c2,
        )?;













        let o = received.checked_sub(fee2).unwrap();
        out = o;
    }


    let mut ps_amount: u64 = 0;
    if out > fee_amount {
        let cpi = CpiContext::new(
            accts.to_token_program.to_account_info(),
            TransferChecked {
                authority: accts.user.to_account_info(),
                from: accts.user_to_token_account.to_account_info(),
                to: accts.ps_token_account.to_account_info(),
                mint: accts.to_token_mint.to_account_info(),
            },
        );
        ps_amount = out - fee_amount;
        transfer_checked(cpi, ps_amount, to_decimals)?;
    }















    let residual: u64 = fee1 - c1;
    let netrem: u64 = fee2 - c2;
    let n1 = super::net_of_transfer_fee(&accts.from_token_mint, residual)?;
    let n2 = super::net_of_transfer_fee(&accts.from_token_mint, c1)?;
    let n3 = super::net_of_transfer_fee(&accts.to_token_mint, netrem)?;
    let n4 = super::net_of_transfer_fee(&accts.to_token_mint, c2)?;
    let n5 = super::net_of_transfer_fee(&accts.to_token_mint, ps_amount)?;




    emit_cpi!(super::PositiveSlippageV2Log {
        user: accts.user.key(),
        router: accts.router.key(),
        from_token_mint: Some(from_mint),
        order_id: original_amount,
        to_token_mint: Some(to_mint),
        user_from_token_account: o3,
        fee_token_account: Some(expected),
        original_amount: n1 + n2,
      received_amount: n3 + n4,
        fee_amount,
        user_to_token_account: o4,
        positive_slippage: out,
        balance_before: n5,
        commission_token_account: commission_key,
        commission:        n2 + n4,
    });
    Ok(())
}
