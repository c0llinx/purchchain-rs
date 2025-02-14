use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use crate::crypto::hash::compute_sha256;
use crate::accounts::transactions::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            nonce: 0,
            hash: String::new(),
        }
    }

    /// Computes the SHA256 hash of the block (excluding the stored hash field).
    pub fn compute_hash(&self) -> String {
        // Serialize only the data used for hashing.
        let block_data = serde_json::to_string(&(
            self.index,
            self.timestamp,
            self.transactions,
            self.previous_hash,
            self.nonce,
        ))
            .unwrap();
        compute_sha256(&block_data)
    }
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
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

    fn create_genesis_block(&mut self) {
        let mut genesis = Block::new(0, vec![], String::from("0"));
        genesis.hash = genesis.compute_hash();
        self.chain.push(genesis);
    }

    pub fn get_last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    /// Mines the given block by finding a hash with the required number of leading zeros.
    pub fn proof_of_work(&self, mut block: Block) -> String {
        let target = "0".repeat(self.difficulty);
        loop {
            let hash = block.compute_hash();
            if hash.starts_with(&target) {
                return hash;
            }
            block.nonce += 1;
        }
    }

    /// Adds a block to the chain after computing proof-of-work.
    pub fn add_block(&mut self, mut block: Block) -> bool {
        block.previous_hash = self.get_last_block().hash.clone();
        block.hash = self.proof_of_work(block.clone());
        if self.validate_block(&block) {
            self.chain.push(block);
            true
        } else {
            false
        }
    }

    pub fn validate_block(&self, block: &Block) -> bool {
        let target = "0".repeat(self.difficulty);
        block.hash.starts_with(&target) && block.hash == block.compute_hash()
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];
            if current.previous_hash != previous.hash || !self.validate_block(current) {
                return false;
            }
        }
        true
    }
}
