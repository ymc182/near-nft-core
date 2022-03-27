use std::convert::TryInto;

use super::*;

#[near_bindgen]
impl Contract {
    pub fn add_to_whitelist(&mut self, account_id: AccountId, amount: u32) {
        self.assert_owner(env::predecessor_account_id());
        self.whitelist.insert(&account_id, &amount);
    }
    pub fn is_whitelisted(&self, account_id: AccountId) -> bool {
        self.whitelist.contains_key(&account_id)
    }
    pub fn apply_for_whitelist(&mut self) {
        let applicant = self.apply_whitelist.get(&env::predecessor_account_id());
        if let Some(_) = applicant {
            panic!("Account is already in the whitelist");
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
    pub fn raffle_whitelist(&mut self) -> AccountId {
        require!(self.apply_whitelist.len() > 0, "No applicants");
        let rand_array = [
            *env::random_seed().get(0).unwrap(),
            *env::random_seed().get(2).unwrap(),
            *env::random_seed().get(3).unwrap(),
        ];
        let len: u8 = self.apply_whitelist.len().try_into().unwrap();
        let rand = rand_array[0] + rand_array[1] + rand_array[2];
        return self
            .apply_whitelist
            .keys()
            .nth((rand % len) as usize)
            .unwrap()
            .clone();
    }
}
