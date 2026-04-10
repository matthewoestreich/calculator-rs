use super::{Binary, Token};
use crate::NumberError;
use bigdecimal::ParseBigDecimalError;
use std::{error, fmt};

#[derive(Debug, Clone)]
pub enum ParserError {
    EmptyExpression,
    InvalidExpression,
    UnrecognizedFunction {
        name: String,
    },
    /// `Operator` argument is what you got instead
    ExpectedUnary(Binary),
    /// `Token` argument is what you got instead
    ExpectedFunction(Token),
    /// `Token` argument is what you got instead
    ExpectedOperator(Token),
    /// `Token` is what you got, not what you expected
    UnexpectedToken(Token),
    UnexpectedChar(char),
    InvalidExponent {
        exponent_str: String,
    },
    InvalidNumber(String),
    NumberErr(NumberError),
    MissingClosingParentheses,
    MissingOpeningParentheses,
    BigDecimalErr(ParseBigDecimalError),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::EmptyExpression => write!(f, "expression cannot be empty"),
            ParserError::InvalidExpression => write!(f, "expression is invalid"),
            ParserError::ExpectedUnary(got) => {
                write!(f, "expected valid unary operator, got '{got}'")
            }
            ParserError::ExpectedFunction(got) => write!(f, "expected function, got '{got}'"),
            ParserError::ExpectedOperator(got) => write!(f, "expected operator, got '{got}'"),
            ParserError::UnrecognizedFunction { name } => {
                write!(f, "function with name '{name}' is not recognized")
            }
            ParserError::InvalidNumber(n_str) => write!(f, "invalid number : '{n_str}'"),
            ParserError::BigDecimalErr(e) => write!(f, "error parsing BigDecimal : {e}"),
            ParserError::NumberErr(ne) => write!(f, "{ne}"),
            ParserError::MissingOpeningParentheses => {
                write!(f, "expression missing opening parentheses")
            }
            ParserError::MissingClosingParentheses => {
                write!(f, "expression missing closing parentheses")
            }
            ParserError::UnexpectedChar(c) => write!(f, "unexpected char '{c}'"),
            ParserError::UnexpectedToken(got) => {
                write!(f, "got '{got}' and did not expect to")
            }
            ParserError::InvalidExponent { exponent_str } => write!(
                f,
                "{exponent_str} : is either Number::Decimal(x) or is unable to be represented by an i64 (eg. it is a float, etc..)"
            ),
        }
    }
}

impl error::Error for ParserError {}

impl From<NumberError> for ParserError {
    fn from(error: NumberError) -> Self {
        Self::NumberErr(error)
    }
}

impl From<ParseBigDecimalError> for ParserError {
    fn from(value: ParseBigDecimalError) -> Self {
        Self::BigDecimalErr(value)
    }
}
