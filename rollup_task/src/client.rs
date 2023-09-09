use k256::ecdsa;
use k256::ecdsa::{SigningKey, VerifyingKey, signature::Signer};
use bincode;
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use rand::rngs::OsRng;

#[derive(serde::Serialize, serde::Deserialize)]
struct DataPacket {
    transactions: Vec<String>,
}

pub async fn start_client() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to server...");
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connected to server");
    
    let data = DataPacket {
        transactions: vec!["tx1".into(), "tx2".into(), "tx3".into(), "tx4".into()],
    };
    
    let encoded_data = bincode::serialize(&data)?;
    
    // Generate secret key and sign the data
    let signing_key = SigningKey::random(&mut OsRng);
    let verifying_key = VerifyingKey::from(&signing_key);
        
    // Serialize the public key and send it
    let encoded_point = verifying_key.to_encoded_point(false);
    stream.write_all(encoded_point.as_bytes()).await?;

    let signature: ecdsa::Signature = signing_key.sign(&encoded_data);
    stream.write_all(&encoded_data).await?;
    stream.write_all(&signature.to_bytes()).await?;

    let mut buffer = vec![0; 1024];
    stream.read(&mut buffer).await?;

    println!("Received Merkle Root: {}", String::from_utf8_lossy(&buffer));

    Ok(())
}
