use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use std::mem::size_of;

// Define a simple serialized struct
#[derive(Debug)]
struct MyStruct {
    amount: u64,
}

impl MyStruct {
    // Deserialize the struct from a byte array
    fn from_bytes(bytes: &[u8]) -> Result<Self, ProgramError> {
        if bytes.len() < size_of::<Self>() {
            return Err(ProgramError::InvalidAccountData);
        }
        let amount = u64::from_le_bytes(bytes[..8].try_into().unwrap());
        Ok(Self { amount })
    }

    // Serialize the struct to a byte array
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = vec![0u8; size_of::<Self>()];
        result[..8].copy_from_slice(&self.amount.to_le_bytes());
        result
    }
}

// The entrypoint function
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Casting/Truncation example");

    // Parse accounts
    let accounts_iter = &mut accounts.iter();
    let my_struct_account = next_account_info(accounts_iter)?;

    if my_struct_account.owner != program_id {
        msg!("Incorrect Program ID");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize the struct from the account data
    let my_struct = MyStruct::from_bytes(&my_struct_account.data.borrow())?;

    // Perform the truncation/casting operation
    let amount1 = 10000000000 as u64; // 4294967295 is u32 max
    msg!("Amount 1: {}", amount1);
    let new_amount = (my_struct.amount as u32) + amount1 as u32;
    msg!("new_amount: {}", new_amount);
    msg!("new_amount as u64: {}", new_amount as u64);

    // Serialize the updated struct to the account data
    let new_my_struct = MyStruct { amount: new_amount as u64 };
    my_struct_account.data.borrow_mut().copy_from_slice(&new_my_struct.to_bytes());

    Ok(())
}
