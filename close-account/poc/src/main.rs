use solana_client::rpc_client::RpcClient;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    native_token::LAMPORTS_PER_SOL,
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
};

fn main() {
    let programa_keypair = read_keypair_file("./target/so/close_account-keypair.json").unwrap();
    let programa = programa_keypair.pubkey();

    let cliente1 = String::from("http://localhost:8899");

    let authority = Keypair::new();
    let close_account = Keypair::new();

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

    let data_len = 0;
    let space: u64 = data_len as u64;

    let rent_exemption_amount = env
        .get_minimum_balance_for_rent_exemption(data_len)
        .unwrap();

    let mut tx = Vec::new();

    let tx_create = solana_program::system_instruction::create_account(
        &authority.pubkey(),
        &close_account.pubkey(),
        rent_exemption_amount,
        space,
        &programa,
    );

    tx.push(tx_create);

    let tx_init = Instruction {
        program_id: programa,
        accounts: vec![
            AccountMeta::new(close_account.pubkey(), false),
            AccountMeta::new(authority.pubkey(), false),
        ],
        data: vec![].into(),
    };

    tx.push(tx_init);

    let tx_transfer = solana_program::system_instruction::transfer(
        &authority.pubkey(),
        &close_account.pubkey(),
        rent_exemption_amount,
    );

    tx.push(tx_transfer);

    let recent_blockhash = env.get_latest_blockhash().unwrap();
    let tx_create = Transaction::new_signed_with_payer(
        &tx,
        Some(&authority.pubkey()),
        &[&authority,&close_account],
        recent_blockhash,
    );

    if let Err(err) = env.send_and_confirm_transaction(&tx_create) {
        println!("{:#?}", err);
        panic!();
    };

    println!(
        "Authority lamports: {:?}",
        env.get_account(&authority.pubkey()).unwrap().lamports
    );

    println!(
        "Close account lamports: {:?}",
        env.get_account(&close_account.pubkey()).unwrap().lamports
    );
}
