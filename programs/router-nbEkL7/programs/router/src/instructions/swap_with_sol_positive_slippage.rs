use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::invoke;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::token_interface::{transfer_checked, TokenInterface, TransferChecked};







#[event_cpi]
#[derive(Accounts)]
pub struct SwapWithSolPositiveSlippage<'info> {

    #[account(constraint = crate::is_allowed_router(router.key) @ crate::RouterError::InvalidRouter)]
    pub router: UncheckedAccount<'info>,
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub fee_receiver: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,


    pub user_to_token_account: UncheckedAccount<'info>,

    pub to_token_mint: UncheckedAccount<'info>,

    pub ps_token_account: UncheckedAccount<'info>,

    pub user_from_token_account: UncheckedAccount<'info>,

    pub from_token_mint: UncheckedAccount<'info>,

    pub ps_sol_receiver: UncheckedAccount<'info>,


    pub fee_token_account: UncheckedAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
}























































fn sol_require_token_flag(info: &AccountInfo) -> Result<()> {
    require!(info.is_writable, crate::RouterError::InvalidFeeTokenAccount);
    Ok(())
}




fn sol_fee_token_account(info: &AccountInfo) -> Result<anchor_spl::token_interface::TokenAccount> {
    require!(
        *info.owner == anchor_spl::token::ID || *info.owner == anchor_spl::token_2022::ID,
        crate::RouterError::InvalidFeeTokenAccount
    );
    let data = info.try_borrow_data()?;
    anchor_spl::token_interface::TokenAccount::try_deserialize(&mut &data[..])
        .map_err(|_| error!(crate::RouterError::InvalidFeeTokenAccount))
}

fn sol_ps_mint(info: &AccountInfo) -> Result<anchor_spl::token_interface::Mint> {
    require!(
        *info.owner == anchor_spl::token::ID || *info.owner == anchor_spl::token_2022::ID,
        crate::RouterError::InvalidFeeTokenAccount
    );
    let data = info.try_borrow_data()?;
    anchor_spl::token_interface::Mint::try_deserialize(&mut &data[..])
        .map_err(|_| error!(crate::RouterError::InvalidFeeTokenAccount))
}
pub fn swap_with_sol_positive_slippage<'info>(
    ctx: Context<'_, '_, '_, 'info, SwapWithSolPositiveSlippage<'info>>,

    original_amount: u64,
    commission_rate: u16,
    wrap_sol: bool,
    fee_amount: u64,
    data: &[u8],
) -> Result<()> {
    require!(original_amount != 0, crate::RouterError::InvalidOriginalAmt);
    require!(commission_rate <= 10_000, crate::RouterError::InvalidFee);


    let a_present = *ctx.accounts.user_to_token_account.key != anchor_lang::system_program::ID;
    let b_present = *ctx.accounts.user_from_token_account.key != anchor_lang::system_program::ID;



    let a_present = match (a_present, b_present) { (true, false) => true, (false, true) => false, _ => return err!(crate::RouterError::UnexpectedAccount) };










    fn sol_optional_fee_token_owner(info: &AccountInfo, required: bool, expected_mint: Pubkey) -> Result<Option<Pubkey>> {
        let present = *info.key != anchor_lang::system_program::ID;
        require!(!present || required, crate::RouterError::UnexpectedAccount);
        require!(present || !required, crate::RouterError::MissingRequiredAccount);
        if !present { return Ok(None); }
        sol_require_token_flag(info)?;
        let acct = sol_fee_token_account(info)?;

        require_keys_eq!(acct.mint, expected_mint, crate::RouterError::InvalidFeeTokenAccount);
        Ok(Some(acct.owner))
    }
















    fn sol_arm_a<'info>(ctx: &Context<'_, '_, '_, 'info, SwapWithSolPositiveSlippage<'info>>, original_amount: u64, commission_rate: u16, wrap_sol: bool, fee_amount: u64, data: &[u8]) -> Result<()> {
        require!(*ctx.accounts.from_token_mint.key == anchor_lang::system_program::ID && *ctx.accounts.ps_sol_receiver.key == anchor_lang::system_program::ID, crate::RouterError::UnexpectedAccount);





        require!(*ctx.accounts.to_token_mint.key != anchor_lang::system_program::ID && *ctx.accounts.ps_token_account.key != anchor_lang::system_program::ID, crate::RouterError::MissingRequiredAccount);
        let mint_a = sol_ps_mint(&*ctx.accounts.to_token_mint)?;
        let dec_a: u8 = mint_a.decimals;
        let need_comm_a = commission_rate != 0 && !wrap_sol;
        let mint_key_a: Pubkey = *ctx.accounts.to_token_mint.key;
        let cm = sol_optional_fee_token_owner(&*ctx.accounts.fee_token_account, need_comm_a, ctx.accounts.to_token_mint.key())?;
        sol_require_token_flag(&ctx.accounts.user_to_token_account)?;
        sol_require_token_flag(&ctx.accounts.ps_token_account)?;
        let acct_a = sol_fee_token_account(&*ctx.accounts.user_to_token_account)?;










        require_keys_eq!(acct_a.mint, mint_key_a, crate::RouterError::InvalidFeeTokenAccount);
        require_keys_eq!(acct_a.owner, ctx.accounts.user.key(), crate::RouterError::InvalidFeeTokenAccount);
        let acct_fee_a = sol_fee_token_account(&*ctx.accounts.ps_token_account)?;





        require_keys_eq!(acct_fee_a.mint, mint_key_a, crate::RouterError::InvalidFeeTokenAccount);
        crate::validate_fee_receiver(&acct_fee_a.owner)?;
        let sol_fee_a: u64 = if commission_rate != 0 && wrap_sol {
            let fee = ((original_amount as u128) * (commission_rate as u128) / 10_000u128) as u64;
            if fee != 0 {
                let cpi_a1 = CpiContext::new(
                        ctx.accounts.system_program.to_account_info(),
                        Transfer {
                            from: ctx.accounts.user.to_account_info(),
                            to: ctx.accounts.fee_receiver.to_account_info(),
                        },
                    );
                transfer(cpi_a1,
                    fee,
                )?;
            }
            fee
        } else {
            0
        };
        let lamports_pre_a = ctx.accounts.user.lamports();
        super::route(ctx.accounts.router.key, ctx.remaining_accounts, 0, ctx.remaining_accounts.len(), data)?;
        let post_a: u64 = sol_fee_token_account(&*ctx.accounts.user_to_token_account)?.amount;
        let pre_a: u64 = acct_a.amount;





































        require_gte!(post_a, pre_a, crate::RouterError::InvalidOriginalAmt);
        let lamports_post_a = ctx.accounts.user.lamports();


        require_gte!(lamports_pre_a, lamports_post_a, crate::RouterError::InvalidOriginalAmt);
        let spent_a: u64 = lamports_pre_a - lamports_post_a;









        require_gt!(spent_a, 0u64, crate::RouterError::InvalidOriginalAmt);


        let budget_a: u64 = original_amount.saturating_add(6_117_840);




        require_gte!(budget_a, spent_a, crate::RouterError::InvalidOriginalAmt);
        let amount_a: u64 = post_a - pre_a;
        let (net_a, fee_a): (u64, u64) = if commission_rate != 0 && !wrap_sol {
            let fee = ((amount_a as u128) * (commission_rate as u128) / 10_000u128) as u64;
            if fee != 0 {
                let cpi_a2 = CpiContext::new(
                        ctx.accounts.token_program.to_account_info(),
                        TransferChecked {
                            authority: ctx.accounts.user.to_account_info(),
                            from: ctx.accounts.user_to_token_account.to_account_info(),
                            to: ctx.accounts.fee_token_account.to_account_info(),
                            mint: ctx.accounts.to_token_mint.to_account_info(),
                        },
                    );
                transfer_checked(cpi_a2,
                    fee,
                    dec_a,
                )?;




                (amount_a.checked_sub(fee).unwrap(), fee)
            } else {
                (amount_a, 0)
            }
        } else {
            (amount_a, 0)
        };
        let ps_a: u64 = if net_a > fee_amount {
            let ps = net_a - fee_amount;
            let cpi_a3 = CpiContext::new(ctx.accounts.token_program.to_account_info(), TransferChecked { authority: ctx.accounts.user.to_account_info(), from: ctx.accounts.user_to_token_account.to_account_info(), to: ctx.accounts.ps_token_account.to_account_info(), mint: ctx.accounts.to_token_mint.to_account_info() });
            transfer_checked(cpi_a3,
                ps,
                dec_a,
            )?;
            ps
        } else {
            0
        };
        let (owner_a, mint_opt_a): (Pubkey, Option<Pubkey>) = if need_comm_a {
            let o = cm.unwrap();
            (o, Some(mint_key_a))
        } else {
            (*ctx.accounts.fee_receiver.key, None)
        };
        let user_to_a: Pubkey = acct_fee_a.owner;
        let net_fee_a = super::net_of_transfer_fee(&ctx.accounts.to_token_mint.to_account_info(), fee_a)?;
        let net_ps_a = super::net_of_transfer_fee(&ctx.accounts.to_token_mint.to_account_info(), ps_a)?;

        emit_cpi!(super::PositiveSlippageLog { user: *ctx.accounts.user.key, router: *ctx.accounts.router.key, from_token_mint: None, order_id: original_amount, to_token_mint: Some(mint_key_a), user_from_token_account: owner_a, fee_token_account: mint_opt_a, original_amount: sol_fee_a, received_amount: net_fee_a, user_to_token_account: user_to_a, fee_amount, positive_slippage: net_a, balance_before: net_ps_a });
        Ok(()) } fn sol_arm_b<'info>(ctx: &Context<'_, '_, '_, 'info, SwapWithSolPositiveSlippage<'info>>, original_amount: u64, commission_rate: u16, wrap_sol: bool, fee_amount: u64, data: &[u8]) -> Result<()> {










































        require!(*ctx.accounts.to_token_mint.key == anchor_lang::system_program::ID && *ctx.accounts.ps_token_account.key == anchor_lang::system_program::ID, crate::RouterError::UnexpectedAccount);





        require!(*ctx.accounts.from_token_mint.key != anchor_lang::system_program::ID && *ctx.accounts.ps_sol_receiver.key != anchor_lang::system_program::ID, crate::RouterError::MissingRequiredAccount);
        let mint_b = sol_ps_mint(&*ctx.accounts.from_token_mint)?;
        let dec_b: u8 = mint_b.decimals;
        let mint_key_b: Pubkey = *ctx.accounts.from_token_mint.key;
        let recv_key_b: Pubkey = *ctx.accounts.ps_sol_receiver.key;
        crate::validate_fee_receiver(&recv_key_b)?;
        let need_comm_b = commission_rate != 0 && wrap_sol;
        let cm = sol_optional_fee_token_owner(&*ctx.accounts.fee_token_account, need_comm_b, mint_key_b)?;
        sol_require_token_flag(&ctx.accounts.user_from_token_account)?;
        sol_require_token_flag(&ctx.accounts.ps_sol_receiver)?;
        let acct_b = sol_fee_token_account(&*ctx.accounts.user_from_token_account)?;















        require_keys_eq!(acct_b.mint, mint_key_b, crate::RouterError::InvalidFeeTokenAccount);




        require_keys_eq!(acct_b.owner, ctx.accounts.user.key(), crate::RouterError::InvalidFeeTokenAccount);
        let token_fee_b: u64 = if commission_rate != 0 && wrap_sol {
            let fee = ((original_amount as u128) * (commission_rate as u128) / 10_000u128) as u64;
            if fee != 0 {
                let cpi_b1 = CpiContext::new(
                        ctx.accounts.token_program.to_account_info(),
                        TransferChecked {
                            authority: ctx.accounts.user.to_account_info(),
                            from: ctx.accounts.user_from_token_account.to_account_info(),
                            to: ctx.accounts.fee_token_account.to_account_info(),
                            mint: ctx.accounts.from_token_mint.to_account_info(),
                        },
                    );
                transfer_checked(cpi_b1,
                    fee,
                    dec_b,
                )?;
            }
            fee
        } else {
            0
        };
        let lamports_pre_b = ctx.accounts.user.lamports();
        super::route(ctx.accounts.router.key, ctx.remaining_accounts, 0, ctx.remaining_accounts.len(), data)?;
        let post_b: u64 = sol_fee_token_account(&*ctx.accounts.user_from_token_account)?.amount;
        let pre_b: u64 = acct_b.amount;











        require_gte!(pre_b, post_b, crate::RouterError::InvalidOriginalAmt);
        let spent_b: u64 = pre_b - post_b;







        require_gt!(spent_b, 0u64, crate::RouterError::InvalidOriginalAmt);
        require_gte!(original_amount, spent_b, crate::RouterError::InvalidOriginalAmt);
        let lamports_post_b = ctx.accounts.user.lamports();

        require_gte!(lamports_post_b, lamports_pre_b, crate::RouterError::InvalidOriginalAmt);
        let lamports_delta_b: u64 = lamports_post_b - lamports_pre_b;
        let (net_b, sol_fee_b): (u64, u64) = {
            let sol_fee = if commission_rate != 0 && !wrap_sol {
                ((lamports_delta_b as u128) * (commission_rate as u128) / 10_000u128) as u64
            } else {
                0
            };
            if sol_fee != 0 {
                let cpi_b2 = CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.user.to_account_info(),
                        to: ctx.accounts.fee_receiver.to_account_info(),
                    },
                );
                transfer(cpi_b2, sol_fee)?;
            }







            (lamports_delta_b.checked_sub(sol_fee).unwrap(), sol_fee)
        };
        let ps_b: u64 = if net_b > fee_amount {
            let ps = net_b - fee_amount;
            let cpi_b3 = CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.user.to_account_info(),
                    to: ctx.accounts.ps_sol_receiver.to_account_info(),
                },
            );
            transfer(cpi_b3, ps)?;
            ps
        } else {
            0
        };
        let (owner_b, mint_opt_b): (Pubkey, Option<Pubkey>) = if need_comm_b {
            let o = cm.unwrap();
            (o, Some(mint_key_b))
        } else {
            (*ctx.accounts.fee_receiver.key, None)
        };
        let net_fee_b = super::net_of_transfer_fee(&ctx.accounts.from_token_mint.to_account_info(), token_fee_b)?;
        emit_cpi!(super::PositiveSlippageLog { user: *ctx.accounts.user.key, router: *ctx.accounts.router.key, from_token_mint: Some(mint_key_b), order_id: original_amount, to_token_mint: None, user_from_token_account: owner_b, fee_token_account: mint_opt_b, original_amount: net_fee_b, received_amount: sol_fee_b, user_to_token_account: recv_key_b, fee_amount, positive_slippage: net_b, balance_before: ps_b });
        Ok(()) } if a_present { sol_arm_a(&ctx, original_amount, commission_rate, wrap_sol, fee_amount, data) } else { sol_arm_b(&ctx, original_amount, commission_rate, wrap_sol, fee_amount, data) }
}
