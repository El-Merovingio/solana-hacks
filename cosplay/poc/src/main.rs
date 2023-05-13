use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::{
    rpc_client::RpcClient,
};
use solana_program::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
    native_token::LAMPORTS_PER_SOL,
    
};
use owo_colors::OwoColorize;

// Define two enums
#[derive(BorshSerialize)]
enum SafeEnum {
    One,
    // Two,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
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
    pub fn process(&self) -> Result<(), ProgramError> {
        let data = SafeEnum::One.try_to_vec()?;
        self.safe_account.data.borrow_mut().copy_from_slice(&data);
        Ok(())
    }
}

// Define a struct that deserializes VulnerableEnum
#[derive(BorshSerialize, BorshDeserialize, Debug)]

pub struct DeserializeVulnerable<'a> {
    pub vulnerable_account: &'a AccountInfo<'a>,
}

impl<'a> DeserializeVulnerable<'a> {
    pub fn process(&self) -> Result<(), ProgramError> {
        let data = &self.vulnerable_account.data.borrow();
        let deserialized: VulnerableEnum = BorshDeserialize::try_from_slice(data)?;
        match deserialized {
            VulnerableEnum::One => {
                // This case is handled correctly
                Ok(())
            }
            VulnerableEnum::Two | VulnerableEnum::Three => {
                // These cases are not handled correctly and could be exploited by an attacker
                Err(ProgramError::Custom(1))
            }
        }
    }
}

fn main() {

    let programa_keypair = read_keypair_file("./target/so/cosplay-keypair.json").unwrap();
    let programa = programa_keypair.pubkey();

    let cliente1 = String::from("http://localhost:8899");

    //let payer = Keypair::new();
    let authority = Keypair::new();
    let safe_account = Keypair::new();
    let vulnerable_account = Keypair::new();

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

    let data_len = 1;
    let space:u64 = data_len as u64;
    
    let rent_exemption_amount = env.get_minimum_balance_for_rent_exemption(data_len).unwrap(); 

    let tx_create = solana_program::system_instruction::create_account( 
        &authority.pubkey(), 
        &safe_account.pubkey(), 
        rent_exemption_amount, 
        space, 
        &programa, 

    ); 

    let recent_blockhash = env.get_latest_blockhash().unwrap(); 
    let tx_create = Transaction::new_signed_with_payer( 
        &[tx_create], 
        Some(&authority.pubkey()), 
        &[&authority,&safe_account], 
        recent_blockhash, 
    ); 

    if let Err(err) = env.send_and_confirm_transaction(&tx_create) {
        println!("{:#?}", err);
        panic!();
    };

    let tx_create = solana_program::system_instruction::create_account( 
        &authority.pubkey(), 
        &vulnerable_account.pubkey(), 
        rent_exemption_amount, 
        space, 
        &programa, 

    ); 

    let recent_blockhash = env.get_latest_blockhash().unwrap(); 
    let tx_create = Transaction::new_signed_with_payer( 
        &[tx_create], 
        Some(&authority.pubkey()), 
        &[&authority,&vulnerable_account], 
        recent_blockhash, 
    ); 

    if let Err(err) = env.send_and_confirm_transaction(&tx_create) {
        println!("{:#?}", err);
        panic!();
    };

    println!("");
    println!("{}", "Working...".purple().bold());
    println!("");
    println!("Account 1: {}", safe_account.pubkey());
    println!("Account 2: {}", vulnerable_account.pubkey());

    let tx_init = Instruction {
        program_id: programa,
        accounts: vec![
            AccountMeta::new(safe_account.pubkey(), false),
        ],
        data: vec![0].into(),
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

    let ac1_d = env.get_account_data(&safe_account.pubkey()).unwrap();
    let ac2_d = env.get_account_data(&vulnerable_account.pubkey()).unwrap();

    println!("{}", "Duplicate Mutable Accounts, passing two different accounts".green());

    println!("Account 1 data: {:?}", ac1_d);
    println!("Account 2 data: {:?}", ac2_d);

    //Second call using same account
     let tx_init = Instruction {
        program_id: programa,
        accounts: vec![
            AccountMeta::new(vulnerable_account.pubkey(), false),
        ],
        data: vec![1].into(),
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

    let ac1_d = env.get_account_data(&safe_account.pubkey()).unwrap();
    let data1 = &ac1_d[..];
    let ac2_d = env.get_account_data(&vulnerable_account.pubkey()).unwrap();
    let data2 = &ac2_d[..];

    println!("SafeEnum::One.try_to_vec(): {:?}", SafeEnum::One.try_to_vec());
    println!("safe_account data: {:?}", VulnerableEnum::deserialize(&mut data1.as_ref()));
    println!("vulnerable_account data: {:?}", VulnerableEnum::deserialize(&mut data2.as_ref()));

    println!("");
    
    // Print the account information
    println!("Account safe_account: {:?}", env.get_account(&safe_account.pubkey()));
    println!("Account vulnerable_account: {:?}", env.get_account(&vulnerable_account.pubkey()));
}