use crate::block::Block;
use crate::transaction::Transaction;
use crate::consensus::{proof_of_work, proof_of_learning};

pub struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty,
        };
        blockchain.create_genesis_block();
        blockchain
    }

    pub fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(0, "0".to_string(), 0, vec![]);
        self.chain.push(genesis_block);
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_block = self.get_last_block();
        let mut new_block = Block::new(
            previous_block.index + 1,
            previous_block.hash.clone(),
            chrono::Utc::now().timestamp_millis() as u128,
            transactions,
        );

        while !proof_of_work(&new_block, self.difficulty) {
            new_block = Block::new(
                previous_block.index + 1,
                previous_block.hash.clone(),
                chrono::Utc::now().timestamp_millis() as u128,
                transactions.clone(),
            );
        }

        self.chain.push(new_block);
    }

    pub fn get_last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }
}
