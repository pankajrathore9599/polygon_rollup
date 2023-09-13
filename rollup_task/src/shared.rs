use merkle::MerkleTree;
use ring::digest::SHA256;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub from: Vec<u8>,
    pub to: String,
    pub amount: u64,
    pub nonce: u64,
    pub timestamp: u64,
}

impl Transaction {
    pub fn compute_hash(&self) -> Vec<u8> {
        let encoded = bincode::serialize(&self).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(encoded);
        hasher.finalize().to_vec()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rollup {
    pub transactions: Vec<Transaction>,
    pub hash: Vec<u8>,
}

impl Rollup {
    pub fn compute_hash(&self) -> Vec<u8> {
        let hashes: Vec<_> = self.transactions.iter()
            .map(|tx| Sha256::digest(format!("{:?}{}{}{}{}", tx.from, tx.to, tx.amount, tx.nonce, tx.timestamp).as_bytes()).to_vec())
            .collect();
        
        // Now we're passing &SHA256 and hashes to the from_vec method
        let tree = MerkleTree::from_vec(&SHA256, hashes);
        
        tree.root_hash().to_vec()
    }
}

pub struct MainChain {
    pub rollups: Vec<Rollup>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignedTransaction {
    pub transaction: Transaction,
    pub signature: Vec<u8>,
}
