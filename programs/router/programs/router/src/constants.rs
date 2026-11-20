use anchor_lang::prelude::*;
use std::collections::HashMap;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::invoke;














pub fn is_allowed_router(key: &Pubkey) -> bool {
    pubkey!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4") == *key
        || pubkey!("HuTkmnrv4zPnArMqpbMbFhfwzTR7xfWQZHH1aQKzDKFZ") == *key
        || pubkey!("Hph8HdfcfgEKVjc3UaVLkUZFG74r3u624VsZ8rg2aUSu") == *key
        || pubkey!("DF1ow4tspfHX9JwWJsAb9epbkA8hmpSEAtxXy1V27QBH") == *key
        || pubkey!("T1TANpTeScyeqVzzgNViGDNrkQ6qHz9KrSBS4aNXvGT") == *key
        || pubkey!("FDF8AxHB8UK7RS6xay6aBvwS3h7kez9gozqz14JyfKsg") == *key
}


















pub(crate) fn route<'info>(
    router_key: &Pubkey,
    remaining_accounts: &[AccountInfo<'info>],
    n: u8,
    data: &[u8],
) -> Result<()> {
    let mut metas: Vec<AccountMeta> = Vec::with_capacity(n as usize);
    let mut map: HashMap<&'info Pubkey, AccountInfo<'info>> = HashMap::new();
    for i in 0..n {
        let info = remaining_accounts.get(i as usize).ok_or(crate::RouterError::AddrNotExists)?;
        metas.push(AccountMeta {
            pubkey: *info.key,
            is_signer: info.is_signer,
            is_writable: info.is_writable,
        });
        if map.contains_key(&info.key) {
            continue;
        }
        map.insert(info.key, info.clone());
    }
    let infos: Vec<AccountInfo> = map.values().cloned().collect();
    let ix = Instruction {
        program_id: *router_key,
        accounts: metas,
        data: data.to_vec(),
    };
    invoke(&ix, &infos)?;
    Ok(())
}

pub fn validate_fee_receiver(key: &Pubkey) -> Result<()> {
    if pubkey!("6KFVj2czLwNQeE4yBgFau6LHUS72vqgP11Sv1JUNxghz") == *key
        || pubkey!("GoUzotbKE6kcr63UQJNxL7KZE58ycbyGdRd3KTRURPqP") == *key
        || pubkey!("A9SgkL5i5d5uAHvukDpUmapAoNvnQnw2SZUC1jf7xRC5") == *key
        || pubkey!("8Xn4YW9vTrW3MXkLdT2vbxd3ykcpMQK5iy9RuD146H6y") == *key
    {
        Ok(())
    } else {
        Err(crate::error::RouterError::InvalidFeeReceiver.into())
    }
}
