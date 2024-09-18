pub fn proof_of_work(block: &Block, difficulty: usize) -> bool {
    block.hash.starts_with(&"0".repeat(difficulty))
}

pub fn proof_of_learning(block: &Block, learning_threshold: u64) -> bool {
    block.index % learning_threshold == 0
}
