use anchor_lang::prelude::*;








































#[account]
pub struct DeltaSnapshot {
    pub authority: Pubkey,
    pub watched: Pubkey,
    pub start_bala03j: u64,
    pub bump: u8,
}

pub const TOKEN_DELTA_SEED: &[u8] = b"token-delta";
pub const LAMPORT_DELTA_SEED: &[u8] = b"lamport-delta";






#[event(discriminator = [0x20, 0xb8, 0xac, 0x64, 0x74, 0x61, 0x5c, 0x54])]
pub struct DeltaWindowSettled {
    pub authority: Pubkey,
    pub watched: Pubkey,
    pub mint: Option<Pubkey>,
    pub start_bala03j: u64,
    pub delta: u64,
    pub recipient: Pubkey,
    pub received: u64,
}

pub(crate) fn require_top_level() -> Result<()> {



    require_eq!(
        anchor_lang::solana_program::instruction::get_stack_height(),
        1usize,
        crate::RouterError::NotTopLevelInstruction
    );
    Ok(())
}








pub(crate) fn require_matching_settle(
    ix_sysvar: &AccountInfo,
    settle_discriminator: &[u8],
    expected: &Pubkey,
) -> Result<()> {

    require_keys_eq!(*ix_sysvar.key, anchor_lang::solana_program::sysvar::instructions::ID, crate::RouterError::MalformedInstructionsSysvar);
    let data = ix_sysvar.try_borrow_data()?;
    let total = read_u16(&data, 0)? as usize;
    let current = read_u16(&data, data.len().saturating_sub(2))? as usize;
    let mut i = current + 1;
    let mut p = i * 2;



    while i < total {
        let start = read_u16(&data, p + 2)? as usize;
        let n_accounts = read_u16(&data, start)? as usize;
        let program_id_at = start + 2 + n_accounts * 33;
        if slice_at(&data, program_id_at, 32)? == crate::ID.as_ref() {
            let data_len = read_u16(&data, program_id_at + 32)? as usize;
            let ix_data = slice_at(&data, program_id_at + 34, data_len)?;
            if ix_data.starts_with(settle_discriminator) && n_accounts != 0 {
                if slice_at(&data, start + 2 + 1, 32)? == expected.as_ref() {
                    return Ok(());
                }
            }
        }
        p += 2;
        i += 1;
    }





    Err(error!(crate::RouterError::MissingSettleInstruction))
}

fn slice_at(data: &[u8], at: usize, n: usize) -> Result<&[u8]> {
       data.get(at..at + n)
        .ok_or(error!(crate::RouterError::MalformedInstructionsSysvar)) }


fn read_u16(data: &[u8], at: usize) -> Result<u16> {
    let slice = slice_at(data, at, 2)?;
    Ok(u16::from_le_bytes([slice[0], slice[1]]))
}
