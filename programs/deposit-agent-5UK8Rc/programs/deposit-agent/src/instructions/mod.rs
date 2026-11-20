pub mod collect_to;
pub mod config_addr;
pub mod get_address;
pub mod initialize;
pub mod pause;
pub mod resume;
pub mod set_secp256k1_pubkey;
pub mod sync_native;
pub mod transfer_ownership;

pub use collect_to::*;
pub use config_addr::*;
pub use initialize::*;
pub use pause::*;
pub use resume::*;
pub use set_secp256k1_pubkey::*;
pub use sync_native::*;
pub use transfer_ownership::*;

pub(crate) use collect_to::{
    __client_accounts_collect_to, __cpi_client_accounts_collect_to,
};
pub(crate) use config_addr::{
    __client_accounts_config_addr, __cpi_client_accounts_config_addr,
};
pub(crate) use initialize::{
    __client_accounts_initialize, __cpi_client_accounts_initialize,
};
pub(crate) use set_secp256k1_pubkey::{
    __client_accounts_set_secp256k1_pubkey,
    __cpi_client_accounts_set_secp256k1_pubkey,
};
pub(crate) use sync_native::{
    __client_accounts_sync_native_instruction, __cpi_client_accounts_sync_native_instruction,
};
pub(crate) use pause::{__client_accounts_pause, __cpi_client_accounts_pause};
pub(crate) use resume::{__client_accounts_resume, __cpi_client_accounts_resume};
pub(crate) use transfer_ownership::{
    __client_accounts_transfer_ownership, __cpi_client_accounts_transfer_ownership,
};
