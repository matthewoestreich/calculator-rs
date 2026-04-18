use crate::ast::error::ParserError;
use std::{fmt, str::FromStr};
use varienum::variants_vec;

#[variants_vec]
#[derive(Debug, Clone)]
pub enum Constant {
    PI,
}

impl FromStr for Constant {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "pi" => Self::PI,
            _ => {
                return Err(ParserError::UnrecognizedConstant {
                    name: s.to_string(),
                });
            }
        })
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            //
            // All constants should be lower case!
            //
            Constant::PI => write!(f, "pi"),
        }
    }
}
