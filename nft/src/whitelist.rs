use std::convert::TryInto;

use super::*;

#[near_bindgen]
impl Contract {
    pub fn add_to_whitelist(&mut self, account_id: AccountId, amount: u32) {
        self.assert_owner(env::predecessor_account_id());
        self.whitelist.insert(&account_id, &amount);
    }
    pub fn is_whitelisted(&self, account_id: AccountId) -> bool {
        self.whitelist.keys().any(|x| x == account_id)
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

        let len: u32 = self.apply_whitelist.len().try_into().unwrap();
        let rand = self.get_random_number(len);
        return self
            .apply_whitelist
            .keys()
            .nth((rand % len) as usize)
            .unwrap()
            .clone();
    }
    pub fn raffle_free_mint(&mut self) -> AccountId {
        require!(self.whitelist.len() > 0, "No Whitelist");

        let len: u32 = self.whitelist.len().try_into().unwrap();
        let rand = self.get_random_number(len);
        return self
            .whitelist
            .keys()
            .nth((rand % len) as usize)
            .unwrap()
            .clone();
    }
    pub fn get_random_number(&self, shift_amount: u32) -> u32 {
        let mut seed = env::random_seed();
        env::log_str(&format!("{:?}", seed));
        let seed_len = seed.len();
        let mut arr: [u8; 4] = Default::default();
        seed.rotate_left(shift_amount as usize % seed_len);
        arr.copy_from_slice(&seed[..4]);
        u32::from_le_bytes(arr)
    }
}
