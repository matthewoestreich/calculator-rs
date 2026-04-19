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
                let mut x = stack.pop().ok_or(ParserError::InvalidExpression)?;
                match function {
                    Function::Abs => x.abs_assign(),
                    Function::Floor => x.floor_assign(),
                    Function::Ceil => x.ceil_assign(),
                    Function::Sin => x.sin_assign()?,
                    Function::Cos => x.cos_assign()?,
                    Function::Tan => x.tan_assign()?,
                    Function::Round => x.round_assign(0),
                    Function::Sinh => x.sinh_assign()?,
                    Function::Cosh => x.cosh_assign()?,
                    Function::Tanh => x.tanh_assign()?,
                    Function::Rad => x.rad_assign(64)?,
                };
                stack.push(x);
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
                    let mut lhs = stack.pop().ok_or(ParserError::InvalidExpression)?;
                    match binary {
                        Binary::Add => lhs += rhs,
                        Binary::Subtract => lhs -= rhs,
                        Binary::Multiply => lhs *= rhs,
                        Binary::Divide => lhs.try_div_assign(&rhs)?,
                        Binary::Exponentiation => lhs.pow_assign(rhs.to_i64_saturating())?,
                        Binary::Remainder => lhs %= rhs,
                        Binary::And => lhs &= rhs,
                        Binary::Or => lhs |= rhs,
                        Binary::Xor => lhs ^= rhs,
                        Binary::ShiftLeft => lhs <<= rhs,
                        Binary::ShiftRight => lhs >>= rhs,
                    };
                    stack.push(lhs);
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
    #[case::evaluate_round("round(pi)", "3.0")]
    #[case::evaluate_round("round((1*13)+(99/(16-3)))", "21.0")]
    #[case::evaluate_cos("cos(12)", "0.84385395873249210465")]
    #[case::evaluate_sinh("sinh(-27)", "-266024120300.89930834")]
    #[case::evaluate_cosh("cosh(2)", "3.76219569108363145956221347777374610829")]
    #[case::evaluate_tanh("tanh(-3.14)", "-0.99626020494583190099")]
    #[case::evaluate_deg_to_radians("rad(361145983.342101)", "6303186.4896722574781")]
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
