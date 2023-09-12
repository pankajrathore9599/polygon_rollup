use merkle::MerkleTree;
use ring::digest::SHA256;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub nonce: u64,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rollup {
    pub transactions: Vec<Transaction>,
    pub hash: Vec<u8>,
}

impl Rollup {
    pub fn compute_hash(&self) -> Vec<u8> {
        let hashes: Vec<_> = self.transactions.iter()
            .map(|tx| Sha256::digest(format!("{:?}{}{}{}{}", tx.from, tx.to, tx.amount, tx.nonce, tx.timestamp).as_bytes()).to_vec())
            .collect();
        let tree = MerkleTree::from_vec(&SHA256, hashes);
        tree.root_hash().to_vec()
    }
}

pub struct MainChain {
    pub rollups: Vec<Rollup>,
}
