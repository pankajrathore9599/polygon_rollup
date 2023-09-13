use k256::ecdsa::{SigningKey, Signature};
use k256::schnorr::signature::Signer;
use k256::{SecretKey, PublicKey};
use k256::elliptic_curve::rand_core::RngCore;
use crate::shared::{Transaction, SignedTransaction};
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use bincode;
use bs58;
use rand::Rng;
use chrono::Utc;

pub async fn send_transaction(secret_key: SecretKey) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();

    // Generating a random amount
    let amount = rng.gen_range(1..=200);

    // Get the current timestamp as u64
    let timestamp = Utc::now().timestamp();

    let transaction = Transaction {
        from: PublicKey::from(SigningKey::from(secret_key.clone()).verifying_key()).to_sec1_bytes().to_vec(),
        to: "Bob".to_string(),
        amount,
        nonce: 1,
        timestamp: timestamp as u64,
    };

    let hash = transaction.compute_hash();
    let signing_key = SigningKey::from(secret_key.clone()); // Clone the secret key
    let signature: Signature = signing_key.sign(&hash);
    let signed_transaction = SignedTransaction {
        transaction: transaction.clone(), // Clone the transaction for printing
        signature: signature.to_bytes().to_vec(),
    };
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let encoded_data = bincode::serialize(&signed_transaction)?;
    stream.write_all(&encoded_data).await?;

    // Print the complete transaction details.
    println!("Transaction sent successfully!");
    println!("Transaction Details:");
    println!("From (Base58): {:?}", bs58::encode(signed_transaction.transaction.from).into_string());
    println!("To: {:?}", signed_transaction.transaction.to);
    println!("Amount: {:?}", signed_transaction.transaction.amount);
    println!("Nonce: {:?}", signed_transaction.transaction.nonce);
    println!("Timestamp: {:?}", signed_transaction.transaction.timestamp);

    Ok(())
}

pub async fn start_client() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);
    let secret_key = SecretKey::from_bytes((&bytes).into())?;

    println!("Starting the client...");
    send_transaction(secret_key).await?;
    println!("Client finished successfully!");

    Ok(())
}
