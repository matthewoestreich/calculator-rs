use astro_float::Error as AstroError;
use bigdecimal::ParseBigDecimalError;
use std::{error, fmt};

#[derive(Debug, Clone)]
pub enum NumberError {
    Parsing { value: String },
    InvalidExponent { message: String },
    ParseFloat(AstroError),
    DivisionByZero,
}

impl fmt::Display for NumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumberError::ParseFloat(e) => write!(f, "{e}"),
            NumberError::Parsing { value } => write!(f, "Error parsing value : {value}"),
            NumberError::InvalidExponent { message } => write!(f, "{message}"),
            NumberError::DivisionByZero => write!(f, "attempt to divide by zero"),
        }
    }
}

impl From<ParseBigDecimalError> for NumberError {
    fn from(value: ParseBigDecimalError) -> Self {
        Self::Parsing {
            value: value.to_string(),
        }
    }
}

impl From<AstroError> for NumberError {
    fn from(err: AstroError) -> Self {
        Self::ParseFloat(err)
    }
}

impl error::Error for NumberError {}
