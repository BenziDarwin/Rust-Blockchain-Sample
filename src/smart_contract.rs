use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SmartContract {
    pub id: String,
    pub code: String,
}

impl SmartContract {
    pub fn new(id: String, code: String) -> SmartContract {
        SmartContract { id, code }
    }

    pub fn execute(&self) {
        println!("Executing smart contract with ID: {}", self.id);
        // Simulate execution
        println!("Code: {}", self.code);
    }
}
