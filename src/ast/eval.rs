use super::{Function, Operator, Token, error::ParserError};
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
            Token::Function(ref f) => {
                let x = stack.pop().ok_or(ParserError::InvalidExpression)?;
                stack.push(match f {
                    Function::Abs => x.abs(),
                    Function::Floor => x.floor(),
                    Function::Ceil => x.ceil(),
                });
            }
            Token::Operator(ref o) => {
                let n = eval_operator(o, &mut stack)?;
                stack.push(n);
            }
            _ => return Err(ParserError::UnexpectedToken(token)),
        }
    }

    // There MUST be only one element on the stack here.
    if stack.len() != 1 {
        return Err(ParserError::InvalidExpression);
    }
    Ok(stack.pop().expect("just verified len"))
}

fn eval_operator(o: &Operator, stack: &mut Vec<Number>) -> Result<Number, ParserError> {
    if o.is_unary() {
        let x = stack.pop().ok_or(ParserError::InvalidExpression)?;
        Ok(match o {
            Operator::Negate => -x,
            Operator::Not => !x,
            _ => return Err(ParserError::ExpectedUnary(*o)),
        })
    } else {
        // Order matters here! 'rhs' must be popped before 'lhs'!
        let rhs = stack.pop().ok_or(ParserError::InvalidExpression)?;
        let lhs = stack.pop().ok_or(ParserError::InvalidExpression)?;
        Ok(match o {
            Operator::Add => lhs + rhs,
            Operator::Subtract => lhs - rhs,
            Operator::Multiply => lhs * rhs,
            Operator::Divide => lhs / rhs,
            Operator::Exponentiation => lhs.pow(rhs.to_i64_saturating())?,
            Operator::Remainder => lhs % rhs,
            Operator::And => lhs & rhs,
            Operator::Or => lhs | rhs,
            Operator::Xor => lhs ^ rhs,
            Operator::ShiftLeft => lhs << rhs,
            Operator::ShiftRight => lhs >> rhs,
            _ => unreachable!("unary operators should have been handled already"),
        })
    }
}
