
use std::collections::HashSet;

pub struct Validator {
    pub id: String, // typically a public key or unique identifier
    pub active: bool,
}

pub struct ValidatorSet {
    pub validators: HashSet<String>, // stores validator IDs
}

impl ValidatorSet {
    pub fn new() -> Self {
        ValidatorSet {
            validators: HashSet::new(),
        }
    }

    /// Registers a new validator.
    pub fn register(&mut self, validator_id: String) {
        self.validators.insert(validator_id);
    }

    /// Checks if the given ID is a registered validator.
    pub fn is_validator(&self, validator_id: &String) -> bool {
        self.validators.contains(validator_id)
    }

    /// Replaces an inactive validator with a new one.
    pub fn replace_inactive(&mut self, inactive: &String, new_validator: String) {
        self.validators.remove(inactive);
        self.validators.insert(new_validator);
    }
}
