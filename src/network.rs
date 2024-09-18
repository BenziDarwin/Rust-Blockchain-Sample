use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::json;
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;

pub async fn start_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Server listening on port 8080");

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(handle_connection(socket));
    }
}

async fn handle_connection(mut socket: tokio::net::TcpStream) {
    let mut buf = [0; 1024];
    while let Ok(n) = socket.read(&mut buf).await {
        if n == 0 {
            return;
        }
        let received = String::from_utf8_lossy(&buf[..n]);
        println!("Received: {}", received);

        let response = match parse_request(&received) {
            Ok(response) => response,
            Err(err) => json!({ "status": "error", "message": err }).to_string(),
        };

        if let Err(_) = socket.write_all(response.as_bytes()).await {
            println!("Failed to send response");
            return;
        }
    }
}

fn parse_request(request: &str) -> Result<String, String> {
    let parts: Vec<&str> = request.trim().split_whitespace().collect();
    if parts.len() < 1 {
        return Err("Invalid request".to_string());
    }

    let db_path = "blockchain.db";
    let mut blockchain = Blockchain::new(db_path);

    match parts[0] {
        "CREATE_TRANSACTION" => {
            if parts.len() != 4 {
                return Err("Invalid CREATE_TRANSACTION request".to_string());
            }
            let transaction = Transaction::new(parts[1].to_string(), parts[2].to_string(), parts[3].parse().unwrap_or(0.0));
            blockchain.create_transaction(transaction);
            Ok(json!({ "status": "success", "message": "Transaction created" }).to_string())
        }
        "MINE" => {
            blockchain.mine_pending_transactions();
            Ok(json!({ "status": "success", "message": "Mining completed" }).to_string())
        }
        "GET_CHAIN" => {
            Ok(json!({ "status": "success", "chain": blockchain.get_chain() }).to_string())
        }
        _ => Err("Unknown command".to_string()),
    }
}

pub async fn send_message(message: &str) -> std::io::Result<()> {
    let mut stream = tokio::net::TcpStream::connect("127.0.0.1:8080").await?;
    stream.write_all(message.as_bytes()).await?;
    Ok(())
}
