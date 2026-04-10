use super::{Binary, Function, Operator, Token, Unary, error::ParserError};
use crate::Number;

/// Expects `rpn_tokens` in reverse polish notation
pub fn eval(rpn_tokens: Vec<Token>) -> Result<Number, ParserError> {
    if rpn_tokens.is_empty() {
        return Err(ParserError::EmptyExpression);
    }

    let mut stack = Vec::<Number>::new();

    for token in rpn_tokens {
        match token {
            Token::Number(n) => stack.push(n),
            Token::Function(ref function) => {
                let x = stack.pop().ok_or(ParserError::InvalidExpression)?;
                stack.push(match function {
                    Function::Abs => x.abs(),
                    Function::Floor => x.floor(),
                    Function::Ceil => x.ceil(),
                });
            }
            Token::Operator(ref operator) => match operator {
                Operator::Unary(unary) => {
                    let x = stack.pop().ok_or(ParserError::InvalidExpression)?;
                    stack.push(match unary {
                        Unary::Negate => -x,
                        Unary::Not => !x,
                    });
                }
                Operator::Binary(binary) => {
                    // Order matters here! 'rhs' must be popped before 'lhs'!
                    let rhs = stack.pop().ok_or(ParserError::InvalidExpression)?;
                    let lhs = stack.pop().ok_or(ParserError::InvalidExpression)?;
                    stack.push(match binary {
                        Binary::Add => lhs + rhs,
                        Binary::Subtract => lhs - rhs,
                        Binary::Multiply => lhs * rhs,
                        Binary::Divide => lhs / rhs,
                        Binary::Exponentiation => lhs.pow(rhs.to_i64_saturating())?,
                        Binary::Remainder => lhs % rhs,
                        Binary::And => lhs & rhs,
                        Binary::Or => lhs | rhs,
                        Binary::Xor => lhs ^ rhs,
                        Binary::ShiftLeft => lhs << rhs,
                        Binary::ShiftRight => lhs >> rhs,
                    });
                }
            },
            _ => return Err(ParserError::UnexpectedToken(token)),
        }
    }

    // There MUST be only one element on the stack here.
    if stack.len() != 1 {
        return Err(ParserError::InvalidExpression);
    }
    Ok(stack.pop().expect("just verified len"))
}
