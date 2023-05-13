#![allow(warnings)] 
use borsh::{BorshDeserialize, BorshSerialize};

use solana_client::rpc_client::RpcClient;
use std::str::FromStr;
use solana_program::{
        account_info::{next_account_info, AccountInfo},
        pubkey::Pubkey,
        instruction::{AccountMeta, Instruction},
        bpf_loader,
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
        bpf_loader_upgradeable::UpgradeableLoaderState,
    };
    use std::mem::size_of;
use owo_colors::OwoColorize;

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
    pub fn process(&self) -> Result<(), ProgramError> {
        let mut admin_data = Admin::try_from_slice(&self.admin_account.data.borrow())?;
        admin_data.admin = *self.new_admin_account.key;
        admin_data.serialize(&mut &mut self.admin_account.data.borrow_mut()[..])?;
        Ok(())
    }
  }
fn main() {

    let programa_keypair = read_keypair_file("./target/so/account_match-keypair.json").unwrap();
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

    let data_len = 32;
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
        data: MyInstruction::Update { }.try_to_vec().unwrap(),
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

    let program_accounts = env.get_program_accounts(
        &programa,
    );

    let ac1_d = env.get_account_data(&account1.pubkey()).unwrap();
    let ac2_d = env.get_account_data(&account2.pubkey()).unwrap();

    println!("{}", "Duplicate Mutable Accounts, passing two different accounts".green());

    println!("Account 1 data: {:?}", Admin::deserialize(&mut &ac1_d[..]).unwrap());
    println!("Account 2 data: {:?}", Admin::deserialize(&mut &ac2_d[..]).unwrap());

     let tx_init = Instruction {
        program_id: programa,
        accounts: vec![
            AccountMeta::new(account1.pubkey(), false),
            AccountMeta::new(account1.pubkey(), false),
        ],
        data: MyInstruction::Update { }.try_to_vec().unwrap(),
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

    let program_accounts = env.get_program_accounts(
        &programa,
    );

    let ac1_d = env.get_account_data(&account1.pubkey()).unwrap();
    let ac2_d = env.get_account_data(&account2.pubkey()).unwrap();

    if ac1_d == ac2_d { 
        println!("{}", "Duplicate Mutable Accounts, passing the same account twice".red());
    }

    println!("Account 1 data: {:?}", Admin::deserialize(&mut &ac1_d[..]).unwrap());
    println!("Account 2 data: {:?}", Admin::deserialize(&mut &ac2_d[..]).unwrap());

}