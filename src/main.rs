use crate::blockchain::Blockchain;
use crate::transaction::Transaction;

mod blockchain;
mod block;
mod transaction;
mod consensus;

#[tokio::main]
async fn main() {
    let mut blockchain = Blockchain::new(4); // Example difficulty level

    let transactions = vec![
        Transaction { from: "Alice".to_string(), to: "Bob".to_string(), amount: 10 },
        Transaction { from: "Bob".to_string(), to: "Charlie".to_string(), amount: 20 },
    ];

    blockchain.add_block(transactions);

    println!("{:?}", blockchain.chain);
}
