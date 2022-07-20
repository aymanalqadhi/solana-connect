use borsh::{BorshDeserialize, BorshSerialize};

pub const CREATE_TOKEN_COMMAND: u8 = 0x01;
pub const CREATE_ACCOUNT_COMMAND: u8 = 0x02;
pub const MINT_COMMAND: u8 = 0x03;
pub const TRANSFER_COMMAND: u8 = 0x04;

#[derive(Copy, Clone, Debug, BorshSerialize, BorshDeserialize)]
pub enum Command {
    CreateToken,
    CretaeAccount,
    Mint(u64),
    Transfer(u64),
}
