pub mod close_nonce_account;
pub mod close_wsol;
pub mod config_addr;
pub mod emergency_withdraw;
pub mod initialize;
pub mod rebalance_out;
pub mod remove_trade_token;
pub mod resume;
pub mod set_partially_filled_remaining_pct_base;
pub mod set_secp256k1_pubkey;
pub mod set_trade_token;
pub mod swap_to_sol;
pub mod swap_to_sol_v2;
pub mod swap_to_token;
pub mod swap_to_token_v2;
pub mod sync_native;
pub mod upgrade_to_v2;
pub mod batch_create_token_accounts;

pub use close_nonce_account::*; pub use close_wsol::*; pub use config_addr::*; pub use emergency_withdraw::*; pub use initialize::*;
pub use rebalance_out::*; pub use remove_trade_token::*; pub use resume::*; pub use set_partially_filled_remaining_pct_base::*;
pub use set_secp256k1_pubkey::*; pub use set_trade_token::*; pub use swap_to_sol::*; pub use swap_to_sol_v2::*; pub use swap_to_token::*;
pub use swap_to_token_v2::*; pub use sync_native::*; pub use upgrade_to_v2::*; pub use batch_create_token_accounts::*;

use anchor_lang::prelude::*;

use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::invoke_signed;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{self, TokenAccount, TokenInterface, TransferChecked};
use crate::state::{ConfigData, RebalanceOutEvent, RebalanceOutSolEvent};
use crate::ErrorCode;

pub fn check_mint(mint: &Pubkey) -> Result<()> {
    match *mint {
        WSOL_MINT => Ok(()),
        ALT_MINT => Ok(()),
        _ => Err(ErrorCode::InvalidProgram.into()),
    }
}
pub fn agg_router_swap<'info>(
    agg_router: &Pubkey,
    token_holder: &Pubkey,
    remaining_accounts: &[AccountInfo<'info>],
    account_len: u8,
    data: &[u8],
    signer_seeds: &[&[&[u8]]; 1],
) -> Result<()> {
    let mut metas: Vec<AccountMeta> = Vec::with_capacity(account_len as usize);
    let mut seen: std::collections::HashMap<&'info Pubkey, AccountInfo<'info>> =
        std::collections::HashMap::new();
    for i in 0..account_len {
        let acc = remaining_accounts.get(i as usize).ok_or(ErrorCode::AddrNotExists)?;
        metas.push(AccountMeta {
            pubkey: *acc.key,
            is_signer: acc.is_signer || *acc.key == *token_holder,
            is_writable: acc.is_writable,
        });
        if seen.contains_key(&acc.key) { continue; }
        seen.insert(acc.key, acc.clone());
    }
    let infos: Vec<AccountInfo<'info>> = seen.values().cloned().collect();










    invoke_signed(
        &Instruction {
            program_id: *agg_router,
            accounts: metas,
            data: data.to_vec(),
        },
        &infos,
        signer_seeds,
    )?;
    Ok(())
}
















pub fn is_rebalance_to(key: &Pubkey, expected: &Pubkey) -> bool {
    *key != Pubkey::default() && *key == *expected
}
pub fn check_partially_filled(a: u64, b: u64, p: u64) -> Result<()> {
    if b == a { return Ok(()); }
    if b > a && b - a < b / p { return Ok(()); }


    Err(error!(ErrorCode::OrderPartiallyFilled).with_values((a, b)))
}


pub(crate) fn check_nonce<'info>(
    payer: &Signer<'info>,
    nonce_data: &UncheckedAccount<'info>,
    system_program: &Program<'info, System>,
    nonce: u64,
    bump: u8,
) -> Result<()> {
    let data_old = nonce_data.data.borrow();
    if data_old.len() != 0 { return Err(error!(ErrorCode::NonceUsed).with_values((data_old.len() as u64, 0))); }
    drop(data_old);
    let need = Rent::get()?.minimum_balance(1);
    let lamports = if need > nonce_data.lamports() {
        need - nonce_data.lamports()
    } else {
        0
    };
    invoke_signed(
        &anchor_lang::solana_program::system_instruction::create_account(payer.key, nonce_data.key, lamports, 1u64, &crate::ID),
        &[payer.to_account_info(), nonce_data.to_account_info(), system_program.to_account_info()],
        &[&[b"nonce_data", &nonce.to_le_bytes(), &[bump]]],
    )?;
    nonce_data.realloc(1usize, false)?;










    nonce_data.data.borrow_mut()[0] = 5;
    Ok(())
}





pub fn token_account_amount(info: &AccountInfo) -> Result<u64> {
    let data = info.try_borrow_data()?;
    let account = TokenAccount::try_deserialize(&mut &data[..])?;
    Ok(account.amount)
}








































































pub const WSOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
pub const ALT_MINT: Pubkey = pubkey!("9pan9bMn5HatX4EJdBwg9VgCa7Uz5HL8N1m5D3NdXejP");















pub mod config_read;








pub const AGG_ROUTER_ALT: Pubkey = pubkey!("HuTkmnrv4zPnArMqpbMbFhfwzTR7xfWQZHH1aQKzDKFZ");
pub const AGG_ROUTER_JUPITER: Pubkey = pubkey!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");
pub fn is_agg_router(expected: &Pubkey, key: &Pubkey) -> bool {
    expected == key || *key == AGG_ROUTER_ALT || *key == AGG_ROUTER_JUPITER
}








pub const TOKEN_2022_PROGRAM: Pubkey = pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");
pub const TOKEN_PROGRAM: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
