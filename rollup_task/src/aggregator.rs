use crate::shared::{Transaction, Rollup};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use bincode;
use std::collections::VecDeque;

async fn send_rollup_to_mainchain(rollup: Rollup) -> Result<(), Box<dyn std::error::Error>> {
    println!("Sending rollup to the main chain...");
    let mut stream = TcpStream::connect("127.0.0.1:8081").await?;
    let encoded_rollup = bincode::serialize(&rollup)?;
    stream.write_all(&encoded_rollup).await?;
    println!("Rollup sent to the main chain.");
    Ok(())
}

pub async fn start_aggregator() -> Result<(), Box<dyn std::error::Error>> {
    println!("Aggregator started.");
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let mut collected_transactions: VecDeque<Transaction> = VecDeque::new();

    loop {
        let (mut socket, _) = listener.accept().await?;
        let mut buffer = vec![0; 1024];
        let n = socket.read(&mut buffer).await?;
        let transaction: Transaction = bincode::deserialize(&buffer[..n])?;
        println!("Received transaction from {} to {}.", transaction.from, transaction.to);
        collected_transactions.push_back(transaction.clone());


        if collected_transactions.len() >= 5 {
            println!("Enough transactions collected. Preparing rollup...");
            let transactions: Vec<_> = collected_transactions.drain(0..5).collect();
            let mut rollup = Rollup {
                transactions: transactions,
                hash: vec![], // Temporary placeholder
            };
            rollup.hash = rollup.compute_hash(); // Compute the hash
            send_rollup_to_mainchain(rollup).await?;
        }
    }
}
