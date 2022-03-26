use super::*;
use near_sdk::require;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn free_nft_mint(&mut self) -> Token {
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
    pub fn nft_mint(&mut self) -> Token {
        require!(
            env::attached_deposit() >= MINT_COST, //6.66 NEAR 6660000000000000000000000
            "deposit must be 5 NEAR"
        );
        self.internal_nft_mint(env::predecessor_account_id())
    }
    #[payable]
    pub fn internal_nft_mint(&mut self, receiver_id: AccountId) -> Token {
        let supply: U128 = self.tokens.nft_total_supply();
        require!(
            supply.0 < MAX_SUPPLY,
            "NFT total supply has reached maximum"
        );
        let token_id = (supply.0 + 1).to_string();
        let token_metadata: TokenMetadata = TokenMetadata {
            copies: None,
            title: Some(format!("{}", token_id)),
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
        self.tokens.internal_mint_with_refund(
            token_id,
            receiver_id,
            Some(token_metadata),
            Some(self.tokens.owner_id.clone()),
        )
    }
}
