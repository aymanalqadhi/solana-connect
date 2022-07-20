use borsh::BorshDeserialize;
use byteorder::{BigEndian, ReadBytesExt};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
};

use std::io::Cursor;

use crate::custom_token::{
    command,
    commands::{
        create_account::create_account_command,
        create_token::create_token_command,
        mint::mint_command,
        transfer::{transfer_command, TAX},
    },
    token::Token,
};

pub fn process<'a>(
    accounts: &'a [AccountInfo],
    data: &'a [u8],
) -> ProgramResult {
    if data.len() < 1 {
        msg!("[ERROR] invaild request");
        return Err(ProgramError::InvalidInstructionData);
    }

    let mut accounts_iter = accounts.into_iter();

    let command = data[0];
    let payload = &data[1..];

    let result = match command {
        command::CREATE_TOKEN_COMMAND => {
            let token_account = next_account_info(&mut accounts_iter)?;
            let token = Token::try_from_slice(payload)?;

            msg!(
                "[INFO] creating a new token on account {} with {:?}",
                token_account.key,
                token
            );

            create_token_command(token_account, token)
        }
        command::CREATE_ACCOUNT_COMMAND => {
            let owner_account = next_account_info(&mut accounts_iter)?;
            let token_account = next_account_info(&mut accounts_iter)?;
            let new_account = next_account_info(&mut accounts_iter)?;

            msg!(
                "[INFO] creating a new account for {} with token {} by account {}",
                new_account.key,
                token_account.key,
                owner_account.key,
            );

            create_account_command(owner_account, token_account, new_account)
        }
        command::MINT_COMMAND => {
            let mut be_reader = Cursor::new(payload);

            let token_account = next_account_info(&mut accounts_iter)?;
            let to_account = next_account_info(&mut accounts_iter)?;
            let amount = be_reader.read_u64::<BigEndian>()?;

            msg!(
                "[INFO] minting {} tokens from token {} for account {}",
                amount,
                token_account.key,
                to_account.key,
            );

            mint_command(token_account, to_account, amount)
        }
        command::TRANSFER_COMMAND => {
            let mut be_reader = Cursor::new(payload);

            let token_account = next_account_info(&mut accounts_iter)?;
            let from_account = next_account_info(&mut accounts_iter)?;
            let to_account = next_account_info(&mut accounts_iter)?;
            let amount = be_reader.read_u64::<BigEndian>()?;

            msg!(
                "[INFO] transferring {} tokens of token {} from {} to {} with {}% tax",
                amount,
                token_account.key,
                from_account.key,
                to_account.key,
                TAX * 100f32,
            );

            transfer_command(token_account, from_account, to_account, amount)
        }
        _ => {
            msg!("[ERROR] invaild command");
            return Err(ProgramError::InvalidArgument);
        }
    };

    if let Err(err) = result {
        msg!("[ERROR] {}", err);
        return Err(err.into());
    }

    Ok(())
}
