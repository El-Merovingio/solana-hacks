use borsh::{BorshDeserialize, BorshSerialize};

use solana_client::rpc_client::RpcClient;
use solana_program::{
        instruction::{AccountMeta, Instruction},
        program_error::ProgramError,
        program_pack::{IsInitialized, Pack, Sealed},
    };
use solana_sdk::{
        signature::{Keypair, read_keypair_file}, 
        signer::Signer, 
        commitment_config::CommitmentConfig,
        transaction::Transaction,
        native_token::LAMPORTS_PER_SOL,
    };

use owo_colors::OwoColorize;

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


fn main() {

    let programa_keypair = read_keypair_file("./target/so/duplicated_accounts-keypair.json").unwrap();
    let programa = programa_keypair.pubkey();

    let cliente1 = String::from("http://localhost:8899");

    //let payer = Keypair::new();
    let authority = Keypair::new();
    let account1 = Keypair::new();
    let account2 = Keypair::new();

    let env = RpcClient::new_with_commitment(cliente1, CommitmentConfig::confirmed());

    match env.request_airdrop(&authority.pubkey(), LAMPORTS_PER_SOL) {
        Ok(sig) => loop {
            if let Ok(confirmed) = env.confirm_transaction(&sig) {
                if confirmed {
                    println!("Transaction: {} Status: {}", sig, confirmed);
                    break;
                }
            }
        },
        Err(_) => println!("Error requesting airdrop"),
    };

    let data_len = 8;
    let space:u64 = data_len as u64;
    
    let rent_exemption_amount = env.get_minimum_balance_for_rent_exemption(data_len).unwrap(); 

    let tx_create = solana_program::system_instruction::create_account( 
        &authority.pubkey(), 
        &account1.pubkey(), 
        rent_exemption_amount, 
        space, 
        &programa, 

    ); 

    let recent_blockhash = env.get_latest_blockhash().unwrap(); 
    let tx_create = Transaction::new_signed_with_payer( 
        &[tx_create], 
        Some(&authority.pubkey()), 
        &[&authority,&account1], 
        recent_blockhash, 
    ); 

    if let Err(err) = env.send_and_confirm_transaction(&tx_create) {
        println!("{:#?}", err);
        panic!();
    };

    let tx_create = solana_program::system_instruction::create_account( 
        &authority.pubkey(), 
        &account2.pubkey(), 
        rent_exemption_amount, 
        space, 
        &programa, 

    ); 

    let recent_blockhash = env.get_latest_blockhash().unwrap(); 
    let tx_create = Transaction::new_signed_with_payer( 
        &[tx_create], 
        Some(&authority.pubkey()), 
        &[&authority,&account2], 
        recent_blockhash, 
    ); 

    if let Err(err) = env.send_and_confirm_transaction(&tx_create) {
        println!("{:#?}", err);
        panic!();
    };

    println!("");
    println!("{}", "Working...".purple().bold());
    println!("");
    println!("Account 1: {}", account1.pubkey());
    println!("Account 2: {}", account2.pubkey());

    let tx_init = Instruction {
        program_id: programa,
        accounts: vec![
            AccountMeta::new(account1.pubkey(), false),
            AccountMeta::new(account2.pubkey(), false),
        ],
        data: MyInstruction::Update { a: 102400, b: 12800 }.try_to_vec().unwrap(),
    };
    let recent_blockhash = env.get_latest_blockhash().unwrap();

    let tx_init = Transaction::new_signed_with_payer(
        &[tx_init],
        Some(&authority.pubkey()),
        &[&authority],
        recent_blockhash,
    );
        
    if let Err(err) = env.send_and_confirm_transaction(&tx_init) {
        println!("{:#?}", err);
        panic!();
    };

    let ac1_d = env.get_account_data(&account1.pubkey()).unwrap();
    let ac2_d = env.get_account_data(&account2.pubkey()).unwrap();

    println!("{}", "Duplicate Mutable Accounts, passing two different accounts".green());

    println!("Account 1 data: {:?}", ac1_d);
    println!("Account 2 data: {:?}", ac2_d);


    /*
    Second call using same account
     */

     let tx_init = Instruction {
        program_id: programa,
        accounts: vec![
            AccountMeta::new(account1.pubkey(), false),
            AccountMeta::new(account1.pubkey(), false),
        ],
        data: MyInstruction::Update { a: 102400, b: 12800 }.try_to_vec().unwrap(),
    };
    let recent_blockhash = env.get_latest_blockhash().unwrap();

    let tx_init = Transaction::new_signed_with_payer(
        &[tx_init],
        Some(&authority.pubkey()),
        &[&authority],
        recent_blockhash,
    );
        
    if let Err(err) = env.send_and_confirm_transaction(&tx_init) {
        println!("{:#?}", err);
        panic!();
    };

    let ac1_d = env.get_account_data(&account1.pubkey()).unwrap();
    let ac2_d = env.get_account_data(&account2.pubkey()).unwrap();

    if ac1_d == ac2_d { 
        println!("{}", "Duplicate Mutable Accounts, passing the same account twice".red());
    }

    println!("Account 1 data: {:?}", ac1_d);
    println!("Account 2 data: {:?}", ac2_d);

}