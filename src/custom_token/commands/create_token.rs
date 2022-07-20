use solana_program::account_info::AccountInfo;

use crate::custom_token::error::CustomTokenResult;
use crate::custom_token::serialize::RawSerializable;
use crate::custom_token::{
    error::CustomTokenError, state::AccountState, token::Token,
};

pub fn create_token_command(
    token_account: &AccountInfo,
    token: Token,
) -> CustomTokenResult<()> {
    if !token_account.is_signer {
        return Err(CustomTokenError::UnsignedAccount(*token_account.key));
    }

    if token.initial_supply > token.available_supply
        || token.available_supply != token.total_supply
    {
        return Err(CustomTokenError::InvalidTokenInfo(token));
    }

    AccountState::check_emptiness(token_account)?;
    token.serialize_raw_into_account(token_account)?;

    Err(CustomTokenError::AccountAlreadyExists(*token_account.key))
}
