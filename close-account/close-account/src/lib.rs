use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account_to_close = next_account_info(accounts_iter)?;
    let destination = next_account_info(accounts_iter)?;

    let dest_starting_lamports = destination.lamports();
    let lamports_to_transfer = account_to_close.lamports();

    **destination.lamports.borrow_mut() = dest_starting_lamports
        .checked_add(lamports_to_transfer)
        .ok_or(ProgramError::InvalidArgument)?;

    **account_to_close.lamports.borrow_mut() = 0;

    Ok(())
}