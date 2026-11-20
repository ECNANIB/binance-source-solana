



















use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::invoke;
use anchor_spl::{token::spl_token, token_2022::spl_token_2022};
use spl_token_2022::extension::transfer_fee::TransferFeeConfig;
use spl_token_2022::extension::{BaseStateWithExtensions, StateWithExtensions};
use std::collections::HashMap;

pub mod batch_create_token_accounts;
pub mod close_account;
pub mod delta_window;
pub mod dynamic_rewrite;
pub mod lamport_delta_window;
#[event]
pub struct SwapWithFeeLogEvent {
    pub mint: Option<Pubkey>,
    pub receiver: Pubkey,
    pub amount: u64,
}
pub mod swap_with_dynamic_input;
pub mod swap_with_fee_log;
pub mod swap_with_positive_slippage;
pub mod swap_with_positive_slippage_v2;
pub mod swap_with_sol_fee_log;
pub mod swap_with_sol_positive_slippage;
pub mod swap_with_sol_positive_slippage_v2;
pub mod sync_native;
pub mod take_fee_log;
pub mod take_sol_fee_log;
pub mod token_delta_window;
pub use batch_create_token_accounts::*;
pub use close_account::*;
pub use delta_window::*;
pub use dynamic_rewrite::*;
pub use lamport_delta_window::*;
pub use swap_with_dynamic_input::*;
pub use swap_with_fee_log::*;
pub use swap_with_positive_slippage::*;
pub use swap_with_positive_slippage_v2::*;
pub use swap_with_sol_fee_log::*;
pub use swap_with_sol_positive_slippage::*;
pub use swap_with_sol_positive_slippage_v2::*;
pub use take_fee_log::*;
pub use take_sol_fee_log::*;
pub use token_delta_window::*;
pub use sync_native::*;

pub fn is_allowed_router(key: &Pubkey) -> bool {
    pubkey!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4") == *key
        || pubkey!("HuTkmnrv4zPnArMqpbMbFhfwzTR7xfWQZHH1aQKzDKFZ") == *key
        || pubkey!("Hph8HdfcfgEKVjc3UaVLkUZFG74r3u624VsZ8rg2aUSu") == *key
        || pubkey!("DF1ow4tspfHX9JwWJsAb9epbkA8hmpSEAtxXy1V27QBH") == *key
        || pubkey!("T1TANpTeScyeqVzzgNViGDNrkQ6qHz9KrSBS4aNXvGT") == *key
        || pubkey!("FDF8AxHB8UK7RS6xay6aBvwS3h7kez9gozqz14JyfKsg") == *key
}

























#[event]
pub struct SwapExecutedEvent {
    pub user: Pubkey,
    pub router: Pubkey,
    pub from_token_mint: Option<Pubkey>,
    pub order_id: u64,
    pub to_token_mint: Option<Pubkey>,
    pub user_from_token_account: Pubkey,
    pub fee_token_account: Option<Pubkey>,
    pub original_amount: u64,
    pub received_amount: u64,
    pub user_to_token_account: Pubkey,
    pub fee_amount: u64,
    pub positive_slippage: u64,
    pub balance_before: u64,
}
pub type PositiveSlippageLog = SwapExecutedEvent;
pub type PositiveSlippageV2Log = SwapExecutedV2Event;
































#[event]
pub struct SwapExecutedV2Event {
    pub user: Pubkey,
    pub router: Pubkey,
    pub from_token_mint: Option<Pubkey>,
    pub order_id: u64,
    pub to_token_mint: Option<Pubkey>,
    pub user_from_token_account: Pubkey,
    pub fee_token_account: Option<Pubkey>,
    pub original_amount: u64,
    pub received_amount: u64,
    pub user_to_token_account: Pubkey,
    pub fee_amount: u64,
    pub positive_slippage: u64,
    pub balance_before: u64,
    pub commission_token_account: Pubkey,
    pub commission: u64,
}


























































pub fn route<'info>(
    router_key: &Pubkey,
    accounts: &[AccountInfo<'info>],
    start: usize,
    end: usize,
    data: &[u8],
) -> Result<()> {
    require_gte!(end, start, crate::RouterError::AddrNotExists);

    let mut metas: Vec<AccountMeta> = Vec::with_capacity(end - start);
    let mut map: HashMap<Pubkey, AccountInfo<'info>> = HashMap::with_capacity(end - start);
    for i in start..end {
        let info = accounts.get(i).ok_or(crate::RouterError::AddrNotExists)?;



        metas.push(AccountMeta {
            pubkey: *info.key,
            is_signer: info.is_signer,
            is_writable: info.is_writable,
        });
        map.entry(*info.key).or_insert_with(|| info.clone());


    }
    let infos: Vec<AccountInfo> = map.into_values().collect();
    let ix = Instruction {
        program_id: *router_key,
        accounts: metas,
        data: data.to_vec(),
    };
    invoke(&ix, &infos)?;
    Ok(())
}



































































































fn addr_not_exists_error() -> anchor_lang::error::Error {
    anchor_lang::error::Error::from(anchor_lang::error::AnchorError { error_name: crate::RouterError::AddrNotExists.name(), error_code_number: crate::RouterError::AddrNotExists.into(), error_msg: crate::RouterError::AddrNotExists.to_string(), error_origin: None, compared_values: None })
}

























pub(crate) fn net_of_transfer_fee(
    mint: &AccountInfo,

    amount: u64,
) -> Result<u64> {
    fn transfer_fee(mint: &AccountInfo, amount: u64) -> Result<u64> {
        let token_program = mint.owner;
    if *token_program == spl_token::ID || amount == 0 {
        return Ok(0);
}
    let data = mint.try_borrow_data()?;
    let state = StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&data).map_err(|_| error!(crate::RouterError::InvalidFeeTokenAccount))?;













    let fee = match state.get_extension::<TransferFeeConfig>() {
        Ok(cfg) => cfg.calculate_epoch_fee(Clock::get()?.epoch, amount).unwrap_or(0),
        Err(_) => 0u64,
    };
    Ok(fee) } let fee = transfer_fee(mint, amount)?;
    Ok(amount - fee)
}
