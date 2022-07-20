use solana_program::account_info::AccountInfo;

use crate::custom_token::account::Account;
use crate::custom_token::error::*;
use crate::custom_token::serialize::RawSerializable;
use crate::custom_token::state::AccountState;

pub fn create_account_command(
    owner_account: &AccountInfo,
    token_account: &AccountInfo,
    new_account: &AccountInfo,
) -> CustomTokenResult<()> {
    if !token_account.is_signer {
        return Err(CustomTokenError::UnsignedAccount(*token_account.key));
    }

    AccountState::check_emptiness(new_account)?;

    let mut token = AccountState::get_token(token_account)?;

    let new_account_state = Account {
        owner_key: *owner_account.key,
        token_key: *token_account.key,
        balance: token.consume_supply(token.initial_supply)?,
    };

    token.serialize_raw_into_account(&token_account)?;
    new_account_state.serialize_raw_into_account(&new_account)?;

    return Ok(());
}
