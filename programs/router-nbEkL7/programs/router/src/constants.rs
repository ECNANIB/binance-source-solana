use anchor_lang::prelude::*;




pub const ALLOWED_ROUTERS: [Pubkey; 6] = [
    pubkey!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"),
    pubkey!("HuTkmnrv4zPnArMqpbMbFhfwzTR7xfWQZHH1aQKzDKFZ"),
    pubkey!("Hph8HdfcfgEKVjc3UaVLkUZFG74r3u624VsZ8rg2aUSu"),
    pubkey!("DF1ow4tspfHX9JwWJsAb9epbkA8hmpSEAtxXy1V27QBH"),
    pubkey!("T1TANpTeScyeqVzzgNViGDNrkQ6qHz9KrSBS4aNXvGT"),
    pubkey!("FDF8AxHB8UK7RS6xay6aBvwS3h7kez9gozqz14JyfKsg"),
];















pub fn validate_fee_receiver(key: &Pubkey) -> Result<()> {
    if pubkey!("Bj9RUfyBsGqP9RnR7VivwncrpJKv1qTqi2oVQFdaqX1w") == *key {
        Ok(())
    } else {
        Err(crate::error::RouterError::InvalidFeeReceiver.into())
    }
}
