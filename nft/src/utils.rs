use super::*;

#[near_bindgen]
impl Contract {
    pub fn get_max_supply(&self) -> u128 {
        self.max_supply
    }
}
pub fn get_random_number(shift_amount: u128) -> u32 {
    let mut seed = env::random_seed();

    let seed_len = seed.len();
    let mut arr: [u8; 4] = Default::default();
    seed.rotate_left(shift_amount as usize % seed_len);
    arr.copy_from_slice(&seed[..4]);
    u32::from_le_bytes(arr)
}
