use crate::{CalculatorError, Number};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use regex::Regex;
use std::str::FromStr;

/// Shunting-Yard algorithm.
/// Expects an infix string, which we convert to reverse polish notation and evaluate.
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
pub fn parse(infix: &str) -> Result<Number, CalculatorError> {
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
            t if t.parse::<BigInt>().is_ok() || t.parse::<BigDecimal>().is_ok() => output.push(t),
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
fn eval_rpn(rpn: &str) -> Result<Number, CalculatorError> {
    if rpn.is_empty() {
        return Err(CalculatorError::EmptyExpression);
    }

    let mut stack = vec![];
    let rpn = rpn.trim();
    let rpn_tokens: Vec<_> = rpn.split_whitespace().collect();

    for token in rpn_tokens {
        if let Ok(v) = Number::from_str(token) {
            stack.push(v);
            continue;
        }

        // Order matters here! 'b' must come before 'a'!
        let b = stack.pop().ok_or(CalculatorError::InvalidExpression)?;
        let a = stack.pop().ok_or(CalculatorError::InvalidExpression)?;

        match token {
            "+" => stack.push(&a + &b),
            "-" => stack.push(&a - &b),
            "*" | "x" => stack.push(&a * &b),
            "/" => stack.push(&a / &b),
            "^" => {
                let result = a.pow(b.to_i64().ok_or(CalculatorError::InvalidExponent {
                    exponent_str: b.to_string(),
                })?)?;
                stack.push(result);
            }
            _ => return Err(CalculatorError::InvalidExpression),
        };
    }

    stack
        .into_iter()
        .next()
        .ok_or(CalculatorError::InvalidExpression)
}

fn tokenize(expression: &str) -> Vec<&str> {
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
        let expected = Number::Int(1.into());
        let result = parse(expression).unwrap();
        assert_eq!(
            result, expected,
            "expression = {expression} : expected {expected} got {result}"
        );

        let expression = "3.1 + 2";
        let expected = Number::from_f64(5.1).unwrap();
        let result = parse(expression).unwrap();
        assert_eq!(
            result, expected,
            "expression = {expression} : expected {expected} got {result}"
        );
    }

    #[test]
    fn dec_leading_zero() {
        let i = "1 / 2";
        let e = Number::from_f64(0.5).unwrap();
        let r = parse(i).unwrap();
        println!("{r}");
        assert_eq!(r, e, "expected {e} got {r}");
    }

    #[test]
    fn float_return() {
        // Tests return when it should be a Float
        let expression = "2 / (1 - 56)";
        let expected = Number::from_f64(-0.03636363636).unwrap();
        let mut result = parse(expression).unwrap();
        result.set_scale(11);
        assert_eq!(
            result, expected,
            "expression = {expression} : expected {expected} got {result}"
        );
    }

    #[test]
    fn pow() {
        let i = "2 ^ 3";
        let e = Number::Int(8.into());
        let r = parse(i).unwrap();
        assert_eq!(r, e, "expected {e} got {r}");
    }

    #[test]
    fn very_large_int() {
        let r = parse("340282366920938463463374607431768211455 * 137").unwrap();
        let e = Number::from_str("46618684268168569494482321218152244969335").unwrap();
        assert_eq!(r, e, "expected {e} got {r}");
    }

    #[test]
    fn very_large_dec() {
        let r = parse("340282366920938463463374607431768211455 * 137.3367").unwrap();
        let e = Number::from_str("46733257341110849475130439448474521326131.8985").unwrap();
        assert_eq!(r, e, "expected {e} got {r}");
    }
}
