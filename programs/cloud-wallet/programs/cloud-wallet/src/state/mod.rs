pub mod big_nonce_data;
pub mod config_data;
pub mod empty_data;
pub mod nonce_data;
pub mod nonce_increase_data;

pub use big_nonce_data::*;
pub use config_data::*;
pub use empty_data::*;
pub use nonce_data::*;
pub use nonce_increase_data::*;

pub const EMPTY_DATA_SPACE: usize = 1;
pub const NONCE_INCREASE_DATA_SPACE: usize = 9;
pub const CONFIG_DATA_SPACE_V4: usize = 2835;
pub const BIG_NONCE_DATA_SPACE: usize = 10240;
pub const BIG_NONCE_BUCKET: u64 = 1146752;
