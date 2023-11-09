// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::log_str;
use near_sdk::near_bindgen;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract2 {
    greeting: String,
}

// Define the default, which automatically initializes the contract
impl Default for Contract2 {
    fn default() -> Self {
        Self { greeting: "Hello".to_string() }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract2 {
    // Public method - returns the greeting saved, defaulting to DEFAULT_GREETING
    pub fn get_greeting2(&self) -> String {
        return self.greeting.clone();
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_greeting2(&mut self, greeting: String) {
        log_str(&format!("Saving greeting: {greeting}"));
        self.greeting = greeting;
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract2::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            contract.get_greeting2(),
            "Hello".to_string()
        );
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract2::default();
        contract.set_greeting2("howdy".to_string());
        assert_eq!(
            contract.get_greeting2(),
            "howdy".to_string()
        );
    }
}
