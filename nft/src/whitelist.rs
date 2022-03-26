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
        if let Some(account) = applicant {
            panic!("{} is already in the whitelist", account);
        }
        self.apply_whitelist
            .insert(&env::predecessor_account_id(), &false);
    }
}
