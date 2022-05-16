use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};
use arrayref::{array_refs};
use std::io::Write;

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    assert!(accounts.len() == 1);
    let account_data = &accounts[0];
    let borrow_data = &mut *account_data.try_borrow_mut_data()?;

    let ( offset_u64, data) = array_refs![instruction_data, 8; ..;];
    let offset = u64::from_le_bytes(*offset_u64) as usize;
    Ok((&mut borrow_data[offset..]).write_all(&data[..])?)
}