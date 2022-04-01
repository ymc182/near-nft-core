use super::*;

#[near_bindgen]
impl Contract {
    pub fn get_max_supply(&self) -> u128 {
        MAX_SUPPLY
    }
}
