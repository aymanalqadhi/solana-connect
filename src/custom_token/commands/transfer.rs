use solana_program::account_info::AccountInfo;

use crate::custom_token::error::CustomTokenError;
use crate::custom_token::serialize::RawSerializable;
use crate::custom_token::state::AccountState;

use crate::custom_token::error::CustomTokenResult;

/// 5% tax
pub const TAX_FACTOR: f64 = 0.05;

pub fn transfer_command(
    token_account: &AccountInfo,
    from_account: &AccountInfo,
    to_account: &AccountInfo,
    amount: u64,
) -> CustomTokenResult<()> {
    if !from_account.is_signer {
        return Err(CustomTokenError::UnsignedAccount(*from_account.key));
    }

    let _ = AccountState::get_token(token_account)?;
    let mut from = AccountState::get_account(from_account)?;
    let mut to = AccountState::get_account(to_account)?;

    if from.token_key != *token_account.key {
        return Err(CustomTokenError::InvalidAccountToken(*from_account.key));
    } else if to.token_key != *token_account.key {
        return Err(CustomTokenError::InvalidAccountToken(*to_account.key));
    }

    let tax = (amount as f64 * TAX_FACTOR).floor() as u64;

    if from.balance < amount + tax {
        return Err(CustomTokenError::InsufficientBalance(*from_account.key));
    }

    from.balance -= amount + tax;
    to.balance += amount;

    from.serialize_raw_into_account(from_account)?;
    to.serialize_raw_into_account(to_account)?;

    Ok(())
}
