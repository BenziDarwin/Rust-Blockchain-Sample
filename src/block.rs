use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub previous_hash: String,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub hash: String,
    pub proof_of_learning: String, // Field for the PoL solution
}

impl Block {
    pub fn new(index: u32, previous_hash: String, timestamp: u128, transactions: Vec<Transaction>, proof_of_learning: String) -> Block {
        let mut block = Block {
            index,
            previous_hash,
            timestamp,
            transactions,
            hash: String::new(),
            proof_of_learning,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}{:?}{}", self.index, self.previous_hash, self.timestamp, self.transactions, self.proof_of_learning));
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}
