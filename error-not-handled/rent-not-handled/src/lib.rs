use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

// Define the User struct
#[derive(Debug, BorshSerialize, BorshDeserialize)]
struct User {
    key: Pubkey,
    name: String,
}

// Solana program entrypoint
entrypoint!(process_instruction);
#[allow(unused_variables)]
// Process instruction function
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Create User");

    // Verify accounts
    let account_info_iter = &mut accounts.iter();
    let user_account = next_account_info(account_info_iter)?;

    // Check account size
    let rent = &Rent::get().unwrap(); // Error not handled!
    let min_space = std::mem::size_of::<User>();
    if user_account.data_len() < min_space {
        return Err(ProgramError::InvalidAccountData);
    }

    // Deserialize account data into User struct
    let name = String::from_utf8(instruction_data.to_vec())
    .expect("Bytes were not valid UTF-8");
    let new_data = User {
        key: *user_account.key,
        name: name
    };

    new_data.serialize(&mut user_account.try_borrow_mut_data()?.as_mut())?;

    Ok(())
}