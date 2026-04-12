use crate::ast::Token;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Associativity {
    Left,
    Right,
}

// ===========================================================================================
// ========================== Operator =======================================================
// ===========================================================================================

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Binary(Binary),
    Unary(Unary),
}

impl Operator {
    pub fn precedence(&self) -> i32 {
        match self {
            Operator::Unary(_) => 8,
            Operator::Binary(binary) => match binary {
                Binary::Exponentiation => 7,
                Binary::Multiply | Binary::Divide | Binary::Remainder => 6,
                Binary::Add | Binary::Subtract => 5,
                Binary::ShiftLeft | Binary::ShiftRight => 4,
                Binary::And => 3,
                Binary::Xor => 2,
                Binary::Or => 1,
            },
        }
    }

    pub fn associativity(&self) -> Associativity {
        match self {
            Operator::Unary(_) => Associativity::Right,
            Operator::Binary(binary) => match binary {
                Binary::Exponentiation => Associativity::Right,
                _ => Associativity::Left,
            },
        }
    }

    /// Return an array of tuple-chars containing operators with 2 characters.
    pub fn two_char_ops() -> [(char, char); 3] {
        [('<', '<'), ('>', '>'), ('*', '*')]
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
            Operator::Binary(binary) => write!(f, "{binary}"),
            Operator::Unary(unary) => write!(f, "{unary}"),
        }
    }
}

// ===========================================================================================
// ========================== Unary Operators ================================================
// ===========================================================================================

#[derive(Debug, Clone, Copy)]
pub enum Unary {
    Negate, // -
    Not,    // !
}

impl fmt::Display for Unary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unary::Negate => write!(f, "NEG"),
            Unary::Not => write!(f, "NOT"),
        }
    }
}

// ===========================================================================================
// ========================== Binary Operators ===============================================
// ===========================================================================================

#[derive(Debug, Clone, Copy)]
pub enum Binary {
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
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Binary::Add => write!(f, "ADD"),
            Binary::Subtract => write!(f, "SUB"),
            Binary::Multiply => write!(f, "MUL"),
            Binary::Divide => write!(f, "DIV"),
            Binary::Exponentiation => write!(f, "EXP"),
            Binary::Remainder => write!(f, "REM"),
            Binary::And => write!(f, "AND"),
            Binary::Or => write!(f, "OR"),
            Binary::Xor => write!(f, "XOR"),
            Binary::ShiftLeft => write!(f, "SHL"),
            Binary::ShiftRight => write!(f, "SHR"),
        }
    }
}
