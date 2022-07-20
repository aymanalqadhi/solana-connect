use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Copy, Clone, Debug, BorshSerialize, BorshDeserialize)]
pub struct Account {
    pub owner_key: Pubkey,
    pub token_key: Pubkey,
    pub balance: u64,
}
