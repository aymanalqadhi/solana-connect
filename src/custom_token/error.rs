use derive_more::Display;
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

use super::token::Token;

#[derive(Copy, Clone, Debug, Display)]
pub enum CustomTokenError {
    #[display(fmt = "invalid account data of: {}", _0)]
    InvalidAccountData(Pubkey),

    #[display(fmt = "invalid account token of: {}", _0)]
    InvalidAccountToken(Pubkey),

    #[display(fmt = "account {} already exists", _0)]
    AccountAlreadyExists(Pubkey),

    #[display(fmt = "account {} is not signed", _0)]
    UnsignedAccount(Pubkey),

    #[display(fmt = "invalid token info: {:?}", _0)]
    InvalidTokenInfo(Token),

    #[display(fmt = "account has insufficient space")]
    InsufficientAccountDataSize,

    #[display(fmt = "serialization/deserialization error")]
    ConversionError,

    #[display(fmt = "token does not have enough supply of {}", _0)]
    InsufficientTokenSupply(u64),

    #[display(fmt = "account {} has an insufficient balance", _0)]
    InsufficientBalance(Pubkey),
}

pub type CustomTokenResult<T> = Result<T, CustomTokenError>;

impl From<CustomTokenError> for ProgramError {
    fn from(rhs: CustomTokenError) -> Self {
        match rhs {
            CustomTokenError::UnsignedAccount(_) => {
                ProgramError::MissingRequiredSignature
            }

            CustomTokenError::InvalidAccountData(_) => {
                ProgramError::InvalidAccountData
            }

            CustomTokenError::InvalidAccountToken(_) => {
                ProgramError::IllegalOwner
            }

            CustomTokenError::InvalidTokenInfo(_) => {
                ProgramError::InvalidArgument
            }

            CustomTokenError::InsufficientAccountDataSize => {
                ProgramError::MaxAccountsDataSizeExceeded
            }

            CustomTokenError::ConversionError => {
                ProgramError::BorshIoError("Borsh Error".into())
            }

            CustomTokenError::InsufficientTokenSupply(_)
            | CustomTokenError::InsufficientBalance(_) => {
                ProgramError::InvalidArgument
            }

            _ => unreachable!(),
        }
    }
}
