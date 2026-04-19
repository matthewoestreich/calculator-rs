use crate::ast::error::ParserError;
use std::{fmt, str::FromStr};
use varienum::variants_vec;

///
/// -- Important info --
///
/// `round` : We round to nearest integer (0 decimal places); e.g., `12345.9448820304` -> `12346` and `12345.4448820304` -> `12345`.
///           Whole numbers are just returned as is; e.g., `12345` -> `12345` and `69420` -> `69420`.
///           Rounding mode is half even; round to ‘nearest neighbor’, if equidistant, round towards nearest even digit.
///

#[variants_vec]
#[derive(Debug, Clone)]
pub enum Function {
    Abs,
    Floor,
    Ceil,
    Sin,
    Cos,
    Tan,
    Round,
    Sinh,
    Cosh,
    Tanh,
}

impl FromStr for Function {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "abs" => Self::Abs,
            "floor" => Self::Floor,
            "ceil" => Self::Ceil,
            "sin" => Self::Sin,
            "cos" => Self::Cos,
            "tan" => Self::Tan,
            "round" => Self::Round,
            "sinh" => Self::Sinh,
            "cosh" => Self::Cosh,
            "tanh" => Self::Tanh,
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
        match self {
            //
            // All functions should be lower case!
            //
            Function::Abs => write!(f, "abs"),
            Function::Floor => write!(f, "floor"),
            Function::Ceil => write!(f, "ceil"),
            Function::Sin => write!(f, "sin"),
            Function::Cos => write!(f, "cos"),
            Function::Tan => write!(f, "tan"),
            Function::Round => write!(f, "round"),
            Function::Sinh => write!(f, "sinh"),
            Function::Cosh => write!(f, "cosh"),
            Function::Tanh => write!(f, "tanh"),
        }
    }
}
