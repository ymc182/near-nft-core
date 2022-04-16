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
}
