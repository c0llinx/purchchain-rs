use crate::core::blockchain::Block;
use crate::core::validator::ValidatorSet;

pub fn create_block_with_poa(validator_set: &ValidatorSet, block: Block, validator_id: &str) -> Option<Block> {
    if validator_set.is_validator(&validator_id.to_string()) {
        // In a full implementation, the block would be signed by the validator.
        Some(block)
    } else {
        None
    }
}