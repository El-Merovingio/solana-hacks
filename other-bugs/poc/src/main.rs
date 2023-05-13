use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::{rpc_client::RpcClient};
use solana_program::{
    instruction::{AccountMeta, Instruction},
};

use solana_sdk::{
    commitment_config::CommitmentConfig, 
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
    native_token::LAMPORTS_PER_SOL,

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

#[allow(dead_code)]
fn main() {

    // Get the program account's public key
    let programa_keypair = read_keypair_file("./target/so/other_bugs-keypair.json").unwrap();
    let programa = programa_keypair.pubkey();

    let cliente1 = String::from("http://localhost:8899");

    // Create a new keypair to use as the signer
    let authority = Keypair::new();
    let result_account = Keypair::new();

    // Create a new account to store the data
    // let data_account = Keypair::new();

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

    let amount1: f64 = 0.1;
    let amount2: f64 = 0.3;

    let data_len = std::mem::size_of::<MyData>();
    let space:u64 = data_len as u64;
    
    let rent_exemption_amount = env.get_minimum_balance_for_rent_exemption(data_len).unwrap(); 

    let tx_create = solana_program::system_instruction::create_account( 
        &authority.pubkey(), 
        &result_account.pubkey(), 
        rent_exemption_amount, 
        space, 
        &programa, 

    ); 

    let recent_blockhash = env.get_latest_blockhash().unwrap();

    let tx_init = Transaction::new_signed_with_payer(
        &[tx_create],
        Some(&authority.pubkey()),
        &[&authority,&result_account],
        recent_blockhash,
    );
        
    // println!("TX {:#?}", tx_init);    
    if let Err(err) = env.send_and_confirm_transaction(&tx_init) {
        println!("{:#?}", err);
        panic!();
    };


    let tx_init = Instruction {
        program_id: programa,
        accounts: vec![
            AccountMeta::new(result_account.pubkey(), false),
        ],
        data: MyInstruction::InconsistRounding { amount1: amount1, amount2: amount2 }.try_to_vec().unwrap(),
    };
    let recent_blockhash = env.get_latest_blockhash().unwrap();

    let tx_init = Transaction::new_signed_with_payer(
        &[tx_init],
        Some(&authority.pubkey()),
        &[&authority],
        recent_blockhash,
    );
        
    // println!("TX {:#?}", tx_init);    
    if let Err(err) = env.send_and_confirm_transaction(&tx_init) {
        println!("{:#?}", err);
        panic!();
    };

    let data_account_info = env.get_account_data(&result_account.pubkey()).unwrap();

    println!("Created account data info: {:?}", MyData::deserialize(&mut data_account_info.as_ref()).unwrap());

    let amount1: f64 = 0.1;
    let amount2: f64 = 3.0;

    let tx_init = Instruction {
        program_id: programa,
        accounts: vec![
            AccountMeta::new(result_account.pubkey(), false),
        ],
        data: MyInstruction::IncorrectCalc { amount1: amount1, amount2: amount2 }.try_to_vec().unwrap(),
    };
    let recent_blockhash = env.get_latest_blockhash().unwrap();

    let tx_init = Transaction::new_signed_with_payer(
        &[tx_init],
        Some(&authority.pubkey()),
        &[&authority],
        recent_blockhash,
    );
        
    // println!("TX {:#?}", tx_init);    
    if let Err(err) = env.send_and_confirm_transaction(&tx_init) {
        println!("{:#?}", err);
        panic!();
    };

    let data_account_info = env.get_account_data(&result_account.pubkey()).unwrap();

    println!("Created account data info: {:?}", MyData::deserialize(&mut data_account_info.as_ref()).unwrap());

    let amount = 1e20;   // 1 with 20 zeros

    let tx_init = Instruction {
        program_id: programa,
        accounts: vec![
            AccountMeta::new(result_account.pubkey(), false),
        ],
        data: MyInstruction::ExponencialComplex{amount: amount }.try_to_vec().unwrap(),
    };
    let recent_blockhash = env.get_latest_blockhash().unwrap();

    let tx_init = Transaction::new_signed_with_payer(
        &[tx_init],
        Some(&authority.pubkey()),
        &[&authority],
        recent_blockhash,
    );
        
    // println!("TX {:#?}", tx_init);    
    if let Err(err) = env.send_and_confirm_transaction(&tx_init) {
        println!("{:#?}", err);
        panic!();
    };
}