use crate::shared::Transaction;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use bincode;

pub async fn send_transaction(transaction: Transaction) -> Result<(), Box<dyn std::error::Error>> {
    println!("Preparing to send transaction to the aggregator...");
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let encoded_data = bincode::serialize(&transaction)?;
    stream.write_all(&encoded_data).await?;
    println!("Transaction sent!");
    Ok(())
}

pub async fn start_client() -> Result<(), Box<dyn std::error::Error>> {
    println!("Client started.");
    let transaction = Transaction {
        from: "Alice".to_string(),
        to: "Bob".to_string(),
        amount: 100,
        nonce: 1,
        timestamp: 1677567654,
    };
    send_transaction(transaction).await?;
    Ok(())
}
