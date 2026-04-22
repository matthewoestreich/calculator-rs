use astro_float::{Error as AstroError, Sign as AstroSign};
use bigdecimal::ParseBigDecimalError;
use num_bigint::ParseBigIntError;
use std::{error, fmt};

/// Error type for [`Number`](crate::Number).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumberError {
    Message(String),
    Parsing { value: String },
    InvalidExponent { message: String },
    DivisionByZero,
    IsNaNOrInfinity,
    ExponentOverflow(AstroSign),
    InvalidArgument,
    MemoryAllocation,
    UnsupportedRadix(u32),
}

impl fmt::Display for NumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumberError::Message(text) => write!(f, "{text}"),
            NumberError::IsNaNOrInfinity => {
                write!(f, "cannot represent NaN or Infinity as a Number")
            }
            NumberError::Parsing { value } => write!(f, "Error parsing value : {value}"),
            NumberError::InvalidExponent { message } => write!(f, "{message}"),
            NumberError::DivisionByZero => write!(f, "attempt to divide by zero"),
            NumberError::ExponentOverflow(sign) => write!(f, "exponent overflow '{sign:?}'"),
            NumberError::InvalidArgument => write!(f, "invalid argument"),
            NumberError::MemoryAllocation => write!(f, "memory allocation failed"),
            NumberError::UnsupportedRadix(radix) => write!(f, "unsupported radix : '{radix}'"),
        }
    }
}

impl From<ParseBigDecimalError> for NumberError {
    fn from(value: ParseBigDecimalError) -> Self {
        Self::Parsing {
            value: format!("[decimal] {value:?}"),
        }
    }
}

impl From<ParseBigIntError> for NumberError {
    fn from(value: ParseBigIntError) -> Self {
        Self::Parsing {
            value: format!("[int] {value:?}"),
        }
    }
}

impl From<AstroError> for NumberError {
    fn from(err: AstroError) -> Self {
        match err {
            AstroError::ExponentOverflow(sign) => Self::ExponentOverflow(sign),
            AstroError::DivisionByZero => Self::DivisionByZero,
            AstroError::InvalidArgument => Self::InvalidArgument,
            AstroError::MemoryAllocation => Self::MemoryAllocation,
        }
    }
}

impl From<NumberError> for String {
    fn from(number_error: NumberError) -> Self {
        number_error.to_string()
    }
}

impl error::Error for NumberError {}
