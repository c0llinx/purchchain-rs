use std::collections::HashMap;

pub struct State {
    pub balances: HashMap<String, u64>,
}

impl State {
    pub fn new() -> Self {
        State {
            balances: HashMap::new(),
        }
    }

    /// Updates the balance for a given address. A negative amount will subtract from the balance.
    pub fn update_balance(&mut self, address: String, amount: i64) {
        let entry = self.balances.entry(address).or_insert(0);
        if amount < 0 {
            *entry = entry.saturating_sub(amount.abs() as u64);
        } else {
            *entry += amount as u64;
        }
    }
}
