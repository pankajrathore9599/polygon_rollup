use k256::ecdsa::{Signature, VerifyingKey, signature::Verifier};
use bincode;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use sha2::{Sha256, Digest};
use merkle::MerkleTree;
use ring::digest::SHA256;

#[derive(serde::Serialize, serde::Deserialize)]
struct DataPacket {
    transactions: Vec<String>,
}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
    loop {
        let (socket, _) = listener.accept().await?;
        println!("Accepted Connection");
        tokio::spawn(handle_client(socket));
    }
}

async fn handle_client(mut socket: TcpStream) {
    // Reading public key
    let mut public_key_buffer = vec![0u8; 65];
    if let Err(e) = socket.read_exact(&mut public_key_buffer).await {
        println!("Failed to read public key: {}", e);
        return;
    }
    
    let verifying_key = match VerifyingKey::from_sec1_bytes(&public_key_buffer) {
        Ok(vk) => vk,
        Err(e) => {
            println!("Failed to decode public key: {:?}", e);
            return;
        }
    };

    // Reading data
    let mut buffer = vec![0; 1024];
    let n = match socket.read(&mut buffer).await {
        Ok(n) => n,
        Err(e) => {
            println!("Failed to read data: {}", e);
            return;
        }
    };

    let data: DataPacket = match bincode::deserialize(&buffer[..n]) {
        Ok(data) => data,
        Err(e) => {
            println!("Failed to deserialize data: {}", e);
            return;
        }
    };

    // Reading signature
    let mut signature_buffer = [0u8; 64];
    if let Err(e) = socket.read_exact(&mut signature_buffer).await {
        println!("Failed to read signature: {}", e);
        return;
    }
    
    let signature = match Signature::from_bytes((&signature_buffer).into()) {
        Ok(sig) => sig,
        Err(e) => {
            println!("Failed to extract signature: {}", e);
            return;
        }
    };

    // Verifying signature
    if verifying_key.verify(&buffer[..n], &signature).is_ok() {
        let hashes: Vec<_> = data.transactions.iter().map(|tx| Sha256::digest(tx.as_bytes()).to_vec()).collect();
        let tree = MerkleTree::from_vec(&SHA256, hashes);
        let root = &tree.root_hash();

        if let Err(e) = socket.write_all(root).await {
            println!("Failed to write response: {}", e);
        }
    } else {
        if let Err(e) = socket.write_all(b"Invalid signature").await {
            println!("Failed to write error response: {}", e);
        }
    }
}
