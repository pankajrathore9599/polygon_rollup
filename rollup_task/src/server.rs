use k256::ecdsa::{Signature, VerifyingKey, signature::Verifier};
use bincode;
use tokio::net::TcpListener;
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
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            // Reading the public key from the client
            let mut public_key_buffer = vec![0u8; 33];
            socket.read_exact(&mut public_key_buffer).await.expect("Failed to read public key");
            
            let verifying_key = VerifyingKey::from_sec1_bytes(&public_key_buffer).expect("Failed to decode public key");

            let mut buffer = vec![0; 1024];
            let n = socket.read(&mut buffer).await.expect("Failed to read data");
            
            let data: DataPacket = bincode::deserialize(&buffer[..n]).expect("Failed to deserialize data");
            
            let mut signature_buffer = [0u8; 64];
            socket.read_exact(&mut signature_buffer).await.expect("Failed to read signature");
            
            let signature = Signature::from_bytes((&signature_buffer).into()).expect("Failed to extract signature");
            
            if verifying_key.verify(&buffer[..n], &signature).is_ok() {
                let hashes: Vec<_> = data.transactions.iter().map(|tx| Sha256::digest(tx.as_bytes()).to_vec()).collect();
                
                // Provide the hashing algorithm (SHA256 in this case) when creating the Merkle tree
                let tree = MerkleTree::from_vec(&SHA256, hashes);
                let root = &tree.root_hash();
            
                socket.write_all(root).await.expect("Failed to write response");
            } else {
                socket.write_all(b"Invalid signature").await.expect("Failed to write error response");
            }
        });
    }
}
