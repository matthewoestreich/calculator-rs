use super::error::ParserError;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone)]
pub enum Function {
    Abs,
    Floor,
    Ceil,
    Sin,
}

impl FromStr for Function {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "abs" => Self::Abs,
            "floor" => Self::Floor,
            "ceil" => Self::Ceil,
            "sin" => Self::Sin,
            _ => {
                return Err(ParserError::UnrecognizedFunction {
                    name: s.to_string(),
                });
            }
        })
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // All functions should be lower case!
        match self {
            Function::Abs => write!(f, "abs"),
            Function::Floor => write!(f, "floor"),
            Function::Ceil => write!(f, "ceil"),
            Function::Sin => write!(f, "sin"),
        }
    }
}
