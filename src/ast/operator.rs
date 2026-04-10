use super::Token;
use std::{fmt, iter, str::Chars};

/// Has the ability to be an operator.
pub trait OperationOrder {
    fn precedence(&self) -> i32;
    fn associativity(&self) -> Associativity;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Associativity {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add,            // +
    Subtract,       // -
    Multiply,       // *
    Divide,         // /
    Exponentiation, // **
    Remainder,      // %
    And,            // &
    Or,             // |
    Xor,            // ^
    ShiftLeft,      // <<
    ShiftRight,     // >>
    Negate,         // -
    Not,            // !
}

impl OperationOrder for Operator {
    fn precedence(&self) -> i32 {
        match self {
            Operator::Negate | Operator::Not => 8,
            Operator::Exponentiation => 7,
            Operator::Multiply | Operator::Divide | Operator::Remainder => 6,
            Operator::Add | Operator::Subtract => 5,
            Operator::ShiftLeft | Operator::ShiftRight => 4,
            Operator::And => 3,
            Operator::Xor => 2,
            Operator::Or => 1,
        }
    }

    fn associativity(&self) -> Associativity {
        if matches!(self, Self::Exponentiation | Self::Negate | Self::Not) {
            return Associativity::Right;
        }
        Associativity::Left
    }
}

impl Operator {
    pub fn is_unary(&self) -> bool {
        matches!(self, Self::Negate | Self::Not)
    }

    /// Checks if an iter has two consecutive chars that qualify as an operator.
    /// Example of two-character operators : `**`, `<<`, `>>`
    pub(crate) fn has_two_chars(first_char: &char, iter: &mut iter::Peekable<Chars>) -> bool {
        iter.peek().is_some_and(|sec_char| {
            matches!((first_char, sec_char), ('*', '*') | ('<', '<') | ('>', '>'))
        })
    }

    /// Determines if an ambiguous operator (such as `-`) is considered
    /// unary or infix given the provided `tokens` context.
    pub(crate) fn is_unary_context(tokens_context: &[Token]) -> bool {
        tokens_context.is_empty()
            || matches!(
                tokens_context.last(),
                Some(Token::ParenthesesOpen | Token::Operator(_))
            )
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "ADD"),
            Operator::Subtract => write!(f, "SUB"),
            Operator::Multiply => write!(f, "MUL"),
            Operator::Divide => write!(f, "DIV"),
            Operator::Exponentiation => write!(f, "EXP"),
            Operator::Remainder => write!(f, "REM"),
            Operator::And => write!(f, "AND"),
            Operator::Or => write!(f, "OR"),
            Operator::Xor => write!(f, "XOR"),
            Operator::ShiftLeft => write!(f, "SHL"),
            Operator::ShiftRight => write!(f, "SHR"),
            Operator::Negate => write!(f, "NEG"),
            Operator::Not => write!(f, "NOT"),
        }
    }
}
