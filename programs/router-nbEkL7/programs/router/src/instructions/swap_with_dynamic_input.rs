use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::invoke_signed;






























































#[derive(AnchorDeserialize)]
#[allow(dead_code)]
struct ProxySwapArgs {
    original_amount: u64,
    commission_rate: u16,
    wrap_sol: bool,
    fee_amount: u64,
    data: Vec<u8>,
}









struct ProxyLayout {
    to_is_sol: bool,
    user_to: usize,
    user_from: usize,
    route_start: usize,
    event_authority: usize,
    program: usize,
}

fn proxy_layout(tag: u8) -> Result<ProxyLayout> {
    let (user_from, user_to, event_authority, program, route_start, to_is_sol) = match tag {
        10 | 15 => (2, 4, 9, 10, 11, false),
        11 | 16 => (7, 1, 12, 13, 14, true),









        _ => return Err(error!(crate::RouterError::UnsupportedProxyInstruction)),
    };
    Ok(ProxyLayout { to_is_sol, user_to, user_from, route_start, event_authority, program })
}

















pub(crate) fn swap_with_dynamic_input_inner<'info>(ctx: Context<'_, '_, 'info, 'info, SwapWithDynamicInput>, keep_amount: u64, min_out_amount: u64, data: &[u8]) -> Result<()> {


    let accounts = ctx.remaining_accounts;
    require!(!data.is_empty(), crate::RouterError::ProxyDataTooShort);
    let layout = proxy_layout(data[0])?;
    let args = decode_proxy_args(&data)?;




    require!(accounts.len() > layout.route_start, crate::RouterError::MissingRequiredAccount);







    require!(crate::is_allowed_router(accounts[0].key), crate::RouterError::InvalidRouter);



    require!(accounts[1].is_signer, crate::RouterError::MissingRequiredAccount);





    let from_before = fee_token_account_owned_by(&accounts[layout.user_from], accounts[1].key)?;

    let budget = from_before.checked_sub(keep_amount).ok_or(error!(crate::RouterError::NothingToSwap))?;
    require!(budget > 0, crate::RouterError::NothingToSwap);









    let (new_data, expected_out) = super::dynamic_rewrite::dynamic_rewrite(&args.data, budget, min_out_amount)?;
    let to_acc = &accounts[layout.user_to];
    let to_before_amount: u64;
    let to_before: u64;
    let to_before_mint: Option<Pubkey>;
    if layout.to_is_sol {
        to_before = accounts[1].lamports();
        to_before_mint = None;
        to_before_amount = 0;
    } else {
        let to = fee_token_account(to_acc)?;


        require_keys_eq!(to.owner, *accounts[1].key, crate::RouterError::InvalidFeeTokenAccount);
        to_before = to.amount;
        to_before_mint = Some(to.mint); to_before_amount = to.amount;
    }














    super::route(accounts[0].key, accounts, layout.route_start, accounts.len(), &new_data)?;




    let from_after = fee_token_account_owned_by(&accounts[layout.user_from], accounts[1].key)?;




    require_gte!(from_after, keep_amount, crate::RouterError::DynamicSpendExceeded);




    let spent = from_before.checked_sub(from_after).ok_or(error!(crate::RouterError::InvalidOriginalAmt))?;
    require_gt!(spent, 0u64, crate::RouterError::InvalidOriginalAmt);




    let (positive_slippage, to_token_mint) = match &to_before_mint {
        None => {

            let positive_slippage = positive_slippage_amount(to_before, accounts[1].lamports(), expected_out)?;
            (positive_slippage, None)
        } Some(before_mint) => {
            let to = fee_token_account(to_acc)?;
            require_keys_eq!(to.owner, *accounts[1].key, crate::RouterError::InvalidFeeTokenAccount);
            let positive_slippage = positive_slippage_amount(to_before_amount, to.amount, expected_out)?;
            (positive_slippage, Some(*before_mint))
        } };


















































    let from = fee_token_account(&accounts[layout.user_from])?;

    let order_id = from_before.checked_sub(keep_amount).ok_or(error!(crate::RouterError::NothingToSwap))?;
    emit_positive_slippage_log(
        &accounts[layout.event_authority],
        &accounts[layout.program],
        &super::SwapExecutedEvent {
            user: *accounts[1].key,
            router: *accounts[0].key,
            from_token_mint: Some(from.mint),
            order_id,
            to_token_mint,
            user_from_token_account: Pubkey::default(),
            fee_token_account: None,
            original_amount: 0,
            received_amount: 0,
            user_to_token_account: Pubkey::default(),
            fee_amount: 0,
            positive_slippage,
            balance_before: 0,
        },
    )?;
    Ok(()) }






















































fn positive_slippage_amount(a: u64, b: u64, c: u64) -> Result<u64> {



    require_gte!(b, a, crate::RouterError::InvalidOriginalAmt);


    let diff = b.checked_sub(a).ok_or(error!(crate::RouterError::InvalidOriginalAmt))?;
    require_gte!(diff, c, crate::RouterError::InvalidOriginalAmt);












    require_neq!(diff, 0u64, crate::RouterError::InvalidOriginalAmt);
    Ok(diff)
}























fn decode_proxy_args(data: &[u8]) -> Result<ProxySwapArgs> {
    let need: usize = match data[0] {
        10 | 11 => 1,
        15 | 16 => 9,
        _ => return Err(error!(crate::RouterError::UnsupportedProxyInstruction)),
    };


    let mut tail: &[u8] = data.get(need..).ok_or(error!(crate::RouterError::ProxyDecodeFailed))?;

    let args = ProxySwapArgs::deserialize(&mut tail).map_err(|_| error!(crate::RouterError::ProxyDecodeFailed))?;
    Ok(args)
}


fn fee_token_account_owned_by(
    info: &AccountInfo,
    owner: &Pubkey,
) -> Result<u64> {
    let acct = fee_token_account(info)?;
    require_keys_eq!(acct.owner, *owner, crate::RouterError::InvalidFeeTokenAccount);
    Ok(acct.amount)
}

fn fee_token_account(info: &AccountInfo) -> Result<anchor_spl::token_interface::TokenAccount> {
    require!(
        *info.owner == anchor_spl::token::ID || *info.owner == anchor_spl::token_2022::ID,
        crate::RouterError::InvalidFeeTokenAccount
    );
    let data = info.try_borrow_data()?;
    anchor_spl::token_interface::TokenAccount::try_deserialize(&mut &data[..])
        .map_err(|_| error!(crate::RouterError::InvalidFeeTokenAccount))
}











pub(crate) fn emit_positive_slippage_log<'info>(event_authority: &AccountInfo<'info>, program: &AccountInfo<'info>, event: &super::SwapExecutedEvent) -> Result<()> {
    let (expected, bump) = Pubkey::find_program_address(&[b"__event_authority"], &crate::ID);
    require_keys_eq!(*event_authority.key, expected, crate::RouterError::UnexpectedAccount);
    require_keys_eq!(*program.key, crate::ID, crate::RouterError::UnexpectedAccount);
    let mut ix_data = anchor_lang::event::EVENT_IX_TAG_LE.to_vec();
    ix_data.extend_from_slice(super::SwapExecutedEvent::DISCRIMINATOR);
    event.serialize(&mut ix_data)?;
    let ix = Instruction {
        program_id: crate::ID,
        accounts: vec![AccountMeta::new_readonly(expected, true)],
        data: ix_data,
    };
    invoke_signed(
        &ix,
        &[event_authority.clone(), program.clone()],
        &[&[b"__event_authority", &[bump]]],
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct SwapWithDynamicInput {}
