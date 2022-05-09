use super::*;
use near_contract_standards::non_fungible_token::events;
use near_sdk::{require, ONE_NEAR};

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn free_mint(&mut self) -> Token {
        require!(
            env::signer_account_id() == self.tokens.owner_id,
            "Only owner can call this method"
        );

        return self.internal_nft_mint(env::signer_account_id());
    }
    #[payable]
    pub fn whitelist_nft_mint(&mut self) -> Token {
        require!(
            env::attached_deposit() >= self.wl_price * ONE_NEAR, //6.66 NEAR 6660000000000000000000000
            "Not enough attached deposit"
        );
        require!(
            self.pre_sale_active || self.sales_active,
            "Pre-sale is not active"
        );
        let whitelist_amount = self.whitelist.get(&env::predecessor_account_id());
        if let Some(amount) = whitelist_amount {
            let new_amount = amount - 1;
            if new_amount == 0 {
                self.whitelist.remove(&env::predecessor_account_id());
            } else {
                self.whitelist
                    .insert(&env::predecessor_account_id(), &new_amount);
            }

            return self.internal_nft_mint(env::predecessor_account_id());
        } else {
            panic!("Account Id is not whitelisted");
        }
    }
    #[payable]
    pub fn whitelist_nft_mint_multi(&mut self, amount: u128) -> Vec<Token> {
        require!(
            env::attached_deposit() >= self.wl_price * ONE_NEAR * amount, //6.66 NEAR 6660000000000000000000000
            "Not enough attached deposit"
        );
        require!(
            self.pre_sale_active || self.sales_active,
            "Pre-sale is not active"
        );
        //check max supply
        let supply: U128 = self.tokens.nft_total_supply();
        require!(
            supply.0 + amount <= self.max_supply,
            "NFT total supply has reached maximum"
        );
        let mut result: Vec<Token> = Vec::new();

        let whitelist_amount = self
            .whitelist
            .get(&env::predecessor_account_id())
            .expect("Account Id is not whitelisted");

        require!(
            whitelist_amount as u128 >= amount,
            "Not enough amount in whitelist"
        );
        for _ in 0..amount {
            result.push(self.internal_nft_mint(env::predecessor_account_id()));
        }

        let new_amount = whitelist_amount - amount as u32;
        if new_amount == 0 {
            self.whitelist.remove(&env::predecessor_account_id());
        } else {
            self.whitelist
                .insert(&env::predecessor_account_id(), &new_amount);
        }
        result
    }
    #[payable]
    pub fn nft_mint_multi(&mut self, amount: u128) -> Vec<Token> {
        let mut result: Vec<Token> = Vec::new();

        //check public sales active
        require!(self.sales_active, "Public sales is not active");

        //Check public mint limit
        let mint_limit = self
            .public_mint_limit
            .get(&env::signer_account_id())
            .unwrap_or(0);
        require!(mint_limit + amount as u32 <= 4, "Mint limit exceeded");

        //check max supply
        let supply: U128 = self.tokens.nft_total_supply();
        require!(
            supply.0 + amount <= self.max_supply,
            "NFT total supply has reached maximum"
        );
        require!(
            env::attached_deposit() >= (self.mint_price * ONE_NEAR) * amount, //mint price
            "Not enough attached deposit"
        );

        for _ in 0..amount {
            let token = self.internal_nft_mint(env::predecessor_account_id());
            result.push(token);
        }

        let new_mint_limit = mint_limit + amount as u32;

        self.public_mint_limit
            .insert(&env::signer_account_id(), &new_mint_limit);

        result
    }
    #[payable]
    pub fn nft_mint(&mut self) -> Token {
        require!(
            env::attached_deposit() >= self.mint_price * ONE_NEAR, //6.66 NEAR 6660000000000000000000000
            "Not enough attached deposit"
        );
        require!(self.sales_active, "Public sales is not active");

        self.internal_nft_mint(env::predecessor_account_id())
    }
    #[payable]
    fn internal_nft_mint(&mut self, receiver_id: AccountId) -> Token {
        let supply: U128 = self.tokens.nft_total_supply();

        require!(
            supply.0 < self.max_supply,
            "NFT total supply has reached maximum"
        );
        let offset = OFFSET as u64; //first number of next drop
        let token_id = (self.available_nft.draw() + offset + 1).to_string();

        /* let token_id = (supply.0 + 1).to_string(); */
        let token_metadata: TokenMetadata = TokenMetadata {
            copies: None,
            title: Some(format!("Smashrooms#{}", token_id)),
            media: Some(format!("{}.png", token_id)),
            description: Some(NFT_TOKEN_DESCRIPTION.to_string()),
            expires_at: None,
            extra: None,
            issued_at: Some(env::block_timestamp().to_string()),
            reference: Some(format!("{}.json", token_id)),
            reference_hash: None,
            starts_at: None,
            media_hash: None,
            updated_at: None,
        };

        let token = self.tokens.internal_mint_with_refund(
            token_id,
            receiver_id,
            Some(token_metadata),
            Some(self.tokens.owner_id.clone()),
        );
        let event = events::NftMint {
            owner_id: &token.owner_id,
            memo: None,
            token_ids: &[&token.token_id],
        };
        event.emit();

        token
    }
}
