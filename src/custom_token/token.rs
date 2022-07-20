use crate::custom_token::error::CustomTokenError;
use borsh::{BorshDeserialize, BorshSerialize};

use crate::custom_token::error::CustomTokenResult;

#[derive(Copy, Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct Token {
    pub total_supply: u64,
    pub available_supply: u64,
    pub initial_supply: u64,
}

impl Token {
    pub fn consume_supply(&mut self, amount: u64) -> CustomTokenResult<u64> {
        if self.available_supply < amount {
            return Err(CustomTokenError::InsufficientTokenSupply(amount));
        }

        self.available_supply -= amount;

        Ok(amount)
    }
}
