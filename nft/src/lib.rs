use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    assert_one_yocto, env, near_bindgen, AccountId, Balance, PanicOnDefault,
    PromiseOrValue, ext_contract
};
use std::collections::HashMap;

// Metadata for the NFT
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Metadata {
    title: String,
    description: String,
    media: String, // URL to the media, IPFS link in this case
}

// Structure for the NFT
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NFT {
    owner_id: AccountId,
    metadata: Metadata,
    approved_account_ids: HashMap<AccountId, u64>, // The u64 could represent an approval_id

                                                   // Add more fields as necessary
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NFTCreationContract {
    nfts: UnorderedMap<u64, NFT>, // Stores the NFTs by a unique ID
    next_nft_id: u64,             // Keeps track of the next ID to assign to an NFT
}

#[near_bindgen]
impl NFTCreationContract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            nfts: UnorderedMap::new(b"nfts"),
            next_nft_id: 0,
        }
    }

    // Function to create an NFT given an IPFS link and other metadata
    pub fn create_nft(&mut self, title: String, description: String, media: String) -> u64 {
        // Ensure that the function call is attached with a minimum amount of NEAR tokens
        assert_one_yocto();

        let owner_id = env::predecessor_account_id();
        let nft_id = self.next_nft_id;

        let metadata = Metadata {
            title,
            description,
            media,
        };

        let nft = NFT {
            owner_id: owner_id.clone(),
            metadata,
        };

        // Insert the NFT into the map and increment the ID counter
        assert!(
            self.nfts.insert(&nft_id, &nft).is_none(),
            "NFT already exists"
        );
        self.next_nft_id += 1;

        // You can emit an event (log) that the NFT was created
        env::log_str(&format!("NFT {} created for account {}", nft_id, owner_id));

        nft_id
    }

    // Transfer an NFT to another account
    pub fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: u64,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) {
        // Ensure that the function call is attached with a minimum amount of NEAR tokens
        assert_one_yocto();

        // Get the NFT and check the ownership and approvals
        let mut nft = self.nfts.get(&token_id).expect("No NFT with this ID");
        let sender_id = env::predecessor_account_id();

        // Make sure the sender has permission to send the token
        // This would involve checking approval IDs if they're implemented
        assert_eq!(
            &sender_id, &nft.owner_id,
            "Sender must be the owner of the NFT"
        );

        // Transfer the NFT to the receiver
        nft.owner_id = receiver_id.clone();
        self.nfts.insert(&token_id, &nft);

        // Log the transfer
        env::log_str(&format!(
            "NFT {} transferred from {} to {} with memo: {:?}",
            token_id, sender_id, receiver_id, memo
        ));
    }

    // Query metadata for a specific NFT
    pub fn nft_token(&self, token_id: u64) -> Option<JsonToken> {
        self.nfts.get(&token_id).map(|nft| JsonToken {
            token_id: token_id.to_string(),
            owner_id: nft.owner_id,
            metadata: nft.metadata,
            // Add more fields as per the Metadata standard in NEP-177
        })
    }

    // Approves an account to transfer a token on behalf of the owner
    pub fn nft_approve(&mut self, token_id: u64, account_id: AccountId, msg: Option<String>) {
        let account_id: AccountId = account_id.into();
        let mut nft = self.nfts.get(&token_id).expect("No NFT with this ID");
        let owner_id = env::predecessor_account_id();

        // Ensure the caller is the owner of the NFT
        assert_eq!(
            owner_id, nft.owner_id,
            "Only the owner can approve transfers"
        );

        // Create an approval_id which could be a timestamp or a hash of some sort
        let approval_id = env::block_timestamp();

        // Approve the account to transfer the token
        nft.approved_account_ids
            .insert(account_id.clone(), approval_id);
        self.nfts.insert(&token_id, &nft);

        // Implement any logic needed for the `msg` argument
        // ...

        // Log the approval
        env::log_str(&format!(
            "Account {} approved to transfer NFT {}",
            account_id, token_id
        ));
    }

    // Revokes the approval for an account to transfer a token on behalf of the owner
    pub fn nft_revoke(&mut self, token_id: u64, account_id: ValidAccountId) {
        let account_id: AccountId = account_id.into();
        let mut nft = self.nfts.get(&token_id).expect("No NFT with this ID");
        let owner_id = env::predecessor_account_id();

        // Ensure the caller is the owner of the NFT
        assert_eq!(
            owner_id, nft.owner_id,
            "Only the owner can revoke transfers"
        );

        // Remove the account from the list of approved accounts
        if nft.approved_account_ids.remove(&account_id).is_none() {
            env::panic_str("Account was not approved");
        }
        self.nfts.insert(&token_id, &nft);

        // Log the revocation
        env::log_str(&format!(
            "Account {} approval to transfer NFT {} revoked",
            account_id, token_id
        ));
    }

    // Revokes all approvals for a token
    pub fn nft_revoke_all(&mut self, token_id: u64) {
        let mut nft = self.nfts.get(&token_id).expect("No NFT with this ID");
        let owner_id = env::predecessor_account_id();

        // Ensure the caller is the owner of the NFT
        assert_eq!(
            owner_id, nft.owner_id,
            "Only the owner can revoke all transfers"
        );

        // Clear all approvals
        nft.approved_account_ids.clear();
        self.nfts.insert(&token_id, &nft);

        // Log the full revocation
        env::log_str(&format!(
            "All transfer approvals for NFT {} have been revoked",
            token_id
        ));
    }

    // Transfers an NFT to another account and calls a method on the receiver's contract
    pub fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: u64,
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool> {
        // First, perform the transfer like in the nft_transfer method
        // ...

        // Call the method on the receiver's contract
        // The method should be "nft_on_transfer" and the contract must return a boolean indicating whether the transfer is accepted
        ext_receiver::nft_on_transfer(
            env::predecessor_account_id(), // sender_id
            receiver_id.to_string(),       // receiver_id
            token_id.to_string(),          // token_id
            msg,                           // message to pass to the receiver's contract
            &receiver_id,                  // receiver's account ID (the contract)
            0,                             // attached deposit to be forwarded
            env::prepaid_gas() / 2,        // forward half the remaining gas
        )
        // Then we need to resolve the transfer based on the promise result
        .then(Self::ext(env::current_account_id()).resolve_transfer(
            env::predecessor_account_id(), // sender_id
            receiver_id.to_string(),       // receiver_id
            token_id,                      // token_id
            &env::current_account_id(),    // this contract's account ID
            0,                             // no attached deposit
            env::prepaid_gas() / 2,        // forward the remaining half of the gas
        ))
        .into()
    }

    // Method to resolve the transfer after the receiver contract has executed `nft_on_transfer`
    pub fn resolve_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        token_id: u64,
    ) -> bool {
        // This method should check the execution outcome of the nft_on_transfer call
        // and handle the token state accordingly (e.g., revert the transfer if the receiver rejected it)
        // For simplicity, let's assume the transfer always succeeds
        true
    }

    // Implement other necessary methods such as transfer, and metadata standards
    // ...
}

// A helper struct to represent the NFT metadata in a way that's compliant with NEP-171
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    token_id: String,
    owner_id: AccountId,
    metadata: Metadata,
    // Include other fields like `approved_account_ids` if your contract implements approvals
}

// Trait to define the external receiver interface
#[ext_contract(ext_receiver)]
trait ExtReceiver {
    fn nft_on_transfer(
        &mut self,
        sender_id: String,
        receiver_id: String,
        token_id: String,
        msg: String,
    ) -> PromiseOrValue<bool>;
}

// Trait to define the callbacks interface on this contract
#[ext_contract(Self)]
trait NFTCallbacks {
    fn resolve_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        token_id: u64,
    ) -> bool;
}

// Add more methods here, like transfer, and other view methods to get NFT data
// ...

// Implement the NEP-171 interface and methods
// ...
