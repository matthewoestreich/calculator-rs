use crate::{Number, NumberError};
use bigdecimal::ParseBigDecimalError;
use std::{error, fmt};

#[derive(Debug, Clone)]
pub enum Operator {
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
    Negate,         // -
    Not,            // !
}

impl Operator {
    /// This method assumes you have already verified the first char!
    /// What you are passing in would be the second char.
    pub fn is_multichar_infix(second_char: &char) -> bool {
        matches!(second_char, '*' | '<' | '>')
    }

    /// Determines if an ambiguous operator (such as `-`) is considered
    /// unary or infix given the provided `tokens` context.
    /// `tokens` represent the currently parsed tokens at any given time,
    /// hence why we refer to them as the 'context'.
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
            Operator::Add => write!(f, "ADD"),
            Operator::Subtract => write!(f, "SUB"),
            Operator::Multiply => write!(f, "MUL"),
            Operator::Divide => write!(f, "DIV"),
            Operator::Exponentiation => write!(f, "EXP"),
            Operator::Remainder => write!(f, "REM"),
            Operator::And => write!(f, "AND"),
            Operator::Or => write!(f, "OR"),
            Operator::Xor => write!(f, "XOR"),
            Operator::ShiftLeft => write!(f, "SHL"),
            Operator::ShiftRight => write!(f, "SHR"),
            Operator::Negate => write!(f, "NEG"),
            Operator::Not => write!(f, "NOT"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Associativity {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub enum Token {
    Number(Number),
    Operator(Operator),
    ParenthesesOpen,
    ParenthesesClose,
}

impl Token {
    pub fn is_unary(&self) -> bool {
        matches!(self, Self::Operator(Operator::Negate | Operator::Not))
    }

    /// Determines `&Token` associativity.
    /// Default associativity is `Associativity::Left`
    pub fn associativity(&self) -> Associativity {
        match self {
            Token::Operator(Operator::Exponentiation | Operator::Negate | Operator::Not) => {
                Associativity::Right
            }
            _ => Associativity::Left,
        }
    }

    /// Determines `&Token` precedence.
    /// We use "C-style" operator precedence.
    pub fn precedence(&self) -> i32 {
        match self {
            Token::Operator(o) => match o {
                Operator::Negate | Operator::Not => 8,
                Operator::Exponentiation => 7,
                Operator::Multiply | Operator::Divide | Operator::Remainder => 6,
                Operator::Add | Operator::Subtract => 5,
                Operator::ShiftLeft | Operator::ShiftRight => 4,
                Operator::And => 3,
                Operator::Xor => 2,
                Operator::Or => 1,
            },
            _ => 0,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(number) => write!(f, "{number}"),
            Token::Operator(op_kind) => write!(f, "{op_kind}"),
            Token::ParenthesesOpen => write!(f, "("),
            Token::ParenthesesClose => write!(f, ")"),
        }
    }
}

// ===========================================================================================
// ========================== Tokenize =======================================================
// ===========================================================================================

pub fn tokenize(expression: &str) -> Result<Vec<Token>, ParserError> {
    let mut tokens = Vec::<Token>::new();
    let mut iter = expression.chars().peekable();

    while let Some(c) = iter.next() {
        if c.is_whitespace() {
            continue;
        }

        match c {
            '(' => tokens.push(Token::ParenthesesOpen),
            ')' => tokens.push(Token::ParenthesesClose),

            // If 'c' is considered unary given the `tokens`` context.
            '-' | '!' if Operator::is_unary_context(&tokens) => {
                tokens.push(Token::Operator(match c {
                    '-' => Operator::Negate,
                    '!' => Operator::Not,
                    _ => {
                        return Err(ParserError::UnexpectedToken {
                            expected: "valid unary operator".to_string(),
                        });
                    }
                }));
            }

            // Infix operators
            '+' | '-' | '*' | '/' | '%' | '&' | '|' | '^' | '<' | '>' => {
                // Multi-character infix operator check and parsing.
                if let Some(p) = iter.peek()
                    && Operator::is_multichar_infix(p)
                {
                    tokens.push(Token::Operator(match p {
                        '*' => Operator::Exponentiation,
                        '<' => Operator::ShiftLeft,
                        '>' => Operator::ShiftRight,
                        _ => {
                            return Err(ParserError::UnexpectedToken {
                                expected: "multi-character infix operator".to_string(),
                            });
                        }
                    }));

                    // Skip peeked char
                    _ = iter.next();
                    continue;
                }

                // Single-character infix operator parsing.
                tokens.push(Token::Operator(match c {
                    '+' => Operator::Add,
                    '-' => Operator::Subtract,
                    '*' => Operator::Multiply,
                    '/' => Operator::Divide,
                    '%' => Operator::Remainder,
                    '&' => Operator::And,
                    '|' => Operator::Or,
                    '^' => Operator::Xor,
                    _ => {
                        return Err(ParserError::UnexpectedToken {
                            expected: "single-character infix operator".to_string(),
                        });
                    }
                }));
            }

            // Parse single-character or multi-character numbers.
            c if c.is_ascii_digit() || c == '.' => {
                let mut num_str = String::from(c);

                // Multi-character numbers.
                while let Some(&p) = iter.peek()
                    && (p.is_ascii_digit() || p == '.')
                {
                    num_str.push(p);
                    _ = iter.next();
                }

                let number = num_str
                    .parse::<Number>()
                    .map_err(|_| ParserError::InvalidExpression)?;

                tokens.push(Token::Number(number));
                continue;
            }

            // Should be unreachable
            _ => return Err(ParserError::InvalidExpression),
        }
    }

    Ok(tokens)
}

// ===========================================================================================
// ========================== Parse ==========================================================
// ===========================================================================================

/// Uses shunting yard algorithm.
/// Returns `Vec<Token>` in reverse polish notation.
pub fn parse(infix_tokens: Vec<Token>) -> Result<Vec<Token>, ParserError> {
    let mut output = vec![];
    let mut stack = vec![];

    for token in infix_tokens {
        match token {
            Token::Number(_) => output.push(token),
            Token::ParenthesesOpen => stack.push(token),
            Token::ParenthesesClose => {
                while let Some(t) = stack.pop() {
                    if matches!(t, Token::ParenthesesOpen) {
                        break;
                    }
                    output.push(t);
                }
            }
            Token::Operator(_) => {
                let token_precedence = token.precedence();

                while let Some(top) = stack.last() {
                    if matches!(top, Token::ParenthesesOpen) {
                        break;
                    }

                    let top_precedence = top.precedence();
                    if match token.associativity() {
                        Associativity::Left => token_precedence > top_precedence,
                        Associativity::Right => token_precedence >= top_precedence,
                    } {
                        break;
                    }

                    output.push(stack.pop().ok_or(ParserError::InvalidExpression)?);
                }

                stack.push(token);
            }
        }
    }

    while let Some(p) = stack.pop() {
        output.push(p);
    }

    Ok(output)
}

// ===========================================================================================
// ========================== Eval ===========================================================
// ===========================================================================================

// Expects tokens in reverse polish notation
pub fn eval(rpn_tokens: Vec<Token>) -> Result<Number, ParserError> {
    if rpn_tokens.is_empty() {
        return Err(ParserError::EmptyExpression);
    }

    let mut stack = Vec::<Number>::new();

    for token in rpn_tokens {
        if let Ok(n) = Number::try_from(&token) {
            stack.push(n);
            continue;
        }

        // Order matters here! 'b' must be popped before 'a'!
        let b = stack.pop().ok_or(ParserError::InvalidExpression)?;

        if token.is_unary() {
            match &token {
                Token::Operator(Operator::Negate) => stack.push(-b),
                Token::Operator(Operator::Not) => stack.push(!b),
                _ => {
                    return Err(ParserError::UnexpectedToken {
                        expected: "valid unary operator".to_string(),
                    });
                }
            }
            continue;
        }

        // Only pop 'a' if we know 'b' was NOT unary.
        // Order matters here! 'a' must be popped after 'b'!
        let a = stack.pop().ok_or(ParserError::InvalidExpression)?;

        match &token {
            Token::Operator(o) => stack.push(match o {
                Operator::Add => a + b,
                Operator::Subtract => a - b,
                Operator::Multiply => a * b,
                Operator::Divide => a / b,
                Operator::Exponentiation => a.pow(b.to_i64_saturating())?,
                Operator::Remainder => a % b,
                Operator::And => a & b,
                Operator::Or => a | b,
                Operator::Xor => a ^ b,
                Operator::ShiftLeft => a << b,
                Operator::ShiftRight => a >> b,
                // Matches all unary operators, which we already handled above.
                _ => unreachable!("unary operators should have been handled already"),
            }),
            _ => return Err(ParserError::InvalidExpression),
        }
    }

    stack
        .into_iter()
        .next()
        .ok_or(ParserError::InvalidExpression)
}

// ===========================================================================================
// ========================== TryFrom<Token> for Number ======================================
// ===========================================================================================

impl TryFrom<&Token> for Number {
    type Error = ParserError;

    fn try_from(token: &Token) -> Result<Self, Self::Error> {
        match token {
            Token::Number(number) => Ok(number.clone()),
            _ => Err(ParserError::InvalidExpression),
        }
    }
}

// ===========================================================================================
// ========================== ParserError ====================================================
// ===========================================================================================

#[derive(Debug, Clone)]
pub enum ParserError {
    EmptyExpression,
    InvalidExpression,
    UnexpectedToken { expected: String },
    InvalidExponent { exponent_str: String },
    NumberErr(NumberError),
    BigDecimalErr(ParseBigDecimalError),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::EmptyExpression => write!(f, "expression cannot be empty"),
            ParserError::InvalidExpression => write!(f, "expression is invalid"),
            ParserError::BigDecimalErr(e) => write!(f, "error parsing BigDecimal : {e}"),
            ParserError::NumberErr(ne) => write!(f, "{ne}"),
            ParserError::UnexpectedToken { expected } => write!(f, "{expected}"),
            ParserError::InvalidExponent { exponent_str } => write!(
                f,
                "{exponent_str} : is either Number::Decimal(x) or is unable to be represented by an i64 (eg. it is a float, etc..)"
            ),
        }
    }
}

impl error::Error for ParserError {}

impl From<NumberError> for ParserError {
    fn from(error: NumberError) -> Self {
        Self::NumberErr(error)
    }
}

impl From<ParseBigDecimalError> for ParserError {
    fn from(value: ParseBigDecimalError) -> Self {
        Self::BigDecimalErr(value)
    }
}

// ===========================================================================================
// ========================== Tests ==========================================================
// ===========================================================================================

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    fn tokens_to_str(tokens: &[Token]) -> String {
        tokens
            .iter()
            .fold(String::new(), |acc, x| format!("{acc} {x}"))
            .trim()
            .to_string()
    }

    #[rstest]
    #[case::parsing1(
        "!-(-3 + 4 * (2 - -5)) ^ 2 << 1 + !-6 * 3 - --7",
        "3 NEG 4 2 5 NEG SUB MUL ADD NEG NOT 2 1 6 NEG NOT 3 MUL ADD 7 NEG NEG SUB SHL XOR"
    )]
    #[case::parsing2("-2 ** 2", "2 NEG 2 EXP")]
    #[case::parsing3("-2 ** 3 ** 2", "2 NEG 3 2 EXP EXP")]
    #[case::parsing4("!-2 ** 2", "2 NEG NOT 2 EXP")]
    #[case::parsing5("-2 * 3 ** 2", "2 NEG 3 2 EXP MUL")]
    #[case::parsing6("!-!-(2+7)", "2 7 ADD NEG NOT NEG NOT")]
    #[case::parsing7("-(2+3) * 4", "2 3 ADD NEG 4 MUL")]
    #[case::parsing8("(1 + 2) * (3 - 4) / 5", "1 2 ADD 3 4 SUB MUL 5 DIV")]
    #[case::parsing9("(1 + 2) << (3 & 4)", "1 2 ADD 3 4 AND SHL")]
    #[case::parsing_use_every_operator(
        "!-3 + 4 * (5 - 6) / 7 % 2 ** 3 << 1 >> 2 & 3 ^ 4 | 5",
        "3 NEG NOT 4 5 6 SUB MUL 7 DIV 2 3 EXP REM ADD 1 SHL 2 SHR 3 AND 4 XOR 5 OR"
    )]
    #[case::parsing10("1/2", "1 2 DIV")]
    #[case::parsing_starts_with_dec(".5 + .5", "0.5 0.5 ADD")]
    fn parsing(#[case] raw_infix: &str, #[case] expect_rpn: &str) {
        let tokens = match tokenize(raw_infix) {
            Ok(t) => t,
            Err(e) => panic!("TOKENIZATION ERROR = {e:?}"),
        };
        let rpn_tokens = match parse(tokens.clone()) {
            Ok(t) => t,
            Err(e) => panic!("PARSER ERROR = {e:?}"),
        };
        let rpn_str = tokens_to_str(&rpn_tokens);
        assert_eq!(
            rpn_str,
            String::from(expect_rpn),
            "expected {expect_rpn} got {rpn_str}"
        );
    }

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
            "expression {raw_infix} | expected {expected:?} got {result:?}"
        );
    }

    /*
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
    */
}
