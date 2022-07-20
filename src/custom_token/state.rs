use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;

use crate::custom_token::account::Account;
use crate::custom_token::error::CustomTokenResult;
use crate::custom_token::token::Token;

use super::error::CustomTokenError;

#[derive(Copy, Clone, Debug)]
pub enum AccountState {
    Empty,
    Token(Token),
    Account(Account),
}

impl AccountState {
    pub fn from_account(
        account: &AccountInfo,
    ) -> CustomTokenResult<AccountState> {
        let buf = &account.data.borrow();

        if buf.len() == 0 {
            return Ok(AccountState::Empty);
        }

        if let Ok(account) = Account::try_from_slice(&buf) {
            return Ok(AccountState::Account(account));
        }

        if let Ok(token) = Token::try_from_slice(&buf) {
            return Ok(AccountState::Token(token));
        }

        Err(CustomTokenError::InvalidAccountData(*account.key))
    }

    pub fn check_emptiness(account: &AccountInfo) -> CustomTokenResult<()> {
        return if account.data.borrow().len() == 0 {
            Ok(())
        } else {
            Err(CustomTokenError::AccountAlreadyExists(*account.key))
        };
    }

    pub fn get_account(account: &AccountInfo) -> CustomTokenResult<Account> {
        return match Self::from_account(account)? {
            AccountState::Account(account) => Ok(account),
            _ => Err(CustomTokenError::InvalidAccountData(*account.key)),
        };
    }

    pub fn get_token(account: &AccountInfo) -> CustomTokenResult<Token> {
        return match Self::from_account(account)? {
            AccountState::Token(token) => Ok(token),
            _ => Err(CustomTokenError::InvalidAccountData(*account.key)),
        };
    }
}
