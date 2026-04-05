use crate::{CalculatorError, Value};
use regex::Regex;
use std::str::FromStr;

// Shunting-Yard algorithm.
pub fn parse(infix: &str) -> Result<Value, CalculatorError> {
    let mut output = vec![];
    let mut stack = vec![];
    let tokens = tokenize(infix);

    for token in tokens {
        match token {
            "(" => stack.push(token),
            ")" => {
                while let Some(t) = stack.pop() {
                    if t == "(" {
                        break;
                    }
                    output.push(t);
                }
            }
            t if t.parse::<i128>().is_ok() || t.parse::<f64>().is_ok() => output.push(t),
            t => {
                while let Some(&top) = stack.last() {
                    if top == "(" || precedence(top) < precedence(t) {
                        break;
                    }
                    output.push(stack.pop().ok_or(CalculatorError::InvalidExpression)?);
                }
                stack.push(t);
            }
        }
    }

    while let Some(p) = stack.pop() {
        output.push(p);
    }

    eval_rpn(&output.join(" "))
}

/// Evaluates a reverse polish notation string.
/// Examples:
/// |-------------------|---------------------|
/// |  infix (regular)  |        rpn          |
/// |-------------------|---------------------|
/// | 3 + 4             | 3 4 +               |
/// | (5 + 6) * 3       | 5 6 + 3 *           |
/// | (4 + 8)(1 + 3)    | 4 8 + 1 3 + /       |
/// | (2 / 4) * (5 - 6) | 2 4 / 5 6 - *       |
/// | 2 ^ 3             | 2 3 ^               |
/// |-------------------|---------------------|
fn eval_rpn(rpn: &str) -> Result<Value, CalculatorError> {
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
            continue;
        }

        // Order matters here! 'b' must come before 'a'!
        let b = stack.pop().ok_or(CalculatorError::InvalidExpression)?;
        let a = stack.pop().ok_or(CalculatorError::InvalidExpression)?;

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

    stack
        .into_iter()
        .next()
        .ok_or(CalculatorError::InvalidExpression)
}

fn tokenize(expression: &str) -> Vec<&str> {
    // Matches numbers (\d+), variables ([a-zA-Z]+), or single operators/parentheses ([()+\-*/^])
    let re = Regex::new(r"\d+(?:\.\d+)?|[a-zA-Z]+|[()+\-*/^]").unwrap();
    re.find_iter(expression).map(|mat| mat.as_str()).collect()
}

// Higher precedence value means higher priority
fn precedence(op: &str) -> i32 {
    match op {
        "+" | "-" => 1,
        "*" | "x" | "/" => 2,
        "^" => 3,
        _ => 0,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eval() {
        let expression = "3 + 4 * 2 / (1 - 5)";
        let expected = Value::SignedInt(1);
        let result = parse(expression).unwrap();
        assert_eq!(
            result, expected,
            "expression = {expression:?} : expected {expected:?} got {result:?}"
        );

        let expression = "3.1 + 2";
        let expected = Value::Float(5.1);
        let result = parse(expression).unwrap();
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
        let result = parse(expression).unwrap();
        assert_eq!(
            result, expected,
            "expression = {expression:?} : expected {expected:?} got {result:?}"
        );
    }
}
