/*
This Solana Rust raw program is a simple smart contract that 
    allows creating and modifying an account with a single u64 data field.
*/

/*
These are the required dependencies for the smart contract, 
including the Borsh serialization and deserialization library, 
the Solana program library, and the system variable library.
*/
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::create_account,
    sysvar::Sysvar,
};

/*
This defines an enumeration called MyInstruction with two variants, 
Create and Modify, each with different fields. 
*/
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum MyInstruction {
    Create {amount: u64},
    Modify {amount: u64, new_amount: u64},
}

/*
This defines a struct called MyData with a single field amount of type u64. 
*/
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct MyData {
    pub amount: u64,
}

entrypoint!(process_instruction);

/*
This is the function that will be called when the program is invoked. 
It takes three arguments: program_id, which is the program's public key; 
    accounts, which is a slice of AccountInfo structs containing information 
    about the accounts involved in the instruction; and instruction_data, 
    which is a slice of bytes that contains the instruction data.
*/
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    mut instruction_data: &[u8],
) -> ProgramResult {
    /* 
    This deserializes the instruction data into a MyInstruction enum and then matches 
        the variant to either call create_acc or modify_acc function.
   */
    match MyInstruction::deserialize(&mut instruction_data)? {
        MyInstruction::Create { amount } => create_acc(&program_id, &accounts, amount),
        MyInstruction::Modify { amount, new_amount } => modify_acc(&program_id, &accounts, amount, new_amount),
    }
}
/* 
This is a function that creates a new account with the given amount and returns a ProgramResult.
*/
pub fn create_acc(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
/* 
This initializes the accounts_iter variable to an iterator over the accounts slice, 
    and then uses the next_account_info function to get the next three account infos from the iterator: 
    the payer, data_account, and system_program.
*/
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let data_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

/* 
This creates a bump seed for the account address by converting the amount to little-endian bytes 
    and then uses the create_program_address function to generate the expected address of the new account.
*/
    let bump = amount.to_le_bytes();

    let expected_address = Pubkey::create_program_address(
        &[&bump.as_ref()], program_id)?;

/*
This checks if the actual address of the data_account matches the expected address, and if not, returns an InvalidArgument error.
*/
    if expected_address != *data_account.key {
        return Err(ProgramError::InvalidArgument);
    }

/* 
This creates the new account using the create_account system instruction, with the payer account paying for it. 
The invoke_signed function is used to sign the instruction with the payer account's signature, 
    and the data_account, payer, and system_program accounts are passed as arguments. 
The amount is also passed as a slice of bytes.
*/
    invoke_signed(
        &create_account(
            &payer.key,
            &expected_address,
            Rent::get()?.minimum_balance(std::mem::size_of::<MyData>()),
            std::mem::size_of::<MyData>()
                .try_into()
                .map_err(|_| ProgramError::InsufficientFunds)?,
            &program_id,
        ),
        &[payer.clone(), data_account.clone(), system_program.clone()],
        &[&[ amount.to_le_bytes().as_ref()]],
    )?;

/*
This serializes the new MyData struct and writes it to the data field of the new account. 
Finally, it returns an Ok result, indicating that the function executed successfully.
*/
    let new_data = MyData {
        amount: amount,
    };

    new_data.serialize(&mut data_account.try_borrow_mut_data()?.as_mut())?;

    Ok(())
}


/*
This is a function that modifies an existing account by changing the amount field to a new value new_amount.
*/
pub fn modify_acc(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
    new_amount: u64,
) -> ProgramResult {
/*
This initializes the accounts_iter variable to an iterator over the accounts slice, 
    and then uses the next_account_info function to get the next account info from the iterator: the data_account.
*/
    let accounts_iter = &mut accounts.iter();
    let data_account = next_account_info(accounts_iter)?;

/*
This creates a bump seed for the account address by converting the amount to little-endian bytes and then uses 
    the create_program_address function to generate the expected address of the existing account.
*/
    let bump = amount.to_le_bytes();
    let expected_address = Pubkey::create_program_address(
        &[&bump.as_ref()], program_id)?;

/*
This checks if the actual address of the data_account matches the expected address, 
    and if not, returns an InvalidArgument error.
*/
    if expected_address != *data_account.key {
        return Err(ProgramError::InvalidArgument);
    }

/* 
This creates a new MyData struct with the amount field set to the new value new_amount, serializes it, 
    and writes it to the data field of the existing account. Finally, it returns an Ok result, 
    indicating that the function executed successfully.
*/
    let new_data = MyData {
        amount: new_amount,
    };

    new_data.serialize(&mut data_account.try_borrow_mut_data()?.as_mut())?;
    Ok(())
}
