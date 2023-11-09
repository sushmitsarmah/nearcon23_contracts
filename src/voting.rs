use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{collections::LookupMap, near_bindgen, AccountId, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NFTVotingContract {
    // Maps an NFT to its like/dislike count
    votes: LookupMap<String, (u64, u64)>, // NFT ID to (likes, dislikes)
    // Maps an account and NFT to whether they liked/disliked it
    account_votes: LookupMap<(AccountId, String), bool>, // (Account ID, NFT ID) to liked (true) or disliked (false)
}

#[near_bindgen]
impl NFTVotingContract {
    #[init]
    pub fn new() -> Self {
        Self {
            votes: LookupMap::new(b"v".to_vec()),
            account_votes: LookupMap::new(b"a".to_vec()),
        }
    }

    // Function to like an NFT
    pub fn like(&mut self, nft_id: String) {
        // Logic to record a like for the NFT
        // Update both the `votes` and `account_votes`
    }

    // Function to dislike an NFT
    pub fn dislike(&mut self, nft_id: String) {
        // Logic to record a dislike for the NFT
        // Update both the `votes` and `account_votes`
    }

    // Function to get the number of likes and dislikes for an NFT
    pub fn get_votes(&self, nft_id: String) -> (u64, u64) {
        // Return the count of likes and dislikes
        self.votes.get(&nft_id).unwrap_or((0, 0))
    }

    // Function to get the accounts that liked or disliked an NFT
    // Returns a tuple of vectors containing account IDs
    pub fn get_voting_accounts(&self, nft_id: String) -> (Vec<AccountId>, Vec<AccountId>) {
        // Logic to return the accounts that liked and disliked
        // Iterate over `account_votes` to build the lists
    }
}
