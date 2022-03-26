use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::json_types::U128;
use near_sdk::{
    env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,
};
use std::collections::HashMap;

mod mint;
mod royalty;
mod test;
mod whitelist;
use crate::royalty::Royalties;
const NFT_NAME: &str = "Nephilim";
const NFT_SYMBOL: &str = "Nep";
const NFT_BASE_URI: &str = "ipfs://nftFolder/";
//6660000000000000000000000
const MINT_COST: u128 = 6660000000000000000000000;
const MAX_SUPPLY: u128 = 1000;
const NFT_TOKEN_DESCRIPTION: &str = "Nephilim Token";
const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    //custom
    max_supply: u128,
    whitelist: LookupMap<AccountId, u32>,
    royalties: LazyOption<Royalties>,
    apply_whitelist: LookupMap<AccountId, bool>, //reserved
}
#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
    //Custom
    Whitelist,
    Royalties,
    WhitelistApplication,
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract owned by `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: NFT_NAME.to_string(),
                symbol: NFT_SYMBOL.to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                base_uri: Some(NFT_BASE_URI.to_string()),
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");

        metadata.assert_valid();
        let mut perpetual_royalties: HashMap<AccountId, u8> = HashMap::new();
        perpetual_royalties.insert(owner_id.clone(), 100);
        let royalties: Royalties = Royalties {
            accounts: perpetual_royalties,
            percent: 5,
        };
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id.clone(),
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            //custom
            max_supply: MAX_SUPPLY,
            whitelist: LookupMap::new(StorageKey::Whitelist.try_to_vec().unwrap()),
            royalties: LazyOption::new(StorageKey::Royalties, Some(&royalties)),
            apply_whitelist: LookupMap::new(StorageKey::WhitelistApplication.try_to_vec().unwrap()),
        }
    }
    #[init(ignore_state)]
    pub fn migrate(owner_id: AccountId) -> Self {
        let prev: Contract = env::state_read().expect("ERR_NOT_INITIALIZED");
        assert_eq!(
            prev.tokens.owner_id, owner_id,
            "Only owner can call this method"
        );
        let mut perpetual_royalties: HashMap<AccountId, u8> = HashMap::new();
        perpetual_royalties.insert(owner_id, 100);
        let royalties: Royalties = Royalties {
            accounts: perpetual_royalties,
            percent: 5,
        };
        let metadata = prev.metadata.get().unwrap();
        // let prev_base_uri = prev.metadata.get().unwrap().base_uri.unwrap();
        let this = Contract {
            tokens: prev.tokens,
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            max_supply: prev.max_supply,
            whitelist: prev.whitelist,
            royalties: LazyOption::new(StorageKey::Royalties, Some(&royalties)),
            apply_whitelist: prev.apply_whitelist,
        };

        this
    }
    pub fn assert_owner(&self, account_id: AccountId) {
        require!(
            self.tokens.owner_id == account_id,
            "Only owner can call this method"
        );
    }
}

near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
