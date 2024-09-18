use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, timestamp: u128, transactions: Vec<Transaction>) -> Self {
        let hash = Block::calculate_hash(index, &previous_hash, timestamp, &transactions);
        Block {
            index,
            previous_hash,
            timestamp,
            transactions,
            hash,
        }
    }

    pub fn calculate_hash(index: u64, previous_hash: &str, timestamp: u128, transactions: &Vec<Transaction>) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_string());
        hasher.update(previous_hash);
        hasher.update(timestamp.to_string());
        let transactions_json = serde_json::to_string(transactions).unwrap();
        hasher.update(transactions_json);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}
