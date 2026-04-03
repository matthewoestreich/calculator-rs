use std::{error, fmt, str::FromStr};

mod shunting_yard;
pub(crate) mod value;

pub use value::{Value, error::ValueError};

pub fn parse_expression(expression: &str) -> Result<Value, CalculatorError> {
    let sy = shunting_yard::parse(expression).ok_or(CalculatorError::InvalidExpression)?;
    println!("reverse_polish_notation = {sy:?}");
    eval_shunting_yard(&sy)
}

fn eval_shunting_yard(rpn: &str) -> Result<Value, CalculatorError> {
    if rpn.is_empty() {
        return Err(CalculatorError::EmptyExpression);
    }

    let rpn = rpn.trim();
    let rpn_tokens: Vec<_> = rpn.split_whitespace().collect();
    println!("rpn_tokens = {rpn_tokens:?}");
    let mut stack = vec![];

    for token in rpn_tokens {
        println!("eval token : {token:?}");
        if let Ok(v) = Value::from_str(token) {
            stack.push(v);
        } else {
            // Order matters here! 'b' must come before 'a'!
            let b = stack.pop().expect("stack not empty");
            let a = stack.pop().expect("stack not empty");

            match token {
                "+" => {
                    let result = &a + &b;
                    println!("add : a = {a:?} + b = {b:?} = {result:?}");
                    stack.push(result);
                }
                "-" => {
                    let result = &a - &b;
                    println!("sub : a = {a:?} - b = {b:?} = {result:?}");
                    stack.push(result);
                }
                "*" | "x" => {
                    let result = &a * &b;
                    println!("mul : a = {a:?} * b = {b:?} = {result:?}");
                    stack.push(result);
                }
                "/" => {
                    let result = &a / &b;
                    println!("div : a = {a:?} / b = {b:?} = {result:?}");
                    stack.push(result);
                }

                "^" => stack.push(a.pow(b)?),
                _ => {}
            };
        }
    }

    stack
        .into_iter()
        .next()
        .ok_or(CalculatorError::InvalidExpression)
}

#[derive(Debug, Clone)]
pub enum CalculatorError {
    EmptyExpression,
    InvalidExpression,
    ValueError(ValueError),
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eval() {
        let expression = "3 + 4 * 2 / (1 - 5)";
        let expected = Value::SignedInt(1);
        let result = parse_expression(expression).unwrap();
        assert_eq!(
            result, expected,
            "expression = {expression:?} : expected {expected:?} got {result:?}"
        );

        let expression = "3.1 + 2";
        let expected = Value::Float(5.1);
        let result = parse_expression(expression).unwrap();
        assert_eq!(
            result, expected,
            "expression = {expression:?} : expected {expected:?} got {result:?}"
        );
    }

    #[test]
    fn float_return() {
        // Tests return when it should be a Float
        let expression = "2 / (1 - 56)";
        let expected = Value::Float(-0.03636363636);
        let result = parse_expression(expression).unwrap();
        assert_eq!(
            result, expected,
            "expression = {expression:?} : expected {expected:?} got {result:?}"
        );
    }
}
