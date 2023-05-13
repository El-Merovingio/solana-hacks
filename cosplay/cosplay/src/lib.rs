use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    msg
};

// Define two enums
#[allow(dead_code)]
#[derive(BorshSerialize)]
enum SafeEnum {
    One,
    Two,
}

#[derive(BorshSerialize, BorshDeserialize)]
enum VulnerableEnum {
    One,
    Two,
    Three,
}

// Define a struct that serializes SafeEnum
pub struct SerializeSafe<'a> {
    pub safe_account: &'a AccountInfo<'a>,
}

impl<'a> SerializeSafe<'a> {
    pub fn process(&self) -> ProgramResult {
        let data = SafeEnum::One.try_to_vec()?;
        self.safe_account.data.borrow_mut().copy_from_slice(&data);
        msg!("Serialized safe: {:?}", self.safe_account.data);
        Ok(())
    }
}

// Define a struct that deserializes VulnerableEnum
pub struct DeserializeVulnerable<'a> {
    pub vulnerable_account: &'a AccountInfo<'a>,
}

impl<'a> DeserializeVulnerable<'a> {
    pub fn process(&self) -> ProgramResult {
        let data = &self.vulnerable_account.data.borrow();
        let deserialized: VulnerableEnum = BorshDeserialize::try_from_slice(data)?;
        match deserialized {
            VulnerableEnum::One => {
                // This case is handled correctly
                msg!("Deserialized handled correctly: {:?}", self.vulnerable_account.data);
                Ok(())
            }
            VulnerableEnum::Two | VulnerableEnum::Three => {
                // These cases are not handled correctly and could be exploited by an attacker
                msg!("Deserialized not handled correctly: {:?}", self.vulnerable_account.data);
                Err(ProgramError::Custom(1))
            }
        }
    }
}

// The program entrypoint
entrypoint!(process_instruction);

pub fn process_instruction<'a>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data[0] {
        // Call SerializeSafe when the first byte of the instruction data is 0
        0 => {
            let accounts_iter = &mut accounts.iter();
            let safe_account = next_account_info(accounts_iter)?;
            let serialize_safe = SerializeSafe {
                safe_account,
            };
            serialize_safe.process()
        }
        // Call DeserializeVulnerable when the first byte of the instruction data is 1
        1 => {
            let accounts_iter = &mut accounts.iter();
            let vulnerable_account = next_account_info(accounts_iter)?;
            let deserialize_vulnerable = DeserializeVulnerable {
                vulnerable_account,
            };
            deserialize_vulnerable.process()
        }
        // Invalid instruction
        _ => Err(ProgramError::InvalidInstructionData),
    }
}