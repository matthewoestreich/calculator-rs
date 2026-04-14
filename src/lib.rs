mod ast;
mod calculator;
mod number;

pub use bigdecimal;
pub use calculator::*;
pub use num_bigint;
pub use number::{Formatting, Number, NumberOrder, ToNumber, error::NumberError};

/// Evaluates expression.
pub fn eval(infix_expression: &str) -> Result<Number, CalculatorError> {
    let tokens = ast::tokenize(infix_expression)?;
    let rpn_tokens = ast::parse(tokens)?;
    let result = ast::eval(rpn_tokens)?;
    Ok(result)
}
