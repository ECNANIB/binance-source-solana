use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::invoke;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::token_interface::{transfer_checked, TokenInterface, TransferChecked};



#[event_cpi]
#[derive(Accounts)]
pub struct SwapWithSolPositiveSlippageV2<'info> {

    #[account(constraint = crate::is_allowed_router(router.key) @ crate::RouterError::InvalidRouter)]
    pub router: UncheckedAccount<'info>,
    #[account(mut)]
    pub user: Signer<'info>,









    #[account(mut)]
    pub spl_user_ata: UncheckedAccount<'info>,

    pub spl_mint: UncheckedAccount<'info>,

    #[account(mut)]
    pub ps_receiver: UncheckedAccount<'info>,

    #[account(mut)]
    pub fee_receiver: UncheckedAccount<'info>,


    pub commission_receiver: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}








































































fn v2_require_token_flag(info: &AccountInfo) -> Result<()> {
    require!(info.is_writable, crate::RouterError::InvalidFeeTokenAccount);
    Ok(())
}

fn sol_v2_fee_token_account(info: &AccountInfo) -> Result<anchor_spl::token_interface::TokenAccount> {
    require!(
        *info.owner == anchor_spl::token::ID || *info.owner == anchor_spl::token_2022::ID,
        crate::RouterError::InvalidFeeTokenAccount
    );
    let data = info.try_borrow_data()?;
    anchor_spl::token_interface::TokenAccount::try_deserialize(&mut &data[..])
        .map_err(|_| error!(crate::RouterError::InvalidFeeTokenAccount))
}













fn v2_require_token_program(info: &AccountInfo, err: crate::RouterError) -> Result<()> {
    if *info.owner == anchor_spl::token::ID || *info.owner == anchor_spl::token_2022::ID {
        return Err(error!(err));
    }
    Ok(())
}








fn v2_mint(info: &AccountInfo, program: &Pubkey) -> Result<anchor_spl::token_interface::Mint> {
    require_keys_eq!(*info.owner, *program, crate::RouterError::InvalidFeeTokenAccount);
    let data = info.try_borrow_data()?;
    anchor_spl::token_interface::Mint::try_deserialize(&mut &data[..]).map_err(|_| error!(crate::RouterError::InvalidFeeTokenAccount))
}





pub fn swap_with_sol_positive_slippage_v2<'info>(
    ctx: Context<'_, '_, '_, 'info, SwapWithSolPositiveSlippageV2<'info>>,
    original_amount: u64,
    commission_rate: u32,
    fee_amount: u64,
    slippage_bps: u16,
    data: &[u8]) -> Result<()> {
    require!(original_amount != 0, crate::RouterError::InvalidOriginalAmt);
    require!(commission_rate < 100_000_000, crate::RouterError::InvalidFee);
    require!((slippage_bps & 0x3fff) < 10_000, crate::RouterError::InvalidCommissionRate);
    let bps: u16 = slippage_bps & 0x3fff;
    let flag14: bool = (slippage_bps & 0x4000) != 0;
    v2_swap(ctx, original_amount, commission_rate, fee_amount, data, bps, flag14, (slippage_bps as i16) >= 0)

}








































fn v2_optional_fee_token(info: &AccountInfo, check: bool, mint_only: bool, expected_mint: &Pubkey) -> Result<(Pubkey, Option<Pubkey>)> {
    if !check {
        return Ok((*info.key, None));
}
    v2_require_token_flag(info)?;
    if mint_only {
        require!(*info.key != anchor_lang::system_program::ID, crate::RouterError::InvalidFeeTokenAccount);
        v2_require_token_program(info, crate::RouterError::InvalidFeeTokenAccount)?;
        return Ok((*info.key, None));
    }
    let acct = sol_v2_fee_token_account(info)?;
    require_keys_eq!(acct.mint, *expected_mint, crate::RouterError::InvalidFeeTokenAccount);
    Ok((acct.owner, Some(*expected_mint)))
}


















fn v2_commission(ctx: &Context<'_, '_, '_, '_, SwapWithSolPositiveSlippageV2<'_>>, take: bool, program_only: bool, amount: u64, rate: u16, expected_mint: &Pubkey) -> Result<(Pubkey, u64)> {
    if !take {
        require!(*ctx.accounts.commission_receiver.key == anchor_lang::system_program::ID, crate::RouterError::InvalidCommissionAccount);
        return Ok((Pubkey::default(), 0));
    }
    let commission = if amount != 0 {
        ((amount as u128) * (rate as u128) / 10_000) as u64
    } else {
        0
    };



    require!(ctx.accounts.commission_receiver.key.as_ref() != anchor_lang::system_program::ID.as_ref(), crate::RouterError::InvalidCommissionAccount);
    require!(ctx.accounts.commission_receiver.is_writable, crate::RouterError::InvalidCommissionAccount);
    if program_only {
        v2_require_token_program(&ctx.accounts.commission_receiver, crate::RouterError::InvalidCommissionAccount)?;
        return Ok((*ctx.accounts.commission_receiver.key, commission));
    }
    let acct = sol_v2_fee_token_account(&ctx.accounts.commission_receiver)?;
    require_keys_eq!(acct.mint, *expected_mint, crate::RouterError::InvalidCommissionAccount);
    Ok((acct.owner, commission))
}







pub fn v2_sol_split_transfer<'info>(
    accounts: &SwapWithSolPositiveSlippageV2<'info>,
    to: &AccountInfo<'info>,
    fee_to: Option<&AccountInfo<'info>>,
    total: u64,
    fee: u64,
) -> Result<()> {
    if total != 0 {
        let sol_amount = total - fee;
        let cpi1;
        if sol_amount != 0 {
            cpi1 = CpiContext::new(
                accounts.system_program.to_account_info(),
                Transfer {
                    from: accounts.user.to_account_info(),
                    to: to.clone(),
                },
            );
            transfer(cpi1, sol_amount)?;
        }
        if let Some(fee_to) = fee_to {
            if fee != 0 {
                transfer(
                    CpiContext::new(
                        accounts.system_program.to_account_info(),
                        Transfer {
                            from: accounts.user.to_account_info(),
                            to: fee_to.clone(),
                        },
                    ),
                    fee,
                )?;
            }
        }
    }
    Ok(())
}

pub fn v2_split_transfer<'info>(
    accounts: &SwapWithSolPositiveSlippageV2<'info>,
    from: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    fee_to: Option<&AccountInfo<'info>>,
    mint: &AccountInfo<'info>,
    decimals: u8,
    total: u64,
    fee: u64,
) -> Result<()> {
    if total != 0 {
        let net_amount = total - fee;
        let cpi1;
        if net_amount != 0 {
            cpi1 = CpiContext::new(
                accounts.token_program.to_account_info(),
                TransferChecked {
                    authority: accounts.user.to_account_info(),
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
                        accounts.token_program.to_account_info(),
                        TransferChecked {
                            authority: accounts.user.to_account_info(),
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
    }
    Ok(())
}



fn sol_to_token<'info>(
    ctx: Context<'_, '_, '_, 'info, SwapWithSolPositiveSlippageV2<'info>>,
    original_amount: u64,
    commission_rate: u32,
    fee_amount: u64,
    data: &[u8], bps: u16, flag14: bool,
) -> Result<()> {


        let mint = v2_mint(&*ctx.accounts.spl_mint, ctx.accounts.token_program.key)?;
        let decimals = mint.decimals;
        let mint_key: Pubkey = *ctx.accounts.spl_mint.key;
        let (opt_key, opt_mint) = v2_optional_fee_token(&*ctx.accounts.fee_receiver, commission_rate != 0, flag14, &ctx.accounts.spl_mint.key())?;
        v2_require_token_flag(&ctx.accounts.spl_user_ata)?;
        v2_require_token_flag(&ctx.accounts.ps_receiver)?;
        let ta1 = sol_v2_fee_token_account(&*ctx.accounts.spl_user_ata)?;



        require_keys_eq!(ta1.mint, mint_key, crate::RouterError::InvalidFeeTokenAccount);
        require_keys_eq!(ta1.owner, ctx.accounts.user.key(), crate::RouterError::InvalidFeeTokenAccount);
        let ta2 = sol_v2_fee_token_account(&*ctx.accounts.ps_receiver)?;
        let ps_owner = ta2.owner;



        require_keys_eq!(ta2.mint, mint_key, crate::RouterError::InvalidFeeTokenAccount);
        crate::validate_fee_receiver(&ta2.owner)?;
        let first = commission_rate != 0 && flag14; let take = bps > 0; let mut commission_key = Pubkey::default(); let (take2, fee1, commission1) = if first {
            let fee1 = ((original_amount as u128) * (commission_rate as u128) / 100_000_000u128) as u64;
            let (key, c1) = v2_commission(&ctx, take, true, fee1, bps, &mint_key)?; commission_key = key;
            v2_sol_split_transfer(&*ctx.accounts, &*ctx.accounts.fee_receiver, if c1 > 0 { Some(&*ctx.accounts.commission_receiver) } else { None }, fee1, c1)?;
            (take, fee1, c1)
        } else if commission_rate != 0 && take {
            (true, 0u64, 0u64)
        } else {
            let (key, _) = v2_commission(&ctx, false, flag14, 0, bps, &mint_key)?; commission_key = key;
            (false, 0u64, 0u64)
        };
        let lamports1 = ctx.accounts.user.lamports();
        super::route(ctx.accounts.router.key, ctx.remaining_accounts, 0, ctx.remaining_accounts.len(), data)?;
        let ta3 = sol_v2_fee_token_account(&*ctx.accounts.spl_user_ata)?;







































        require_gte!(ta3.amount, ta1.amount, crate::RouterError::InvalidOriginalAmt);
        let lamports2 = ctx.accounts.user.lamports();


        require_gte!(lamports1, lamports2, crate::RouterError::InvalidOriginalAmt);
        let spent = lamports1 - lamports2;




        require_gt!(spent, 0u64, crate::RouterError::InvalidOriginalAmt);
        let limit = original_amount.saturating_add(6_117_840);
        require_gte!(limit, spent, crate::RouterError::InvalidOriginalAmt);
        let gain = ta3.amount - ta1.amount;
        let (out, c3, fee_c2) = if !flag14 && commission_rate != 0 {
            let fee = ((gain as u128) * (commission_rate as u128) / 100_000_000u128) as u64;
            let (key, c3) = v2_commission(&ctx, take2, false, fee, bps, &mint_key)?; commission_key = key;
            v2_split_transfer(&*ctx.accounts, &*ctx.accounts.spl_user_ata, &*ctx.accounts.fee_receiver, if c3 > 0 { Some(&*ctx.accounts.commission_receiver) } else { None }, &*ctx.accounts.spl_mint, decimals, fee, c3)?;



























            let ps = gain.checked_sub(fee).unwrap();
            (ps, c3, fee)
        } else {
            (gain, 0u64, 0u64)
        };
        let mut moved: u64 = 0;
        if out > fee_amount {
            let amount = out - fee_amount;
            let cpi_a = CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    TransferChecked {
                        authority: ctx.accounts.user.to_account_info(),
                        from: ctx.accounts.spl_user_ata.to_account_info(),
                        to: ctx.accounts.ps_receiver.to_account_info(),
                        mint: ctx.accounts.spl_mint.to_account_info(),
                    },
            );
            transfer_checked(cpi_a,
                amount,
                decimals,
            )?;
            moved = amount;
        }



        let fee_net = fee_c2 - c3;
        let net_a = super::net_of_transfer_fee(&*ctx.accounts.spl_mint, fee_net)?;
        let net_b = super::net_of_transfer_fee(&*ctx.accounts.spl_mint, c3)?;
        let net_c = super::net_of_transfer_fee(&*ctx.accounts.spl_mint, moved)?;
        emit_cpi!(super::PositiveSlippageV2Log {
            user: *ctx.accounts.user.key, router: *ctx.accounts.router.key,
            from_token_mint: None, order_id: original_amount,
            to_token_mint: Some(mint_key), user_from_token_account: opt_key, fee_token_account: opt_mint,
            original_amount: fee1,







      received_amount: net_a + net_b,
            user_to_token_account: ps_owner,
            fee_amount,
            positive_slippage: out,
            balance_before: net_c,
            commission_token_account: commission_key,

               commission: commission1 + net_b,
        });






    Ok(()) } fn v2_swap<'info>(ctx: Context<'_, '_, '_, 'info, SwapWithSolPositiveSlippageV2<'info>>, original_amount: u64, commission_rate: u32, fee_amount: u64, data: &[u8], bps: u16, flag14: bool, to_sol: bool) -> Result<()> {
        if to_sol {
        let mint = v2_mint(&*ctx.accounts.spl_mint, ctx.accounts.token_program.key)?;
        let decimals = mint.decimals;
        let mint_key: Pubkey = *ctx.accounts.spl_mint.key;
        let ps_key: Pubkey = *ctx.accounts.ps_receiver.key;
        crate::validate_fee_receiver(&ps_key)?;
        v2_require_token_flag(&ctx.accounts.ps_receiver)?;
        let (opt_key, opt_mint) = v2_optional_fee_token(&*ctx.accounts.fee_receiver, commission_rate != 0, !flag14, &mint_key)?;
        v2_require_token_flag(&ctx.accounts.spl_user_ata)?;
        let ta1 = sol_v2_fee_token_account(&*ctx.accounts.spl_user_ata)?;



















        require_keys_eq!(ta1.mint, mint_key, crate::RouterError::InvalidFeeTokenAccount);
        require_keys_eq!(ta1.owner, ctx.accounts.user.key(), crate::RouterError::InvalidFeeTokenAccount);
        let first = commission_rate != 0 && flag14; let take = bps != 0; let mut commission_key = Pubkey::default(); let (take2, fee1, commission1) = if first {
            let fee1 = ((original_amount as u128) * (commission_rate as u128) / 100_000_000u128) as u64;
            let (key, c1) = v2_commission(&ctx, take, false, fee1, bps, &mint_key)?; commission_key = key;
            v2_split_transfer(&*ctx.accounts, &*ctx.accounts.spl_user_ata, &*ctx.accounts.fee_receiver, if c1 > 0 { Some(&*ctx.accounts.commission_receiver) } else { None }, &*ctx.accounts.spl_mint, decimals, fee1, c1)?;
            (take, fee1, c1)
        } else if commission_rate != 0 && take {
            (true, 0u64, 0u64)
        } else {
            let (key, _) = v2_commission(&ctx, false, !flag14, 0, bps, &mint_key)?; commission_key = key;
            (false, 0u64, 0u64)
        };
        let lamports1 = ctx.accounts.user.lamports();
        super::route(ctx.accounts.router.key, ctx.remaining_accounts, 0, ctx.remaining_accounts.len(), data)?;
        let ta2 = sol_v2_fee_token_account(&*ctx.accounts.spl_user_ata)?;









































        require_gte!(ta1.amount, ta2.amount, crate::RouterError::InvalidOriginalAmt);
        let spent = ta1.amount - ta2.amount;
        require_gt!(spent, 0u64, crate::RouterError::InvalidOriginalAmt);
        require_gte!(original_amount, spent, crate::RouterError::InvalidOriginalAmt);
        let lamports2 = ctx.accounts.user.lamports();

        require_gte!(lamports2, lamports1, crate::RouterError::InvalidOriginalAmt);
        let gain = lamports2 - lamports1;
        let (net_gain, c3, fee_calc2) = if !flag14 && commission_rate != 0 {
            let fee_calc2 = ((gain as u128) * (commission_rate as u128) / 100_000_000u128) as u64;
            let (key, c3) = v2_commission(&ctx, take2, true, fee_calc2, bps, &mint_key)?; commission_key = key;
            v2_sol_split_transfer(&*ctx.accounts, &*ctx.accounts.fee_receiver, if c3 > 0 { Some(&*ctx.accounts.commission_receiver) } else { None }, fee_calc2, c3)?;





























            let slip = gain.checked_sub(fee_calc2).unwrap();
            (slip, c3, fee_calc2)
        } else {
            (gain, 0u64, 0u64)
        };
        let mut moved: u64 = 0;
        if net_gain > fee_amount {
            let amount = net_gain - fee_amount;
            let cpi_b = CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.user.to_account_info(),
                        to: ctx.accounts.ps_receiver.to_account_info(),
                    },
            );
            transfer(cpi_b,
                amount,
            )?;
            moved = amount;
        }
        let fee_net_a = fee1 - commission1;
        let net1 = super::net_of_transfer_fee(&*ctx.accounts.spl_mint, fee_net_a)?;
        let net2 = super::net_of_transfer_fee(&*ctx.accounts.spl_mint, commission1)?;
        emit_cpi!(super::PositiveSlippageV2Log {
            user: *ctx.accounts.user.key, router: *ctx.accounts.router.key,
            from_token_mint: Some(mint_key), order_id: original_amount,
            to_token_mint: None, user_from_token_account: opt_key, fee_token_account: opt_mint,







        original_amount: net1 + net2,
            received_amount: fee_calc2,
            user_to_token_account: ps_key,
            fee_amount,
            positive_slippage: net_gain,
            balance_before: moved,
            commission_token_account: commission_key,
               commission: net2 + c3,
        });

    Ok(()) } else { sol_to_token(ctx, original_amount, commission_rate, fee_amount, data, bps, flag14) }
}
