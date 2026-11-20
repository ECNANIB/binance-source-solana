use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::program::invoke_signed;
use std::collections::HashMap;
pub mod buy_to_alpha; pub mod close_wsol; pub mod config_addr; pub mod emergency_withdraw; pub mod increase_config_data_space; pub mod initialize; pub mod pause; pub mod rebalance_out; pub mod rebalance_out_sol; pub mod remove_trade_token; pub mod resume; pub mod sell_to_self_from_alpha; pub mod set_partially_filled_remaining_pct_base; pub mod set_trade_token; pub mod sync_native; pub mod transfer_ownership;
use crate::errors::MemeshotVaultError;


pub const WSOL_MINT: Pubkey = Pubkey::new_from_array([
    0x06, 0x9b, 0x88, 0x57, 0xfe, 0xab, 0x81, 0x84, 0xfb, 0x68, 0x7f, 0x63, 0x46, 0x18, 0xc0, 0x35,
    0xda, 0xc4, 0x39, 0xdc, 0x1a, 0xeb, 0x3b, 0x55, 0x98, 0xa0, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x01,
]);
pub const WSOL_MINT_ALT: Pubkey = Pubkey::new_from_array([
    0x83, 0x0d, 0xfc, 0x9f, 0xde, 0x5f, 0xe6, 0xb8, 0xaa, 0x7c, 0x04, 0xa4, 0x76, 0xe9, 0x1e, 0x8a,
    0xc6, 0xbb, 0x26, 0x4a, 0xad, 0x90, 0xfa, 0x19, 0xc9, 0xdf, 0x49, 0xd8, 0x5c, 0x3e, 0x5b, 0x5e,
]);


pub const TOKEN_PROGRAM: Pubkey = Pubkey::new_from_array([
    0x06, 0xdd, 0xf6, 0xe1, 0xd7, 0x65, 0xa1, 0x93, 0xd9, 0xcb, 0xe1, 0x46, 0xce, 0xeb, 0x79, 0xac,
    0x1c, 0xb4, 0x85, 0xed, 0x5f, 0x5b, 0x37, 0x91, 0x3a, 0x8c, 0xf5, 0x85, 0x7e, 0xff, 0x00, 0xa9,
]);
pub const TOKEN_2022_PROGRAM: Pubkey = Pubkey::new_from_array([
    0x06, 0xdd, 0xf6, 0xe1, 0xee, 0x75, 0x8f, 0xde, 0x18, 0x42, 0x5d, 0xbc, 0xe4, 0x6c, 0xcd, 0xda,
    0xb6, 0x1a, 0xfc, 0x4d, 0x83, 0xb9, 0x0d, 0x27, 0xfe, 0xbd, 0xf9, 0x28, 0xd8, 0xa1, 0x8b, 0xfc,
]);


pub const AGG_ROUTER_A: Pubkey = Pubkey::new_from_array([
    0xfb, 0x2b, 0x71, 0xcc, 0x3a, 0x4d, 0x02, 0x93, 0x20, 0x96, 0xd5, 0x89, 0x77, 0xb2, 0xf8, 0xb3,
    0x9c, 0x6e, 0x49, 0x3f, 0xbd, 0x46, 0x7c, 0xd9, 0xaa, 0x62, 0x1b, 0x3e, 0x5a, 0xd1, 0xd1, 0xc4,
]);
pub const AGG_ROUTER_B: Pubkey = Pubkey::new_from_array([
    0x04, 0x79, 0xd5, 0x5b, 0xf2, 0x31, 0xc0, 0x6e, 0xee, 0x74, 0xc5, 0x6e, 0xce, 0x68, 0x15, 0x07,
    0xfd, 0xb1, 0xb2, 0xde, 0xa3, 0xf4, 0x8e, 0x51, 0x02, 0xb1, 0xcd, 0xa2, 0x56, 0xbc, 0x13, 0x8f,
]);


pub fn require_wsol_mint(key: &Pubkey) -> Result<()> {
    match *key { WSOL_MINT | WSOL_MINT_ALT => Ok(()), _ => Err(MemeshotVaultError::InvalidProgram.into()) }
}

pub fn invoke_route<'info>(program_id: &Pubkey, signer_key: &Pubkey, accounts: &[AccountInfo<'info>], lo: u8, hi: u8, data: &[u8], signer_seeds: &[&[&[u8]]; 1]) -> Result<()> {
    let mut ix_metas: Vec<AccountMeta> = Vec::with_capacity((hi - lo) as usize);
    let mut seen: HashMap<&Pubkey, AccountInfo<'info>> = HashMap::new();
    for i in lo..hi {
        let info = accounts.get(i as usize).ok_or(MemeshotVaultError::AddrNotExists)?;
        ix_metas.push(AccountMeta {
            pubkey: *info.key,
            is_writable: info.is_writable,
            is_signer: info.is_signer || *info.key == *signer_key,
        });
        if seen.contains_key(&info.key) {
            continue;
        }
        seen.insert(info.key, info.clone());
    }





    let account_infos: Vec<AccountInfo<'info>> = seen.values().cloned().collect();
    invoke_signed(&Instruction {
        program_id: *program_id,
        accounts: ix_metas,
        data: data.to_vec(),
    }, &account_infos, signer_seeds)?;
    Ok(())
}
use anchor_spl::token_2022::spl_token_2022::extension::StateWithExtensions;
use anchor_spl::token_2022::spl_token_2022::state::Account as Token2022Account;


pub fn read_token_account(info: &AccountInfo) -> Result<(Pubkey, u64)> {
    let data = info.try_borrow_data()?;
    let state = StateWithExtensions::<Token2022Account>::unpack(&data)?;
    Ok((state.base.mint, state.base.amount))
}


pub fn account_at<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    index: u8,
) -> Result<&'a AccountInfo<'info>> {
    accounts
        .get(index as usize)
        .ok_or(MemeshotVaultError::AddrNotExists.into())
}


pub fn is_registered(addr: &Pubkey, key: &Pubkey) -> bool {
    *addr != Pubkey::default() && addr == key
}







pub fn check_partially_filled(
    filled_amount: u64,
    total_amount: u64,
    pct_base: u64,
) -> Result<()> {
    if !(total_amount == filled_amount
        || (total_amount > filled_amount && total_amount - filled_amount
                      < total_amount / pct_base))
    {

        return Err(error!(MemeshotVaultError::OrderPartiallyFilled)
            .with_values((filled_amount, total_amount)));
    }
    Ok(())
}


pub fn is_agg_router(configured: &Pubkey, router: &Pubkey) -> bool {
    configured == router || router == &AGG_ROUTER_A || router == &AGG_ROUTER_B
}

pub use buy_to_alpha::*;
pub use close_wsol::*;
pub use config_addr::*;
pub use emergency_withdraw::*;
pub use increase_config_data_space::*;
pub use initialize::*;
pub use pause::*;
pub use rebalance_out::*;
pub use rebalance_out_sol::*;
pub use remove_trade_token::*;
pub use resume::*;
pub use sell_to_self_from_alpha::*;
pub use set_partially_filled_remaining_pct_base::*;
pub use set_trade_token::*;
pub use sync_native::*;
pub use transfer_ownership::*;
