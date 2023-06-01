use glass_pumpkin;
use hex::FromHexError;
use num_bigint::{BigInt, TryFromBigIntError};
use std::{error::Error, fmt};

#[derive(Clone, Debug)]

pub enum RSAError {
    PrimeNumberGenerationError(String),
    ConvertBigUIntToBigIntError(String),
    ConvertCipherToHexError(String),
    MessageSizeLimitExceededError(String),
}

impl From<glass_pumpkin::error::Error> for RSAError {
    fn from(err: glass_pumpkin::error::Error) -> RSAError {
        RSAError::PrimeNumberGenerationError(format!(
            "Failed to generate large prime number: {}",
            err
        ))
    }
}

impl From<TryFromBigIntError<BigInt>> for RSAError {
    fn from(err: TryFromBigIntError<BigInt>) -> RSAError {
        RSAError::ConvertBigUIntToBigIntError(format!(
            "Failed to convert Big UInt to BigInt: {}",
            err
        ))
    }
}

impl From<FromHexError> for RSAError {
    fn from(err: FromHexError) -> RSAError {
        RSAError::ConvertCipherToHexError(format!(
            "Failed to convert to ciphertext from hex for decoding: {}",
            err
        ))
    }
}

impl fmt::Display for RSAError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{:?}>", self)
    }
}

impl Error for RSAError {}
