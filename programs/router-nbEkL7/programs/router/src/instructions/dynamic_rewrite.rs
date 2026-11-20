use anchor_lang::prelude::*;

use crate::RouterError;









pub const PROXY_SWAP_V1: [u8; 8] = 0xc88775e1919ec6f8u64.to_le_bytes();
pub const PROXY_SWAP_V3: [u8; 8] = 0xee2c384894822c13u64.to_le_bytes();

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ProxyRo0j6 {
    pub de0j6: Vec<u8>,
    pub weig0j6: Vec<u16>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ProxySwapArgs {
    pub amount_in: u64,
    pub expect_amount_out: u64,
    pub min_return: u64,
    pub amounts: Vec<u64>,
    pub routes: Vec<Vec<ProxyRo0j6>>,
    pub commission_rate: u16,
    pub commission_direction: bool,
}




























pub fn dynamic_rewrite(data: &[u8], amount_in: u64, min_out: u64) -> Result<(Vec<u8>, u64)> {



    require!(data.len() >= 8, RouterError::DynamicRewriteFailed);

    let disc = &data[..8];
    if disc != &PROXY_SWAP_V1[..] && disc != &PROXY_SWAP_V3[..] {
        return Err(error!(RouterError::UnsupportedProxyInstruction));
    }


    let mut cursor: &[u8] = &data[8..];
    let mut args = ProxySwapArgs::deserialize(&mut cursor)



        .map_err(|_| error!(RouterError::DynamicRewriteFailed))?;
    let rest = cursor;
    let min_return = rescale(&mut args, amount_in, min_out)?;


    let mut out = Vec::with_capacity(data.len());
    out.extend_from_slice(disc);
    args
        .serialize(&mut out)
        .map_err(|_| error!(RouterError::DynamicRewriteFailed))?;
    out.extend_from_slice(rest);

    Ok((out, min_return))
}


fn rescale(args: &mut ProxySwapArgs, amount_in: u64, min_out: u64) -> Result<u64> {
    if args.amounts.is_empty() {







        return Err(error!(RouterError::DynamicRewriteFailed));
    }

    let swap_amount =
        amount_after_commission(amount_in, args.commission_rate, args.commission_direction)?;

    let total = args.amounts

        .iter()
        .try_fold(0u64, |acc, amount| acc.checked_add(*amount))


        .ok_or(error!(RouterError::MathOverflow))?;
    if total == 0 { return Err(error!(RouterError::DynamicRewriteFailed)); }

    let old_amount_in = args.amount_in;
    args.amount_in = amount_in;

    let min_return = if min_out == u64::MAX {














        require!(old_amount_in != 0, RouterError::DynamicRewriteFailed);
        let scaled = (swap_amount as u128)
            .checked_mul(args.min_return as u128)
            .ok_or(error!(RouterError::MathOverflow))?
            .checked_div(old_amount_in as u128)
            .ok_or(error!(RouterError::DynamicRewriteFailed))?;






        u64::try_from(scaled).map_err(|_| error!(RouterError::MathOverflow))?
    } else {
        min_out
    };
    args.min_return = min_return;

    let last = args.amounts.len() - 1;
    let mut allocated = 0u64;
    for (i, amount) in args.amounts.iter_mut().enumerate() {
        if i == last {
            *amount = swap_amount



                .checked_sub(allocated).ok_or(error!(RouterError::MathOverflow))?;
        } else {
            let share = (swap_amount as u128)
                .checked_mul(*amount as u128)
                .ok_or(error!(RouterError::MathOverflow))?
                .checked_div(total as u128)
                .ok_or(error!(RouterError::MathOverflow))? as u64;
            *amount = share;
            allocated = allocated
                .checked_add(share)
                .ok_or(error!(RouterError::MathOverflow))?;
        }
    }

    Ok(min_return)
}


fn amount_after_commission(amount: u64, rate: u16, direction: bool) -> Result<u64> {





    require!(rate < 10_000, RouterError::DynamicRewriteFailed);
    let net = if rate == 0 || !direction {
        amount
    } else {
        let commission = (rate as u128)
            .checked_mul(amount as u128)






            .ok_or(error!(RouterError::MathOverflow))?
            .checked_div(10_000).ok_or_else(|| error!(RouterError::MathOverflow))?;

        let net = amount
            .checked_sub(commission as u64)

            .ok_or(error!(RouterError::MathOverflow))?;
        if net == 0 { return Err(error!(RouterError::DynamicRewriteFailed)); }
        net
    };
    Ok(net)
}
