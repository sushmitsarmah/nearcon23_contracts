use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, Balance, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct ImagePaymentContract {
    // Define the state of the contract
    image_cost: Balance,
}

#[near_bindgen]
impl ImagePaymentContract {
    #[init]
    pub fn new(image_cost: Balance) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self { image_cost }
    }

    // Function to receive payments and trigger image generation
    #[payable]
    pub fn pay_for_images(&mut self, num_images: u64) {
        let amount_paid = env::attached_deposit();
        let expected_amount = self.image_cost * num_images;
        
        assert!(
            amount_paid >= expected_amount,
            "Not enough deposit for the requested number of images."
        );

        // Logic to call the frontend or external API for image generation
        // This could be an asynchronous call using promises if needed
    }

    // Helper function to update the cost per image
    pub fn update_image_cost(&mut self, new_cost: Balance) {
        // Add security checks to ensure only the owner can update the cost
        self.image_cost = new_cost;
    }
}
