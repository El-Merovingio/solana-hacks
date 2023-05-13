use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint,
  entrypoint::ProgramResult,
  pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
enum MyInstruction {
  Update { },
}

#[derive(Debug, Default, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Admin {
  pub admin: Pubkey,
}
pub struct UpdateAdmin<'a, 'b> {
  pub admin_account: &'a AccountInfo<'b>,
  pub new_admin_account: &'a AccountInfo<'b>,
}

impl UpdateAdmin<'_, '_> {
  pub fn process(&self) -> ProgramResult {
      let mut admin_data = Admin::try_from_slice(&self.admin_account.data.borrow())?;
      admin_data.admin = *self.new_admin_account.key;
      admin_data.serialize(&mut &mut self.admin_account.data.borrow_mut()[..])?;
      Ok(())
  }
}

entrypoint!(process_instruction);

pub fn process_instruction(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {

  match MyInstruction::try_from_slice(instruction_data)? {
    MyInstruction::Update { } => {
        update_admin(program_id, accounts)
    }
  }
}

pub fn update_admin(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
  let accounts_iter = &mut accounts.iter();
  let admin_account = next_account_info(accounts_iter)?; 
  let new_admin_account = next_account_info(accounts_iter)?;
  let update_admin = UpdateAdmin {
    admin_account: admin_account,
    new_admin_account: new_admin_account,
  };
  update_admin.process()
}
