use solana_program::account_info::AccountInfo;

use crate::custom_token::error::*;
use crate::custom_token::serialize::RawSerializable;
use crate::custom_token::state::AccountState;

pub fn mint_command(
    token_account: &AccountInfo,
    to_account: &AccountInfo,
    amount: u64,
) -> CustomTokenResult<()> {
    if !token_account.is_signer {
        return Err(CustomTokenError::UnsignedAccount(*token_account.key));
    }

    let mut account = AccountState::get_account(to_account)?;
    let mut token = AccountState::get_token(token_account)?;

    if account.token_key != *token_account.key {
        return Err(CustomTokenError::InvalidAccountToken(*to_account.key));
    }

    account.balance += token.consume_supply(amount)?;

    token.serialize_raw_into_account(token_account)?;
    account.serialize_raw_into_account(to_account)?;

    Ok(())
}
