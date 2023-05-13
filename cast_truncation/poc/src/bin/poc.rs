use solana_client::rpc_client::RpcClient;

use solana_program::{
        instruction::{AccountMeta, Instruction},
        program_error::ProgramError,
    };
use solana_sdk::{
        signature::{Keypair, read_keypair_file}, 
        signer::Signer, 
        commitment_config::CommitmentConfig,
        transaction::Transaction,
        native_token::LAMPORTS_PER_SOL,
    };
    use std::mem::size_of;
use owo_colors::OwoColorize;

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

fn main() {

    let programa_keypair = read_keypair_file("./target/so/casting_truncation-keypair.json").unwrap();
    let programa = programa_keypair.pubkey();

    let cliente1 = String::from("http://localhost:8899");

    //let payer = Keypair::new();
    let authority = Keypair::new();
    let account = Keypair::new();

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

    let struct_data = MyStruct { 
        amount: 0
    }; 


    let data_len = (MyStruct::to_bytes(&struct_data)).len();
    let space:u64 = data_len as u64;
    
    let rent_exemption_amount = env.get_minimum_balance_for_rent_exemption(data_len).unwrap(); 

    let tx_create = solana_program::system_instruction::create_account( 
        &authority.pubkey(), 
        &account.pubkey(), 
        rent_exemption_amount, 
        space, 
        &programa, 

    ); 

    let recent_blockhash = env.get_latest_blockhash().unwrap(); 
    let tx_create = Transaction::new_signed_with_payer( 
        &[tx_create], 
        Some(&authority.pubkey()), 
        &[&authority,&account], 
        recent_blockhash, 
    ); 

    env.send_and_confirm_transaction(&tx_create).unwrap(); 

    println!("");
    println!("{}", "Working...".purple().bold());
    println!("");
    let tx_init = Instruction {
        program_id: programa,
        accounts: vec![
            AccountMeta::new(account.pubkey(), true),
        ],
        data: [0].into(),
    };
    let recent_blockhash = env.get_latest_blockhash().unwrap();

    let tx_init = Transaction::new_signed_with_payer(
        &[tx_init],
        Some(&authority.pubkey()),
        &[&authority, &account],
        recent_blockhash,
    );
        
    env.send_and_confirm_transaction(&tx_init).unwrap();

    let deser_data = env.get_account(&account.pubkey()).unwrap().data; 
    let vec_u8 = &deser_data; //&Vec<u8> 
    let data_u8 = &vec_u8; //&[u8] 
    println!("{} {} {}", "Rust provides no implicit type conversion (coercion) between primitive types. 
    But, explicit type conversion (casting) can be performed using the".red(), "as".bold().yellow(), "keyword.".red());
    println!("{}", "Check: https://doc.rust-lang.org/rust-by-example/types/cast.html".red());
    println!("Data: {:?}", MyStruct::from_bytes(data_u8).unwrap());
}