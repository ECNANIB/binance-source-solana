use anchor_lang::prelude::*;

#[account(discriminator = 5)]
#[derive(Default)]
pub struct EmptyData {}
