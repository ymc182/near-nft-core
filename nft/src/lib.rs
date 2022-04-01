use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, UnorderedMap};
use near_sdk::json_types::U128;
use near_sdk::{
    env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,
};
use std::collections::HashMap;

mod constants;
mod mint;
mod royalty;
mod test;
mod utils;
mod whitelist;
use crate::royalty::Royalties;
use constants::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    royalties: LazyOption<Royalties>,
    //custom
    max_supply: u128,
    whitelist: UnorderedMap<AccountId, u32>,
    free_mint_list: UnorderedMap<AccountId, u32>,
    apply_whitelist: UnorderedMap<AccountId, bool>,
    //Sales Control
    sales_active: bool,
    pre_sale_active: bool,
}
#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
    Royalties,
    //Custom
    Whitelist,
    FreeMintList,
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
            sales_active: false,
            pre_sale_active: false,
            whitelist: UnorderedMap::new(StorageKey::Whitelist.try_to_vec().unwrap()),
            royalties: LazyOption::new(StorageKey::Royalties, Some(&royalties)),
            apply_whitelist: UnorderedMap::new(
                StorageKey::WhitelistApplication.try_to_vec().unwrap(),
            ),
            free_mint_list: UnorderedMap::new(StorageKey::FreeMintList.try_to_vec().unwrap()),
        }
    }
    #[init(ignore_state)]
    pub fn migrate(owner_id: AccountId) -> Self {
        let prev: Contract = env::state_read().expect("ERR_NOT_INITIALIZED");
        assert_eq!(
            prev.tokens.owner_id,
            env::predecessor_account_id(),
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
            max_supply: MAX_SUPPLY,
            whitelist: prev.whitelist,
            royalties: LazyOption::new(StorageKey::Royalties, Some(&royalties)),
            apply_whitelist: prev.apply_whitelist,
            sales_active: prev.sales_active,
            pre_sale_active: prev.pre_sale_active,
            free_mint_list: UnorderedMap::new(StorageKey::FreeMintList.try_to_vec().unwrap()),
        };

        this
    }

    pub fn update_uri(&mut self, uri: String) {
        self.assert_owner(env::predecessor_account_id());
        let new_metadata = NFTContractMetadata {
            spec: NFT_METADATA_SPEC.to_string(),
            name: NFT_NAME.to_string(),
            symbol: NFT_SYMBOL.to_string(),
            icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
            base_uri: Some(uri),
            reference: None,
            reference_hash: None,
        };
        self.metadata = LazyOption::new(
            StorageKey::Metadata.try_to_vec().unwrap(),
            Some(&new_metadata),
        )
    }
    pub fn assert_owner(&self, account_id: AccountId) {
        require!(
            self.tokens.owner_id == account_id,
            "Only owner can call this method"
        );
    }

    pub fn flip_public_sale(&mut self) {
        self.assert_owner(env::predecessor_account_id());
        self.sales_active = !self.sales_active;
    }
    pub fn flip_presale(&mut self) {
        self.assert_owner(env::predecessor_account_id());
        self.pre_sale_active = !self.pre_sale_active;
    }
    pub fn transfer_ownership(&mut self, account_id: AccountId) {
        self.assert_owner(env::predecessor_account_id());
        self.tokens.owner_id = account_id;
    }
    pub fn get_owner(&self) -> AccountId {
        return self.tokens.owner_id.clone();
    }
    pub fn get_sale_status(&self) -> bool {
        return self.sales_active;
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
