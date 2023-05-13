use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint,
  entrypoint::ProgramResult,
  program_error::ProgramError,
  program_pack::{IsInitialized, Pack, Sealed},
  pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {

    match MyInstruction::try_from_slice(instruction_data)? {
      MyInstruction::Update { a, b } => {
          update(program_id, accounts, a, b)
      }
  }
}

fn update(_program_id: &Pubkey, accounts: &[AccountInfo], a: u64, b: u64) -> ProgramResult {
  let accounts_iter = &mut accounts.iter();

  let user_a = next_account_info(accounts_iter)?;
  let user_b = next_account_info(accounts_iter)?;

  let mut user_a_data = User::unpack_unchecked(&user_a.data.borrow())?;
  let mut user_b_data = User::unpack_unchecked(&user_b.data.borrow())?;

  user_a_data.data = a;
  user_b_data.data = b;

  User::pack(user_a_data, &mut user_a.data.borrow_mut())?;
  User::pack(user_b_data, &mut user_b.data.borrow_mut())?;

  Ok(())
}

// #[derive(Debug, PartialEq)]
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
enum MyInstruction {
  Update { a: u64, b: u64 },
}

impl Sealed for MyInstruction {}
impl IsInitialized for MyInstruction {
  fn is_initialized(&self) -> bool {
      true
  }
}

impl Pack for MyInstruction {
  const LEN: usize = 17;

  fn pack_into_slice(&self, dst: &mut [u8]) {
      match self {
        MyInstruction::Update { a, b } => {
              dst[0..1].copy_from_slice(&[0]);
              dst[1..9].copy_from_slice(&a.to_le_bytes());
              dst[9..17].copy_from_slice(&b.to_le_bytes());
          }
      }
  }

  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
      let instruction_type = src[0];
      match instruction_type {
          0 => {
              let a = u64::from_le_bytes(src[1..9].try_into().unwrap());
              let b = u64::from_le_bytes(src[9..17].try_into().unwrap());
              Ok(MyInstruction::Update { a, b })
          }
          _ => Err(ProgramError::InvalidInstructionData),
      }
  }
}

#[derive(Debug, PartialEq)]
pub struct User {
  pub data: u64,
}

impl Sealed for User {}
impl IsInitialized for User {
  fn is_initialized(&self) -> bool {
      true
  }
}

impl Pack for User {
  const LEN: usize = 8;

  fn pack_into_slice(&self, dst: &mut [u8]) {
      dst.copy_from_slice(&self.data.to_le_bytes());
  }

  fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
      Ok(User {
          data: u64::from_le_bytes(src.try_into().unwrap()),
      })
  }
}
