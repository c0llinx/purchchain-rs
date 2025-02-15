use crate::core::blockchain::Block;
use reqwest;
use serde_json::json;
use crate::internal::identity::get_node_id;

pub fn broadcast_block(block: &Block, peers: &[&str]) {
    for peer in peers {
        let url = format!("{}/block/receive", peer);
        let client = reqwest::blocking::Client::new();
        let block_data = json!({
            "index": block.index,
            "timestamp": block.timestamp,
            "transactions": block.transactions,
            "previous_hash": block.previous_hash,
            "nonce": block.nonce,
            "hash": block.hash,
        });
        if let Err(e) = client.post(&url).json(&block_data).send() {
            eprintln!("Error broadcasting to {}: {}", peer, e);
        }
    }
}
