use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::{rpc_client::RpcClient};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};
use solana_sdk::{
    commitment_config::CommitmentConfig, 
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
    native_token::LAMPORTS_PER_SOL,

};

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum MyInstruction {
    Create {amount: u64},
    Modify {amount: u64, new_amount: u64},
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct MyData {
    pub amount: u64,
}

#[allow(dead_code)]
fn main() {

    // Get the program account's public key
    let programa_keypair = read_keypair_file("./target/so/bump_canon-keypair.json").unwrap();
    let programa = programa_keypair.pubkey();

    let cliente1 = String::from("http://localhost:8899");

    // Create a new keypair to use as the signer
    let authority = Keypair::new();

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

    let amount: u64 = 10;

    let data_account = Pubkey::create_program_address(
        &[amount.to_le_bytes().as_ref()], &programa).unwrap();

    let tx_init = Instruction {
        program_id: programa,
        accounts: vec![
            AccountMeta::new(authority.pubkey(), true),
            AccountMeta::new(data_account, false),
            AccountMeta::new(system_program::id(), false),
        ],
        data: MyInstruction::Create {amount: amount}.try_to_vec().unwrap(),
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

    // Get the current value of the data
    let data_account_info = env.get_account_data(&data_account).unwrap();
    // let current_value = u64::from_le_bytes(data_account_info.as_slice().try_into().unwrap());

    // println!("Transaction signature: {}", result);
    println!("Created account address: {}", data_account);
    println!("Created account data info: {:?}", MyData::deserialize(&mut data_account_info.as_ref()).unwrap());
    // println!("Data account: {:?}", MyData::deserialize(data_account_info).unwrap());
    // println!("Current value: {}", current_value);

    let fake_account = Keypair::new();

    match env.request_airdrop(&fake_account.pubkey(), LAMPORTS_PER_SOL) {
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

    let new_amout:u64 = 1000;

    // let data_account = Pubkey::create_program_address(
    //     &[amount.to_le_bytes().as_ref()], &programa).unwrap();

    let tx_init = Instruction {
        program_id: programa,
        accounts: vec![
            AccountMeta::new(data_account, false),
        ],
        data: MyInstruction::Modify {amount: amount, new_amount: new_amout}.try_to_vec().unwrap(),
    };
    let recent_blockhash = env.get_latest_blockhash().unwrap();

    let tx_init = Transaction::new_signed_with_payer(
        &[tx_init],
        Some(&fake_account.pubkey()),
        &[&fake_account],
        recent_blockhash,
    );
        
    // println!("TX {:#?}", tx_init);    
    if let Err(err) = env.send_and_confirm_transaction(&tx_init) {
        println!("{:#?}", err);
        panic!();
    };

    let data_account_info = env.get_account_data(&data_account).unwrap();
    // let current_value = u64::from_le_bytes(data_account_info.as_slice().try_into().unwrap());

    // println!("Transaction signature: {}", result);
    println!("Modified account address: {}", data_account);
    println!("Created account data info: {:?}", MyData::deserialize(&mut data_account_info.as_ref()).unwrap());

}