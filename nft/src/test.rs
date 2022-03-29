#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use std::convert::TryInto;

    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    use super::super::*;
    use crate::constants::MINT_COST;
    use crate::royalty::Payouts;
    use near_sdk::{ONE_NEAR, ONE_YOCTO};

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.nft_token("1".to_string()), None);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_mint() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_COST)
            .predecessor_account_id(accounts(0))
            .build());

        let token_id = "1".to_string();
        let token = contract.nft_mint();
        assert_eq!(token.token_id, token_id);
        assert_eq!(token.owner_id, accounts(0));
        assert_eq!(token.approved_account_ids.unwrap(), HashMap::new());
    }
    #[test]
    fn test_mint_multi() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_COST * 3)
            .predecessor_account_id(accounts(0))
            .build());

        let token = contract.nft_mint_multi(3);

        assert_eq!(token.len(), 3);
    }
    #[test]
    fn test_rand_num() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());
        let a = contract.get_random_number(5);
        println!("{}", a);
    }
    #[test]
    fn test_add_white_list() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(0))
            .build());
        contract.add_to_whitelist(accounts(1), 1);
        contract.apply_for_whitelist();
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(2))
            .build());
        contract.apply_for_whitelist();

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(1))
            .build());
        contract.apply_for_whitelist();
        assert!(contract.is_whitelisted(accounts(1)));
        let a = contract.raffle_whitelist();
        println!("Rand num:{}", a)
    }
}
