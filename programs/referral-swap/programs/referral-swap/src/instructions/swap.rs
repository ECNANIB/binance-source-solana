use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::program_pack::Pack;
use anchor_spl::token_2022::spl_token_2022;

use crate::error::ReferralSwapError;
use crate::state::ConfigData;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SwapPar021 {
    pub amount_in: u64,
    pub min_amount_out: u64,
    pub data: Vec<u8>,
}

#[derive(Clone, Copy)]
pub struct TokenInfo {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}
#[event]
pub struct SwapEvent {
    pub from_mint: Pubkey,
    pub amount_in: u64,
    pub to_mint: Pubkey,
    pub amount_out: u64,
}

#[event_cpi]
#[derive(Accounts)]
pub struct S021<'info> {
    #[account(mut,
        constraint = config_data.send021.contains(signer.key) @ ReferralSwapError::NotAllowed)]
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"config_data"],
        bump = config_data.bump,
        constraint = !config_data.pau021 @ ReferralSwapError::Paused
    )]
    pub config_data: Box<Account<'info, ConfigData>>,

    #[account(
        mut,
        seeds = [b"token_holder"],
        bump = config_data.token_holder_b021
    )]
    pub token_holder: UncheckedAccount<'info>,

    pub from_token_account: UncheckedAccount<'info>,

    pub to_token_account: UncheckedAccount<'info>,

    pub router_program: UncheckedAccount<'info>,
}

fn read_token_account(data: &[u8]) -> Result<TokenInfo> {
    let account = spl_token_2022::state::Account::unpack(data)
        .map_err(|_| ReferralSwapError::InvalidParameter)?;
    Ok(TokenInfo {
        owner: account.owner,
        mint: account.mint,
        amount: account.amount,
    })
}






pub(crate) fn handler<'info>(
    mut ctx: Context<'_, '_, '_, 'info, S021<'info>>,
    params: SwapPar021,
) -> Result<()> {
    require!(params.amount_in > 0, ReferralSwapError::InvalidParameter);
    require!(params.min_amount_out > 0, ReferralSwapError::InvalidParameter);
    require!(!params.data.is_empty(), ReferralSwapError::InvalidParameter);
    let accounts = &mut ctx.accounts;
    require!(
        accounts.router_program.key() == accounts.config_data.router_program,
        ReferralSwapError::NotAllowed
    );

    let from_is_native = accounts.from_token_account.key() == Pubkey::default();
    let to_is_token = accounts.to_token_account.key() != Pubkey::default();
    if from_is_native {
        require!(to_is_token, ReferralSwapError::InvalidParameter);
    }



    let (from_mint, from_before);
    if from_is_native {
        from_before = accounts.token_holder.lamports();
        from_mint = Pubkey::default();
    } else {
        let data = accounts.from_token_account.try_borrow_data()?;
        let token = read_token_account(&data)?;
        from_mint = token.mint;
        from_before = token.amount;
    }

    let (to_mint, to_before);
    if to_is_token {
        let data = accounts.to_token_account.try_borrow_data()?;
        let TokenInfo { owner, mint, amount } = read_token_account(&data)?;





        require!(
            owner == accounts.token_holder.key(),
            ReferralSwapError::InvalidParameter
        );
        to_mint = mint; to_before = amount;
    } else {
        to_before = accounts.token_holder.lamports(); to_mint = Pubkey::default();
    }

    require!(from_mint != to_mint, ReferralSwapError::InvalidParameter);


    let mut account_metas = Vec::with_capacity(ctx.remaining_accounts.len());
    for info in ctx.remaining_accounts.iter() {
        account_metas.push(AccountMeta {
            pubkey: *info.key,
            is_signer: *info.key == accounts.token_holder.key() || info.is_signer,
            is_writable: info.is_writable,
        });
    }

    let ix = Instruction {
        program_id: accounts.router_program.key(),
        accounts: account_metas,
        data: params.data.to_vec(),
    };



    let mut account_infos = Vec::with_capacity(ctx.remaining_accounts.len() + 2);
    account_infos.push(accounts.router_program.to_account_info());
    account_infos.push(accounts.token_holder.to_account_info());
    for info in ctx.remaining_accounts.iter() {
        account_infos.push(info.clone());
    }

    let seeds: &[&[&[u8]]] =
        &[&[b"token_holder", &[accounts.config_data.token_holder_b021]]];
    invoke_signed(&ix, &account_infos, seeds)?;

    let from_after = if from_is_native {
        accounts.token_holder.lamports()
    } else {
        let data = accounts.from_token_account.try_borrow_data()?;
        read_token_account(&data)?.amount
    };

    let to_after = if to_is_token {
        let data = accounts.to_token_account.try_borrow_data()?;
        read_token_account(&data)?.amount
    } else {
        accounts.token_holder.lamports()
    };








    let spent = from_before
        .checked_sub(from_after)
        .ok_or(ReferralSwapError::FromTokenOverConsumed)?;
    require!(spent <= params.amount_in, ReferralSwapError::FromTokenOverConsumed);

    let received = to_after
        .checked_sub(to_before)
        .ok_or(ReferralSwapError::InsufficientOutputAmount)?;

    require!(received >= params.min_amount_out, ReferralSwapError::InsufficientOutputAmount);

    emit_cpi!(SwapEvent {
        from_mint,
        amount_in: spent,
        to_mint,
        amount_out: received,
    });

    Ok(())
}
