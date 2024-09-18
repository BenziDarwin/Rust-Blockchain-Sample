use crate::block::Block;
use crate::transaction::Transaction;
use crate::db::Db;
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json;

pub struct Blockchain {
    db: Db,
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn new(db_path: &str) -> Blockchain {
        let db = Db::new(db_path).expect("Failed to initialize database");

        let mut blockchain = Blockchain {
            db,
            chain: vec![],
        };

        // Load the blockchain from the database if it exists
        blockchain.load_chain_from_db();
        if blockchain.chain.is_empty() {
            blockchain.create_genesis_block();
        }

        blockchain
    }

    pub fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(
            0,
            "0".to_string(),
            Self::current_time(),
            vec![],
            "genesis_solution".to_string(), // Proof of Learning solution for the genesis block
        );

        self.chain.push(genesis_block.clone());
        self.save_block_to_db(&genesis_block);
    }

    pub fn create_transaction(&mut self, transaction: Transaction) {
        if let Some(current_block) = self.chain.last_mut() {
            current_block.transactions.push(transaction.clone());
            self.save_transaction_to_db(&transaction);
        } else {
            println!("No block available to add the transaction");
        }
    }

    pub fn mine_pending_transactions(&mut self) {
        let last_block = self.chain.last().unwrap();
        let mut solution = String::new();

        // Perform Proof of Learning (PoL) to find a valid solution
        loop {
            solution = self.generate_proof_of_learning();
            if self.is_valid_proof_of_learning(&solution) {
                break;
            }
        }

        let new_block = Block::new(
            self.chain.len() as u32,
            last_block.hash.clone(),
            Self::current_time(),
            vec![], // Add transactions from a pool or a buffer
            solution,
        );

        self.chain.push(new_block.clone());
        self.save_block_to_db(&new_block);
    }

    fn current_time() -> u128 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
    }

    fn generate_proof_of_learning(&self) -> String {
        // Generate a proof of learning (simplified placeholder)
        "example_solution".to_string()
    }

    fn is_valid_proof_of_learning(&self, solution: &str) -> bool {
        // Validate the proof of learning (simplified placeholder)
        solution == "example_solution"
    }

    pub fn get_chain(&self) -> &Vec<Block> {
        &self.chain
    }

    fn save_block_to_db(&self, block: &Block) {
        let block_key = format!("block:{}", block.index);
        let block_value = serde_json::to_string(block).expect("Failed to serialize block");
        self.db.save(&block_key, &block_value).expect("Failed to save block to DB");
    }

    fn load_chain_from_db(&mut self) {
        let mut index = 0;
        while let Ok(Some(block_value)) = self.db.get(&format!("block:{}", index)) {
            let block: Block = serde_json::from_str(&block_value).expect("Failed to deserialize block");
            self.chain.push(block);
            index += 1;
        }
    }

    fn save_transaction_to_db(&self, transaction: &Transaction) {
        let transaction_key = format!("transaction:{}", transaction.sender); // Example key
        let transaction_value = serde_json::to_string(transaction).expect("Failed to serialize transaction");
        self.db.save(&transaction_key, &transaction_value).expect("Failed to save transaction to DB");
    }
}
