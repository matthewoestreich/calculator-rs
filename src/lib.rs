mod number;
mod parser;

pub use bigdecimal;
pub use num_bigint;
pub use number::{Number, NumberError, NumberOrder, ToNumber};

use crate::parser::ParserError;
use std::{error, fmt};

/// Evaluates infix expression.
pub fn parse_expression(infix_expression: &str) -> Result<Number, CalculatorError> {
    let tokens = parser::tokenize(infix_expression)?;
    let rpn_tokens = parser::parse(tokens)?;
    let result = parser::eval(rpn_tokens)?;
    Ok(result)
}

// ===========================================================================================
// ========================== Calculator =====================================================
// ===========================================================================================

#[derive(Default, Debug, Clone)]
pub struct Calculator {
    infix: String,
}

impl Calculator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_infix(infix: &str) -> Self {
        Self {
            infix: infix.to_string(),
        }
    }

    /// Returns clone of current infix expression
    pub fn infix(&self) -> String {
        String::from(self.infix.clone().trim())
    }

    /// Add digit to infix expression.
    pub fn press(&mut self, key: Key) {
        self.infix = format!("{}{key}", self.infix);
    }

    /// Concates the provided expression to current infix expression with NO trailing space.
    /// It is up to you to ensure the provided expression contains valid characters!
    pub fn expression(&mut self, infix_expression: &str) {
        self.infix = format!("{}{infix_expression}", self.infix);
    }

    /// Calculates constructed infix string.
    /// We set the result to be the new infix, so you can use the result in further calculations.
    pub fn calculate(&mut self) -> Result<Number, CalculatorError> {
        let tokens = parser::tokenize(&self.infix)?;
        let rpn_tokens = parser::parse(tokens)?;
        let result = parser::eval(rpn_tokens)?;
        self.infix = result.to_string();
        Ok(result)
    }

    /// Clear current infix
    pub fn clear(&mut self) {
        self.infix.clear();
    }
}

// ===========================================================================================
// ========================== Key ============================================================
// ===========================================================================================

#[derive(Debug, Clone, Copy)]
pub enum Key {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Add,
    Subtract,
    Multiply,
    Divide,
    Pow,
    ParenthesesOpen,
    ParenthesesClose,
    Period,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = match self {
            Key::Zero => "0",
            Key::One => "1",
            Key::Two => "2",
            Key::Three => "3",
            Key::Four => "4",
            Key::Five => "5",
            Key::Six => "6",
            Key::Seven => "7",
            Key::Eight => "8",
            Key::Nine => "9",
            Key::Add => "+",
            Key::Subtract => "-",
            Key::Multiply => "*",
            Key::Divide => "/",
            Key::Pow => "^",
            Key::ParenthesesOpen => "(",
            Key::ParenthesesClose => ")",
            Key::Period => ".",
        };
        write!(f, "{r}")
    }
}

// ===========================================================================================
// ========================== CalculatorError ================================================
// ===========================================================================================

#[derive(Debug, Clone)]
pub struct CalculatorError {
    message: String,
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CalculatorError {}", self.message)
    }
}

impl From<ParserError> for CalculatorError {
    fn from(error: ParserError) -> Self {
        Self {
            message: format!("{error}").to_string(),
        }
    }
}

impl error::Error for CalculatorError {}

// ===========================================================================================
// ========================== Tests ==========================================================
// ===========================================================================================

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn strictly_press() {
        let mut c = Calculator::new();

        let expected_expression = "3+4*2/(1-5)";
        let expected_answer = Number::Int(1.into());

        c.press(Key::Three);
        c.press(Key::Add);
        c.press(Key::Four);
        c.press(Key::Multiply);
        c.press(Key::Two);
        c.press(Key::Divide);
        c.press(Key::ParenthesesOpen);
        c.press(Key::One);
        c.press(Key::Subtract);
        c.press(Key::Five);
        c.press(Key::ParenthesesClose);

        assert_eq!(
            c.infix(),
            expected_expression,
            "expected '{expected_expression}' got '{}'",
            c.infix()
        );

        let answer = c.calculate().unwrap();

        assert_eq!(
            answer, expected_answer,
            "expected {expected_answer} got {answer}"
        );
    }

    #[test]
    fn press_multiple_digits() {
        let mut c = Calculator::new();

        let expected_expression = "33+44";
        let expected_answer = Number::Int(77.into());

        c.press(Key::Three);
        c.press(Key::Three);
        c.press(Key::Add);
        c.press(Key::Four);
        c.press(Key::Four);

        assert_eq!(
            c.infix(),
            expected_expression,
            "expected '{expected_expression}' got '{}'",
            c.infix()
        );

        let answer = c.calculate().unwrap();

        assert_eq!(
            answer, expected_answer,
            "expected {expected_answer} got {answer}"
        );
    }

    #[test]
    fn press_period() {
        let mut c = Calculator::new();

        let expected_expression = "3.3+4.4";
        let expected_answer = Number::from_f64(7.7).unwrap();

        c.press(Key::Three);
        c.press(Key::Period);
        c.press(Key::Three);
        c.press(Key::Add);
        c.press(Key::Four);
        c.press(Key::Period);
        c.press(Key::Four);

        assert_eq!(
            c.infix(),
            expected_expression,
            "expected '{expected_expression}' got '{}'",
            c.infix()
        );

        let answer = c.calculate().unwrap();

        assert_eq!(
            answer, expected_answer,
            "expected {expected_answer} got {answer}"
        );
    }

    #[test]
    fn mix_press_and_expression() {
        let mut c = Calculator::new();

        let expected_expression = "(3 + 3) /2";
        let expected_answer = Number::Int(3.into());

        c.expression("(3 + 3) ");
        c.press(Key::Divide);
        c.press(Key::Two);

        assert_eq!(
            c.infix(),
            expected_expression,
            "expected '{expected_expression}' got '{}'",
            c.infix()
        );

        let answer = c.calculate().unwrap();

        assert_eq!(
            answer, expected_answer,
            "expected {expected_answer} got {answer}"
        );
    }

    #[test]
    fn strictly_expression() {
        let mut c = Calculator::new();
        c.expression("2+2");
        let answer = c.calculate().unwrap();
        let expected_answer = Number::Int(4.into());
        assert_eq!(
            answer, expected_answer,
            "expected {expected_answer} got {answer}"
        );
    }

    #[test]
    fn continued_calc() {
        let mut c = Calculator::new();
        c.expression("2+2");
        let expected = Number::Int(4.into());
        let answer = c.calculate().unwrap();
        assert_eq!(c.infix(), "4", "expected 4 got {}", c.infix());
        assert_eq!(answer, expected, "expected {expected} got {answer}");
        c.press(Key::Add);
        c.press(Key::Four);
        let expected = Number::Int(8.into());
        let answer = c.calculate().unwrap();
        assert_eq!(answer, expected, "expected {expected} got {answer}");
    }
}
