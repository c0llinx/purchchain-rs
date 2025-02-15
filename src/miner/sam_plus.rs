use crate::core::blockchain::{Blockchain, Block};
use crate::accounts::transactions::Transaction;

pub fn smart_auto_mine(blockchain: &mut Blockchain, pending_txs: Vec<Transaction>) -> Option<Block> {
    if pending_txs.is_empty() {
        return None;
    }
    let last_block = blockchain.get_last_block().clone();
    let mut new_block = Block::new(last_block.index + 1, pending_txs, last_block.hash.clone());
    new_block.hash = blockchain.proof_of_work(new_block.clone());
    Some(new_block)
}