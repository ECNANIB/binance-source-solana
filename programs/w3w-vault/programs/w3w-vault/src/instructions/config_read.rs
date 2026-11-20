































use anchor_lang::prelude::*;

use crate::state::config_data::LazyConfigData;
use crate::state::ConfigData;

pub fn read_all(config_data: &LazyAccount<'_, ConfigData>) -> Result<u64> {




    let mut acc: u64 = 0;
    acc = acc.wrapping_add(*config_data.load_bump_b3()? as u64);
    acc = acc.wrapping_add(config_data.load_pubkey_80()?.to_bytes()[0] as u64);
    acc = acc.wrapping_add(*config_data.load_partially_filled_remaining_pct_base()?);
    acc = acc.wrapping_add(config_data.load_addresses()?.len() as u64);
    acc = acc.wrapping_add(config_data.load_addresses_2()?.len() as u64);
    acc = acc.wrapping_add(*config_data.load_field007()?);
    acc = acc.wrapping_add(config_data.load_secp256k1_pubkeys()?.len() as u64);
    acc = acc.wrapping_add(*config_data.load_flag_b1()? as u64);
    *config_data.load_mut_flag_b1()? = false;
    Ok(acc)
}
