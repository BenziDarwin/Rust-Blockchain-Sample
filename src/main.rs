mod blockchain;
mod transaction;
mod merkle_tree;
mod db;
mod block;
mod network;

use network::start_server;
use tokio;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Start the server
    tokio::spawn(async {
        if let Err(e) = start_server().await {
            eprintln!("Server error: {}", e);
        }
    });

    // You can also add additional code to run in the main function if needed
    // Example: sending a test message
    // network::send_message("CREATE_TRANSACTION sender1 receiver1 10.0").await.unwrap();

    // Keep the main function running
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
