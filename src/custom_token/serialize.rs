use crate::custom_token::error::CustomTokenError;
use borsh::BorshSerialize;
use solana_program::account_info::AccountInfo;

use super::error::CustomTokenResult;

pub trait RawSerializable
where
    Self: BorshSerialize,
{
    fn serialize_raw_into_slice(
        &self,
        slice: &mut [u8],
    ) -> CustomTokenResult<()>;

    fn serialize_raw_into_account(
        &self,
        account: &AccountInfo,
    ) -> CustomTokenResult<()>;
}

impl<T> RawSerializable for T
where
    T: BorshSerialize,
{
    fn serialize_raw_into_slice(
        &self,
        slice: &mut [u8],
    ) -> CustomTokenResult<()> {
        let serialized = match self.try_to_vec() {
            Ok(vec) => vec,
            Err(_) => return Err(CustomTokenError::ConversionError),
        };

        if serialized.len() < slice.len() {
            slice.clone_from_slice(serialized.as_slice());
            return Ok(());
        }

        Err(CustomTokenError::InsufficientAccountDataSize)
    }

    fn serialize_raw_into_account(
        &self,
        account: &AccountInfo,
    ) -> CustomTokenResult<()> {
        Self::serialize_raw_into_slice(&self, &mut account.data.borrow_mut())
    }
}
