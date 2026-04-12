use crate::{
    Number,
    ast::{Binary, Constant, Function, Operator, Token, Unary, error::ParserError},
};

/// Expects `rpn_tokens` in reverse polish notation
pub fn eval(rpn_tokens: Vec<Token>) -> Result<Number, ParserError> {
    if rpn_tokens.is_empty() {
        return Err(ParserError::EmptyExpression);
    }

    let mut stack = Vec::<Number>::new();

    for token in rpn_tokens {
        match token {
            Token::Number(n) => stack.push(n),
            Token::Constant(ref constant) => {
                stack.push(match constant {
                    Constant::PI => Number::pi(64)?,
                });
            }
            Token::Function(ref function) => {
                let x = stack.pop().ok_or(ParserError::InvalidExpression)?;
                stack.push(match function {
                    Function::Abs => x.abs(),
                    Function::Floor => x.floor(),
                    Function::Ceil => x.ceil(),
                    Function::Sin => x.sin()?,
                    Function::Tan => x.tan()?,
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
                        Binary::Divide => lhs.try_div(&rhs)?,
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
            // This should be unreachable! Keeping it in for exhaustive pattern matching.
            Token::ParenthesesOpen | Token::ParenthesesClose => {
                return Err(ParserError::UnexpectedToken(token));
            }
        }
    }

    // There MUST be only one element on the stack here.
    if stack.len() != 1 {
        return Err(ParserError::InvalidExpression);
    }
    Ok(stack.pop().expect("just verified len"))
}

#[cfg(test)]
mod test {
    use crate::{
        Number,
        ast::{eval, parse, tokenize},
    };
    use rstest::*;

    #[rstest]
    #[case::eval_sanity_check("2+2", "4")]
    #[case::evaluate6("!-!-(2+7)", "7")]
    #[case::evaluate1("!-(-3 + 4 * (2 - -5)) ^ 2 << 1 + !-6 * 3 - --7", "1048")]
    #[case::evaluate2("-2 ** 2", "4")]
    #[case::evaluate3("-2 ** 3 ** 2", "-512")]
    #[case::evaluate4("!-2 ** 2", "1")]
    #[case::evaluate5("-2 * 3 ** 2", "-18")]
    #[case::evaluate7("-(2+3) * 4", "-20")]
    #[case::evaluate8("(1 + 2) * (3 - 4) / 5", "-0.6")]
    #[case::evaluate9("(1 + 2) << (3 & 4)", "3")]
    #[case::evaluate10("1/2", "0.5")]
    #[case::evaluate_use_every_operator(
        "!-3 + 4 * (5 - 6) / 7 % 2 ** 3 << 1 >> 2 & 3 ^ 4 | 5",
        "5"
    )]
    #[case::evaluate_starts_with_dec(".5 + .5", "1.0")]
    #[case::evaluate_func("2 + abs((2+2)-10)", "8")]
    #[case::evaluate_nested_func("abs( 10 - abs( ( 2 + 2 ) - 10 ) )", "4")]
    #[case::evaluate_nested_func_with_neg("-abs( 10 - abs( -( 2 + 2 ) - 10 ) )", "-4")]
    #[case::evaluate11("!abs(-abs(2+3))", "-6")]
    #[case::evaluate_floor("1 + floor(11.5 + 10.2)", "22.0")]
    #[case::evaluate_ceil("2 - ceil((10 ** 2) / 33)", "-2.0")]
    #[case::evaluate_pi("pi", "3.1415926535897932383")]
    #[case::evaluate_pi("abs(-pi)", "3.1415926535897932383")]
    fn evaluate(#[case] raw_infix: &str, #[case] expect: &str) {
        let tokens = match tokenize(raw_infix) {
            Ok(t) => t,
            Err(e) => panic!("TOKENIZATION ERROR = {e:?}"),
        };
        let rpn_tokens = match parse(tokens.clone()) {
            Ok(t) => t,
            Err(e) => panic!("PARSER ERROR = {e:?}"),
        };
        let expected = expect.parse::<Number>().unwrap();
        let result = match eval(rpn_tokens) {
            Ok(r) => r,
            Err(e) => panic!("EVAL ERROR = {e:?}"),
        };
        assert_eq!(
            result, expected,
            "expression '{raw_infix}' | expected '{expected:?}' got '{result:?}'"
        );
    }
}
