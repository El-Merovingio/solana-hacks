use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};


#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum MyInstruction {
    InconsistRounding {amount1: f64, amount2: f64},
    IncorrectCalc {amount1: f64, amount2: f64},
    ExponencialComplex {amount: f64}
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct MyData {
    pub amount: f64,
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    mut instruction_data: &[u8],
) -> ProgramResult {
    /* 
    This deserializes the instruction data into a MyInstruction enum and then matches 
        the variant to either call functions.
   */
    match MyInstruction::deserialize(&mut instruction_data)? {
        MyInstruction::InconsistRounding { amount1, amount2 } => inconsistent_rounding(&program_id, &accounts, amount1, amount2),
        MyInstruction::IncorrectCalc { amount1, amount2 } => incorrect_calculation(&program_id, &accounts, amount1, amount2),
        MyInstruction::ExponencialComplex { amount } => exponential_complexity(&program_id, &accounts, amount)
    }
}

// Function 1: Arithmetics have inconsistent rounding
pub fn inconsistent_rounding(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    // instruction_data: &[u8],
    amount1: f64,
    amount2: f64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    // Get the account that will hold the result
    let result_account = next_account_info(account_info_iter)?;

    // Compute the result as x/y
    let result = amount1 / amount2;

    // Save the result in new_data using the MyData Struct
    let new_data = MyData {
        amount: result,
    };

    new_data.serialize(&mut result_account.try_borrow_mut_data()?.as_mut())?;

    Ok(())
}

// Function 2: Incorrect calculation
pub fn incorrect_calculation(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    // instruction_data: &[u8],
    amount1: f64,
    amount2: f64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    // Get the account that will hold the result
    let result_account = next_account_info(account_info_iter)?;

    // Compute the result
    let result = amount1 * amount2;

    // Save the result in new_data using the MyData Struct
    let new_data = MyData {
        amount: result,
    };

    new_data.serialize(&mut result_account.try_borrow_mut_data()?.as_mut())?;

    Ok(())
}

// Function 3: Calculation has exponential complexity
pub fn exponential_complexity(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: f64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    // Get the account that will hold the result
    let result_account = next_account_info(account_info_iter)?;

    // Parse the instruction data as a u64 value
    let x = amount as u64;

    // Compute the factorial of n using a recursive function
    let result = factorial(x);

    // Save the result in new_data using the MyData Struct
    let new_data = MyData {
        amount: result,
    };
    // Write the result to the result account
    new_data.serialize(&mut result_account.try_borrow_mut_data()?.as_mut())?;
    
    Ok(())
}

// Recursive function to compute the factorial of a number with f64 values
fn factorial(n: u64) -> f64 {
    if n == 0 {
        1.0
    } else {
        n as f64 * factorial(n - 1)
    }
}