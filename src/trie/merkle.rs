use std::collections::HashMap;

#[derive(Debug)]
pub struct MerkleTrie {
    pub nodes: HashMap<String, String>, // key-value pairs representing state
}

impl MerkleTrie {
    pub fn new() -> Self {
        MerkleTrie {
            nodes: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.nodes.insert(key, value);
    }

    pub fn get(&self, key: &String) -> Option<&String> {
        self.nodes.get(key)
    }

    /// Computes a simplified Merkle root by hashing concatenated keys and values.
    pub fn compute_root(&self) -> String {
        let mut concat = String::new();
        for (k, v) in &self.nodes {
            concat.push_str(k);
            concat.push_str(v);
        }
        crate::crypto::hash::compute_sha256(&concat)
    }
}