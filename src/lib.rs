use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, StorageUsage};

pub mod ft_core;
pub mod events;
pub mod metadata;
pub mod storage;
pub mod internal;

use crate::metadata::*;
use crate::events::*;

/// The image URL for the default icon
const DATA_IMAGE_SVG_GT_ICON: &str = "data:image/jpeg;base64,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";

/// The specific version of the standard we're using
pub const FT_METADATA_SPEC: &str = "ft-1.0.0";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    /// Keep track of each account's balances
    pub accounts: LookupMap<AccountId, Balance>,

    /// Total supply of all tokens.
    pub total_supply: Balance,

    /// The bytes for the largest possible account ID that can be registered on the contract 
    pub bytes_for_longest_account_id: StorageUsage,

    /// Metadata for the contract itself
    pub metadata: LazyOption<FungibleTokenMetadata>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    Accounts,
    Metadata
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
        // Calls the other function "new: with some default metadata and the owner_id & total supply passed in 
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "HarvardTp Token".to_string(),
                symbol: "HTPC".to_string(),
                icon: Some(DATA_IMAGE_SVG_GT_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
        )
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(
        owner_id: AccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        // Create a variable of type Self with all the fields initialized. 
        let mut this = Self {
            // Set the total supply
            total_supply: total_supply.0,
            // Set the bytes for the longest account ID to 0 temporarily until it's calculated later
            bytes_for_longest_account_id: 0,
            // Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            accounts: LookupMap::new(StorageKey::Accounts.try_to_vec().unwrap()),
            metadata: LazyOption::new(
                StorageKey::Metadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
        };

        // Measure the bytes for the longest account ID and store it in the contract.
        this.measure_bytes_for_longest_account_id();

        // Register the owner's account and set their balance to the total supply.
        this.internal_register_account(&owner_id);
        this.internal_deposit(&owner_id, total_supply.into());
        
        // Emit an event showing that the FTs were minted
        FtMint {
            owner_id: &owner_id,
            amount: &total_supply,
            memo: Some("Initial token supply is minted"),
        }
        .emit();

        // Return the Contract object
        this
    }
}