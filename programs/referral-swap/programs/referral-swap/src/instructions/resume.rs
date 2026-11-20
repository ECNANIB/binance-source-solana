use anchor_lang::prelude::*;

use crate::instructions::pause::Pa021;

pub(crate) fn handler(ctx: Context<Pa021>) -> Result<()> {
    ctx.accounts.config_data.pau021 = false;
    Ok(())
}
