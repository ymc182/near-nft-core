use std::convert::TryInto;

use super::*;

#[near_bindgen]
impl Contract {
    pub fn add_to_whitelist(&mut self, account_id: AccountId, amount: u32) {
        self.assert_owner(env::predecessor_account_id());
        if let Some(_is_true) = self.apply_whitelist.get(&account_id) {
            self.apply_whitelist.remove(&account_id);
        }
        self.whitelist.insert(&account_id, &amount);
    }
    pub fn is_whitelisted(&self, account_id: AccountId) -> bool {
        self.whitelist.keys().any(|x| x == account_id)
    }
    pub fn apply_for_whitelist(&mut self) {
        let applicant = self.apply_whitelist.get(&env::predecessor_account_id());
        if let Some(_) = applicant {
            panic!("Account is already applied for whitelist");
        }
        self.apply_whitelist
            .insert(&env::predecessor_account_id(), &false);
    }
    pub fn get_applied_id(&self) -> Vec<AccountId> {
        let mut result: Vec<AccountId> = Vec::new();
        for (key, _value) in self.apply_whitelist.iter() {
            result.push(key.clone());
        }
        result
    }
    pub fn raffle_whitelist(&self) -> AccountId {
        self.assert_owner(env::predecessor_account_id());
        require!(self.apply_whitelist.len() > 0, "No applicants");

        let len: u32 = self.apply_whitelist.len().try_into().unwrap();
        let rand = get_random_number(len.into());
        let result = self
            .apply_whitelist
            .keys()
            .nth((rand % len) as usize)
            .unwrap()
            .clone();
        env::log_str(
            format!(
                "Raffle Whitelist Result : {}, Block Height: {}, Participants: {}",
                result.clone(),
                env::block_height(),
                self.apply_whitelist.len()
            )
            .as_str(),
        );
        result
    }
    pub fn raffle_free_mint(&self) -> AccountId {
        self.assert_owner(env::predecessor_account_id());
        require!(self.whitelist.len() > 0, "No Whitelist");
        let len: u32 = self.whitelist.len().try_into().unwrap();
        let rand = get_random_number(len.into());
        let result = self
            .whitelist
            .keys()
            .nth((rand % len) as usize)
            .unwrap()
            .clone();

        env::log_str(
            format!(
                "Raffle Free Mint Result : {}, Block Height: {}, Participants: {}",
                result.clone(),
                env::block_height(),
                self.whitelist.len()
            )
            .as_str(),
        );
        result
    }
}
