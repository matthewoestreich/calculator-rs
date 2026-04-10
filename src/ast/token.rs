use super::{Function, Operator, error::ParserError};
use crate::Number;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Token {
    Number(Number),
    Operator(Operator),
    Function(Function),
    ParenthesesOpen,
    ParenthesesClose,
}

impl Token {
    /// Determines `&Token` precedence. We use "C-style" operator precedence.
    pub fn precedence(&self) -> i32 {
        if let Token::Operator(o) = self {
            return o.precedence();
        }
        0
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(number) => write!(f, "{number}"),
            Token::Function(func) => write!(f, "{func}"),
            Token::Operator(op_kind) => write!(f, "{op_kind}"),
            Token::ParenthesesOpen => write!(f, "("),
            Token::ParenthesesClose => write!(f, ")"),
        }
    }
}

impl TryFrom<&Token> for Number {
    type Error = ParserError;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match token {
            Token::Number(number) => Ok(number.clone()),
            _ => Err(ParserError::InvalidExpression),
        }
    }
}
