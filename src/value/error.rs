use crate::value::Value;
use std::{error, fmt};

#[derive(Debug, Clone)]
pub enum Error {
    Converting { from: Value, to: String },
    Parsing { value: String },
    ImproperlyFloat,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Converting { from, to } => write!(f, "cannot convert {:?} to {:?}", from, to),
            Error::Parsing { value } => write!(f, "'{value}' cannot be parsed as Value"),
            Error::ImproperlyFloat => write!(
                f,
                "attempted to perform an operation which only makes sense for integers, but value is currently a float"
            ),
        }
    }
}

impl error::Error for Error {}
