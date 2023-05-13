use borsh::{BorshDeserialize, BorshSerialize};
use arrayref::{array_ref};

use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint,
  entrypoint::ProgramResult,
  msg,
  program_error::ProgramError,
  program_pack::{IsInitialized, Pack, Sealed},
  pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
enum MyInstruction {
  Init {},
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct User {
  authority: Pubkey,
}

impl Sealed for User {}

impl IsInitialized for User {
  fn is_initialized(&self) -> bool {
      true
  }
}

impl Pack for User {
  const LEN: usize = 32;

  fn pack_into_slice(&self, dst: &mut [u8]) {
      dst.copy_from_slice(self.authority.as_ref())
  }

  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
      Ok(Self {
          authority: Pubkey::new_from_array(*array_ref![src, 0, 32]),
      })
  }
}

entrypoint!(process_instruction);

pub fn process_instruction(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    match MyInstruction::try_from_slice(instruction_data)? {
      MyInstruction::Init{} => initialize(program_id, accounts, instruction_data),
  }
}

pub fn initialize(_program_id: &Pubkey, accounts: &[AccountInfo], _instruction_data: &[u8]) -> ProgramResult {
  let account_info_iter = &mut accounts.iter();

  let user_info = next_account_info(account_info_iter)?;
  let authority_info = next_account_info(account_info_iter)?;
  
  let mut user_data = user_info.try_borrow_mut_data()?;
  let mut user = User::unpack_unchecked(&user_data)?;
  msg!("User authority address is: {}", user.authority);

  user.authority = *authority_info.key;
  user.pack_into_slice(&mut user_data);

  msg!("User account initialized successfully.");
  msg!("User authority address is: {}", user.authority);
  Ok(())
}
