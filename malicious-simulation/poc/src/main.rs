use solana_client::rpc_client::RpcClient;

use solana_program::{
    instruction::{AccountMeta, Instruction},
    system_program
    };
use solana_sdk::{
    commitment_config::CommitmentConfig,
    native_token::LAMPORTS_PER_SOL,
    signature::{read_keypair_file, Keypair, Signer},
    sysvar::slot_history,
    transaction::Transaction,
};
// use owo_colors::OwoColorize;
#[allow(dead_code)]
fn main() {
    let programa_keypair =
        read_keypair_file("./target/so/malicious_simulation-keypair.json").unwrap();
    let programa = programa_keypair.pubkey();

    let cliente1 = String::from("http://localhost:8899");

    //let payer = Keypair::new();
    let authority = Keypair::new();
    let victim = Keypair::new();

    let env = RpcClient::new_with_commitment(cliente1, CommitmentConfig::confirmed());

    match env.request_airdrop(&authority.pubkey(), LAMPORTS_PER_SOL * 5) {
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

    let data_len = 0;
    let space: u64 = data_len as u64;

    let tx_create = solana_program::system_instruction::create_account(
        &authority.pubkey(),
        &victim.pubkey(),
        LAMPORTS_PER_SOL * 2,
        space,
        &system_program::ID,
    );


    let recent_blockhash = env.get_latest_blockhash().unwrap();
    let tx_init = Transaction::new_signed_with_payer(
        &[tx_create],
        Some(&authority.pubkey()),
        &[&authority,&victim],
        recent_blockhash,
    );

    if let Err(err) = env.send_and_confirm_transaction(&tx_init) {
        println!("{:#?}", err);
        panic!();
    };
    println!("");
    println!("Program address: {}", programa);
    println!("");
    println!("Authority/Attacker address: {}", authority.pubkey());
    println!("Authority/Attacker account lamports #1: {:?}",
        env.get_account(&authority.pubkey()).unwrap().lamports
    );

    println!("Victim address: {}", victim.pubkey());
    println!("Victim account lamports #1: {:?}",
        env.get_account(&victim.pubkey()).unwrap().lamports
    );

    let tx_init = Instruction {
        program_id: programa,
        accounts: vec![
            AccountMeta::new(victim.pubkey(), true),
            AccountMeta::new(slot_history::ID, false),
            AccountMeta::new(system_program::ID, false),
            AccountMeta::new(authority.pubkey(), true),
        ],
        data: vec![].into(),
    };


    let recent_blockhash = env.get_latest_blockhash().unwrap();
    let tx_init = Transaction::new_signed_with_payer(
        &[tx_init],
        Some(&authority.pubkey()),
        &[&authority,&victim],
        recent_blockhash,
    );

    // println!("TX {:#?}", tx_init);
    if let Err(err) = env.send_and_confirm_transaction(&tx_init) {
        println!("{:#?}", err);
        // panic!();
    };

    println!("");
    println!("Final Authority/Attacker account lamports: {:?}",
        env.get_account(&authority.pubkey()).unwrap().lamports
    );

    println!("Final Victim account lamports: {:?}",
    env.get_account(&victim.pubkey()).unwrap().lamports
    );
}
