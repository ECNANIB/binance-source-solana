use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::errors::CloudWalletError;
use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut, address = pubkey!("DqSVfVxQN3sJNmecje6e7b7kesDb2VBuBMK6oXdWF2Mo"))]
    pub payer: Signer<'info>,
    #[account(mut, seeds = [b"config_data"], bump)]

    pub config_data: UncheckedAccount<'info>,
    #[account(seeds = [b"token_holder"], bump)]

    pub token_holder: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}













pub(crate) fn initialize(ctx: Context<Initialize>, secp256k1_pubkeys: Vec<[u8; 64]>) -> Result<()> {
    let pk: &[[u8; 64]] = &secp256k1_pubkeys;
    let mut config_data = ctx.accounts.config_data.data.borrow_mut();
    let data = ConfigData {
        owner: ctx.accounts.payer.key(),
        pending_owner: Pubkey::default(),
        token_holder: ctx.accounts.payer.key(),
        bump: ctx.bumps.config_data,
        token_holder_bump: ctx.bumps.token_holder,
        paused: false,
        managers: vec![],
        withdraw_addrs: vec![],
        chain_id: 1,
        nonce: 1000,
        agg_router: Pubkey::default(),
        secp256k1_pubkeys: pk.to_vec(),
        rebalance_addrs: vec![pubkey!("B895BDBuPzPvsqmv82KQTNw4bHh6u7Jeo2xgvswmTH27")],
        signatures: pk.to_vec(),
    };
    data.try_serialize(&mut *config_data)?;
    config_data[..8].copy_from_slice(ConfigData::DISCRIMINATOR);
    Ok(())
}
