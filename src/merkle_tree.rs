use sha2::{Digest, Sha256};

pub fn calculate_merkle_root(transactions: &[String]) -> String {
    let mut hashes: Vec<String> = transactions.iter().map(|tx| sha256(tx)).collect();
    while hashes.len() > 1 {
        let mut new_hashes = vec![];
        for pair in hashes.chunks(2) {
            let hash = if pair.len() == 2 {
                sha256(&format!("{}{}", pair[0], pair[1]))
            } else {
                sha256(&pair[0])
            };
            new_hashes.push(hash);
        }
        hashes = new_hashes;
    }
    hashes[0].clone()
}

fn sha256(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result)
}
