use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::{rpc_client::RpcClient};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use solana_sdk::{
    commitment_config::CommitmentConfig, 
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
    native_token::LAMPORTS_PER_SOL,

};
pub const MAX_API_LENGTH: usize = 50;

// Define the User struct
#[derive(Debug, BorshSerialize, BorshDeserialize)]
struct User {
    key: Pubkey,
    name: String,
}

#[allow(dead_code)]
fn main() {

    // Get the program account's public key
    let programa_keypair = read_keypair_file("./target/so/rent_not_handled-keypair.json").unwrap();
    let programa = programa_keypair.pubkey();

    let cliente1 = String::from("http://localhost:8899");

    // Create a new keypair to use as the signer
    let authority = Keypair::new();
    let user_account = Keypair::new();

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

    let data_len = std::mem::size_of::<User>();
    let space:u64 = data_len as u64;
    
    let rent_exemption_amount = env.get_minimum_balance_for_rent_exemption(data_len).unwrap(); 

    let tx_create = solana_program::system_instruction::create_account( 
        &authority.pubkey(), 
        &user_account.pubkey(), 
        rent_exemption_amount, 
        space, 
        &programa, 

    ); 

    let recent_blockhash = env.get_latest_blockhash().unwrap();

    let tx_init = Transaction::new_signed_with_payer(
        &[tx_create],
        Some(&authority.pubkey()),
        &[&authority,&user_account],
        recent_blockhash,
    );
        
    // println!("TX {:#?}", tx_init);    
    if let Err(err) = env.send_and_confirm_transaction(&tx_init) {
        println!("{:#?}", err);
        panic!();
    };

    let name = b"Julio";
    let tx_init = Instruction {
        program_id: programa,
        accounts: vec![
            AccountMeta::new(user_account.pubkey(), false),
        ],
        data: name.try_to_vec().unwrap(),
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

    let data_account_info = env.get_account_data(&user_account.pubkey()).unwrap();

    println!("Created account data info: {:?}", User::deserialize(&mut data_account_info.as_ref()).unwrap());

    // Second without the correct len

    let another = Keypair::new();


    let data_len = 0;
    let space:u64 = data_len as u64;
    
    let rent_exemption_amount = env.get_minimum_balance_for_rent_exemption(data_len).unwrap(); 

    let tx_create = solana_program::system_instruction::create_account( 
        &authority.pubkey(), 
        &another.pubkey(), 
        rent_exemption_amount, 
        space, 
        &programa, 

    ); 

    let recent_blockhash = env.get_latest_blockhash().unwrap();

    let tx_init = Transaction::new_signed_with_payer(
        &[tx_create],
        Some(&authority.pubkey()),
        &[&authority,&another],
        recent_blockhash,
    );
           
    if let Err(err) = env.send_and_confirm_transaction(&tx_init) {
        println!("{:#?}", err);
        panic!();
    };

    let name = b"Javierito";
    let tx_init = Instruction {
        program_id: programa,
        accounts: vec![
            AccountMeta::new(another.pubkey(), false),
        ],
        data: name.try_to_vec().unwrap(),
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

    let data_account_info = env.get_account_data(&another.pubkey()).unwrap();

    println!("Created account data info: {:?}", User::deserialize(&mut data_account_info.as_ref()).unwrap());


}