mod number;
mod shunting_yard;
pub(crate) mod value;

pub use bigdecimal::BigDecimal;
pub use num_bigint::BigInt;
pub use number::Number;
pub use value::{Value, error::ValueError};

use bigdecimal::ParseBigDecimalError;
use std::{error, fmt};

pub fn parse_expression(expression: &str) -> Result<Value, CalculatorError> {
    shunting_yard::parse(expression)
}

#[derive(Debug, Clone)]
pub enum CalculatorError {
    ParseBigDecimal(ParseBigDecimalError),
    EmptyExpression,
    InvalidExpression,
    ValueError(ValueError),
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalculatorError::ParseBigDecimal(e) => write!(f, "error parsing BigDecimal : {e}"),
            CalculatorError::EmptyExpression => write!(f, "expression cannot be empty"),
            CalculatorError::InvalidExpression => {
                write!(f, "you may be missing a parenthesis or number somewhere")
            }
            CalculatorError::ValueError(ve) => write!(f, "value error : {ve}"),
        }
    }
}

impl From<ValueError> for CalculatorError {
    fn from(error: ValueError) -> Self {
        Self::ValueError(error)
    }
}

impl From<ParseBigDecimalError> for CalculatorError {
    fn from(value: ParseBigDecimalError) -> Self {
        Self::ParseBigDecimal(value)
    }
}

impl error::Error for CalculatorError {}

#[derive(Debug, Clone)]
pub enum ExpressionError {
    InvalidOrMissingParenthesis,
}

impl fmt::Display for ExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpressionError::InvalidOrMissingParenthesis => {
                write!(f, "Expression is invalid or missing a parenthesis")
            }
        }
    }
}

impl error::Error for ExpressionError {}
