use crate::shared::{Transaction, Rollup, SignedTransaction};
use k256::PublicKey;
use k256::ecdsa::{VerifyingKey, Signature};
use k256::schnorr::signature::Verifier;
use sha2::digest::generic_array::GenericArray;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use bincode;
use bs58;
use std::collections::VecDeque;

async fn send_rollup_to_mainchain(rollup: Rollup) -> Result<(), Box<dyn std::error::Error>> {
    println!("Sending rollup to the main chain...");
    let mut stream = TcpStream::connect("127.0.0.1:8081").await?;
    let encoded_rollup = bincode::serialize(&rollup)?;
    stream.write_all(&encoded_rollup).await?;
    println!("Rollup sent to the main chain.");
    Ok(())
}

async fn handle_client(mut socket: TcpStream, collected_transactions: &mut VecDeque<Transaction>) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = vec![0; 1024];
    let n = socket.read(&mut buffer).await?;
    let signed_transaction: SignedTransaction = bincode::deserialize(&buffer[..n])?;

    let verifying_key = VerifyingKey::from(PublicKey::from_sec1_bytes(&signed_transaction.transaction.from)?);
    let signature_bytes = GenericArray::clone_from_slice(&signed_transaction.signature[..]);
    let valid_signature = verifying_key.verify(signed_transaction.transaction.compute_hash().as_slice().into(), &Signature::from_bytes(&signature_bytes)?).is_ok();

    if !valid_signature {
        println!("Invalid transaction signature.");
        return Ok(());
    }

    // Clone the transaction for printing and add it to the queue
    let cloned_transaction = signed_transaction.transaction.clone();
    collected_transactions.push_back(cloned_transaction);

    // Print the number of received transactions
    println!("Received {} Transaction(s).", collected_transactions.len());

    // Print the received transaction details.
    println!("Received Transaction Details:");
    println!("From (Base58): {:?}", bs58::encode(&signed_transaction.transaction.from).into_string());
    println!("To: {:?}", signed_transaction.transaction.to);
    println!("Amount: {:?}", signed_transaction.transaction.amount);
    println!("Nonce: {:?}", signed_transaction.transaction.nonce);
    println!("Timestamp: {:?}", signed_transaction.transaction.timestamp);
    println!("----------------------------------------");

    if collected_transactions.len() >= 5 {
        println!("Enough transactions collected. Preparing rollup...");
        let transactions: Vec<_> = collected_transactions.drain(..5).collect();
        let rollup = Rollup {
            transactions,
            hash: vec![],
        };
        let finalized_rollup = Rollup {
            transactions: rollup.transactions.clone(),
            hash: rollup.compute_hash(),
        };
        send_rollup_to_mainchain(finalized_rollup).await?;
    }

    Ok(())
}

pub async fn start_aggregator() -> Result<(), Box<dyn std::error::Error>> {
    println!("Aggregator started.");
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let mut collected_transactions: VecDeque<Transaction> = VecDeque::new();

    loop {
        let (socket, _) = listener.accept().await?;
        if let Err(e) = handle_client(socket, &mut collected_transactions).await {
            println!("Error handling client: {}", e);
        }
    }
}
