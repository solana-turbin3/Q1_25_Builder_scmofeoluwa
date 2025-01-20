mod programs;

use std::str::FromStr;

use crate::programs::Turbin3_prereq::{Turbin3PrereqProgram, CompleteArgs};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{message::Message, pubkey::Pubkey, signature::{read_keypair_file, Keypair, Signer}, system_instruction::transfer, system_program, transaction::Transaction};

const RPC_URL: &str = "https://api.devnet.solana.com";

#[test]
fn keygen() {
    // Create a new keypair
    let kp = Keypair::new();
    println!(
        "You've generated a new Solana wallet: {}",
        kp.pubkey().to_string()
    );
    println!("To save your wallet, copy and paste the following into a JSON file:");
    println!("{:?}", kp.to_bytes());
}

#[test]
fn airdrop() {
    // Import our keypair
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

    // Connect to Solana Devnet RPC Client
    let client = RpcClient::new(RPC_URL);

    // Claim 2 devnet SOL tokens (2 billion lamports)
    match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
        Ok(signature) => {
            println!("Success! Check out your TX here:");
            println!("https://explorer.solana.com/tx/{}?cluster=devnet", signature);
        },
        Err(e) => println!("Oops, something went wrong: {}", e),
    }
}

#[test]
fn transfer_sol() {
    // Import our keypair
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

    // Define your Turbin3 public key
    let to_pubkey = Pubkey::from_str("CgBgYuveQN4zmxYKSMu32yzzkWp1jtqu8oUhgmxvDvRC").unwrap();

    // Create a Solana devnet connection
    let rpc_client = RpcClient::new(RPC_URL);

    // Get recent blockhash
    let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

    // Create and sign the transaction
    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash
    );

    // Send the transaction
    let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");

    // Print the transaction signature
    println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
}

#[test]
fn empty_wallet() {
    // Import our keypair
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

    // Define your Turbin3 public key
    let to_pubkey = Pubkey::from_str("CgBgYuveQN4zmxYKSMu32yzzkWp1jtqu8oUhgmxvDvRC").unwrap();

    // Create a Solana devnet connection
    let rpc_client = RpcClient::new(RPC_URL);

    // Get balance of dev wallet
    let balance = rpc_client.get_balance(&keypair.pubkey()).expect("Failed to get balance");

    // Get recent blockhash
    let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

    // Create a test transaction to calculate fees
    let message = Message::new_with_blockhash(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
        Some(&keypair.pubkey()),
        &recent_blockhash
    );

    // Calculate exact fee rate to transfer entire SOL amount out of account minus fees
    let fee = rpc_client.get_fee_for_message(&message).expect("Failed to get fee calculator");

    // Deduct fee from lamports amount and create a TX with correct balance
    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash
    );

    // Send the transaction
    let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");

    // Print the transaction signature
    println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
}

#[test]
fn enroll() {
    // Create a Solana devnet connection
    let rpc_client = RpcClient::new(RPC_URL);

    // Import Turbin3 walletz
    let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");

    // Create PDA for prereq account
    let prereq = Turbin3PrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);

    // Define instruction data
    let args = CompleteArgs {
        github: b"ScMofeoluwa".to_vec(),
    };

    // Get recent blockhash
    let blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

    // Create and send the transaction
    let transaction = Turbin3PrereqProgram::complete(
        &[&signer.pubkey(), &prereq, &system_program::id()],
        &args,
        Some(&signer.pubkey()),
        &[&signer],
        blockhash,
    );

    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
}
