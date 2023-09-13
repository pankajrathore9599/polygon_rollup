use crate::shared::{Rollup, MainChain};
use merkle::MerkleTree;
use ring::digest::SHA256;
use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;
use bincode;

pub async fn start_main_chain() -> Result<(), Box<dyn std::error::Error>> {
    println!("Main chain started.");
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    let mut main_chain = MainChain { rollups: Vec::new() };
    let mut rollup_hashes: Vec<Vec<u8>> = vec![];

    loop {
        let (mut socket, _) = listener.accept().await?;
        let mut buffer = vec![0; 2048];
        let n = socket.read(&mut buffer).await?;
        let rollup: Rollup = bincode::deserialize(&buffer[..n])?;

        if rollup.hash == rollup.compute_hash() {
            // Convert the rollup to bytes
            let rollup_bytes = rollup_to_bytes(&rollup);
            rollup_hashes.push(rollup_bytes);

            // Create a Merkle tree with the hashes
            let tree = MerkleTree::from_vec(&SHA256, rollup_hashes.clone());

            main_chain.rollups.push(rollup);
            println!("Rollup added to the main chain. Total rollups: {}", main_chain.rollups.len());

            // Directly obtain the root hash of the Merkle tree
            let merkle_root = tree.root_hash();
            println!("Merkle root: {:?}", merkle_root);
        } else {
            println!("Rollup hash mismatch.");
        }
    }
}

fn rollup_to_bytes(rollup: &Rollup) -> Vec<u8> {
    bincode::serialize(rollup).unwrap()
}

