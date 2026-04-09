use crate::{Number, NumberError};
use bigdecimal::ParseBigDecimalError;
use std::{error, fmt, str::FromStr};

// ===========================================================================================
// ========================== Operator =======================================================
// ===========================================================================================

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
    pub fn is_unary(&self) -> bool {
        matches!(self, Self::Negate | Self::Not)
    }

    /// This method assumes you have already verified the first char!
    /// What you are passing in would be the second char.
    /// Example of two-character operators : `**`, `<<`, `>>`
    pub fn has_two_chars(second_char: &char) -> bool {
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

// ===========================================================================================
// ========================== Function =======================================================
// ===========================================================================================

#[derive(Debug, Clone)]
pub enum Function {
    Abs,
}

impl FromStr for Function {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s == Self::Abs.to_string() => Ok(Self::Abs),
            _ => Err(ParserError::UnrecognizedFunction {
                name: s.to_string(),
            }),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // All functions should be lower case!
        match self {
            Function::Abs => write!(f, "abs"),
        }
    }
}

// ===========================================================================================
// ========================== Associativity ==================================================
// ===========================================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Associativity {
    Left,
    Right,
}

// ===========================================================================================
// ========================== Token ==========================================================
// ===========================================================================================

#[derive(Debug, Clone)]
pub enum Token {
    Number(Number),
    Operator(Operator),
    Function(Function),
    ParenthesesOpen,
    ParenthesesClose,
}

impl Token {
    pub fn is_unary(&self) -> bool {
        if let Token::Operator(o) = self {
            return o.is_unary();
        }
        false
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
            // Functions have the highest priority
            Token::Function(_) => 100,
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
            Token::Function(func) => write!(f, "{func}"),
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
    if expression.is_empty() {
        return Err(ParserError::EmptyExpression);
    }

    let mut tokens = Vec::<Token>::new();
    let mut iter = expression.chars().peekable();

    while let Some(c) = iter.next() {
        if c.is_whitespace() {
            continue;
        }

        match c {
            '(' => tokens.push(Token::ParenthesesOpen),
            ')' => tokens.push(Token::ParenthesesClose),

            // If we encounter an ASCII character, it means we have a function.
            c if c.is_ascii_alphabetic() => {
                let mut fn_name_str = String::from(c);

                while let Some(&p) = iter.peek()
                    && p.is_ascii_alphabetic()
                {
                    fn_name_str.push(p);
                    // Move to next char
                    _ = iter.next();
                }

                let func = fn_name_str.parse::<Function>()?;
                tokens.push(Token::Function(func));
            }

            // If 'c' is considered unary given the `tokens`` context.
            '-' | '!' if Operator::is_unary_context(&tokens) => {
                tokens.push(Token::Operator(match c {
                    '-' => Operator::Negate,
                    '!' => Operator::Not,
                    _ => return Err(ParserError::UnexpectedChar(c)),
                }));
            }

            // Two-character operators, e.g., `**`, `<<`, `>>`
            '*' | '<' | '>' if iter.peek().is_some_and(Operator::has_two_chars) => {
                let sc = iter.next().expect("just validated next via peek");

                tokens.push(Token::Operator(match sc {
                    '*' => Operator::Exponentiation,
                    '<' => Operator::ShiftLeft,
                    '>' => Operator::ShiftRight,
                    _ => return Err(ParserError::UnexpectedChar(c)),
                }));
            }

            // Single-character operators.
            '+' | '-' | '*' | '/' | '%' | '&' | '|' | '^' | '<' | '>' => {
                tokens.push(Token::Operator(match c {
                    '+' => Operator::Add,
                    '-' => Operator::Subtract,
                    '*' => Operator::Multiply,
                    '/' => Operator::Divide,
                    '%' => Operator::Remainder,
                    '&' => Operator::And,
                    '|' => Operator::Or,
                    '^' => Operator::Xor,
                    _ => return Err(ParserError::UnexpectedChar(c)),
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
                    .map_err(|_| ParserError::InvalidNumber(num_str))?;

                tokens.push(Token::Number(number));
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
    if infix_tokens.is_empty() {
        return Err(ParserError::EmptyExpression);
    }

    let mut output = vec![];
    let mut stack = vec![];

    for token in infix_tokens {
        match token {
            Token::Number(_) => output.push(token),
            Token::Function(_) | Token::ParenthesesOpen => stack.push(token),
            Token::ParenthesesClose => {
                while let Some(t) = stack.pop()
                    && !matches!(t, Token::ParenthesesOpen)
                {
                    output.push(t);
                }

                if matches!(stack.last(), Some(Token::Function(_))) {
                    output.push(stack.pop().expect("just verified .last"));
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
        match token {
            Token::Number(n) => stack.push(n),
            Token::Function(f) => {
                let x = stack.pop().ok_or(ParserError::InvalidExpression)?;

                stack.push(match f {
                    Function::Abs => x.abs(),
                });
            }
            Token::Operator(o) => {
                if o.is_unary() {
                    let x = stack.pop().ok_or(ParserError::InvalidExpression)?;

                    stack.push(match o {
                        Operator::Negate => -x,
                        Operator::Not => !x,
                        _ => return Err(ParserError::ExpectedUnary(o)),
                    });
                } else {
                    // Order matters here! 'rhs' must be popped before 'lhs'!
                    let rhs = stack.pop().ok_or(ParserError::InvalidExpression)?;
                    let lhs = stack.pop().ok_or(ParserError::InvalidExpression)?;

                    stack.push(match o {
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
            _ => return Err(ParserError::UnexpectedToken(token)),
        }
    }

    // There MUST be only one element on the stack here.
    if stack.len() != 1 {
        return Err(ParserError::InvalidExpression);
    }
    Ok(stack.pop().expect("just verified len"))
}

// ===========================================================================================
// ========================== TryFrom impl(s) for Number =====================================
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
    UnrecognizedFunction {
        name: String,
    },
    /// `Operator` argument is what you got instead
    ExpectedUnary(Operator),
    /// `Token` argument is what you got instead
    ExpectedFunction(Token),
    /// `Token` argument is what you got instead
    ExpectedOperator(Token),
    /// `Token` is what you got, not what you expected
    UnexpectedToken(Token),
    UnexpectedChar(char),
    InvalidExponent {
        exponent_str: String,
    },
    InvalidNumber(String),
    NumberErr(NumberError),
    BigDecimalErr(ParseBigDecimalError),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::EmptyExpression => write!(f, "expression cannot be empty"),
            ParserError::InvalidExpression => write!(f, "expression is invalid"),
            ParserError::ExpectedUnary(got) => {
                write!(f, "expected valid unary operator, got '{got}'")
            }
            ParserError::ExpectedFunction(got) => write!(f, "expected function, got '{got}'"),
            ParserError::ExpectedOperator(got) => write!(f, "expected operator, got '{got}'"),
            ParserError::UnrecognizedFunction { name } => {
                write!(f, "function with name '{name}' is not recognized")
            }
            ParserError::InvalidNumber(n_str) => write!(f, "invalid number : '{n_str}'"),
            ParserError::BigDecimalErr(e) => write!(f, "error parsing BigDecimal : {e}"),
            ParserError::NumberErr(ne) => write!(f, "{ne}"),
            ParserError::UnexpectedChar(c) => write!(f, "unexpected char '{c}'"),
            ParserError::UnexpectedToken(got) => {
                write!(f, "got '{got}' and did not expect to")
            }
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
    #[case::tokenization1("2 + abs((2+2)-10)", "2 ADD abs ( ( 2 ADD 2 ) SUB 10 )")]
    #[case::tokenization2(
        "abs( 10 - abs( ( 2 + 2 ) - 10 ) )",
        "abs ( 10 SUB abs ( ( 2 ADD 2 ) SUB 10 ) )"
    )]
    #[case::tokenization3(
        "-abs( 10 - abs( -( 2 + 2 ) - 10 ) )",
        "NEG abs ( 10 SUB abs ( NEG ( 2 ADD 2 ) SUB 10 ) )"
    )]
    // Uses the `Display` impls for `expect_tokens`.
    fn tokenization(#[case] raw_infix: &str, #[case] expect_tokens: &str) {
        let tokens = match tokenize(raw_infix) {
            Ok(t) => t,
            Err(e) => panic!("TOKENIZATION ERROR = {e:?}"),
        };
        let tokens_str = tokens_to_str(&tokens);
        assert_eq!(
            tokens_str, expect_tokens,
            "expected '{expect_tokens}' got '{tokens_str}'"
        );
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
    #[case::parsing_func("2 + abs((2+2)-10)", "2 2 2 ADD 10 SUB abs ADD")]
    #[case::parsing_nested_func(
        "abs( 10 - abs( ( 2 + 2 ) - 10 ) )",
        "10 2 2 ADD 10 SUB abs SUB abs"
    )]
    #[case::parsing_nested_func_with_neg(
        "-abs( 10 - abs( -( 2 + 2 ) - 10 ) )",
        "10 2 2 ADD NEG 10 SUB abs SUB abs NEG"
    )]
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
            "expected '{expect_rpn}' got '{rpn_str}'"
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
    #[case::evaluate_func("2 + abs((2+2)-10)", "8")]
    #[case::evaluate_nested_func("abs( 10 - abs( ( 2 + 2 ) - 10 ) )", "4")]
    #[case::evaluate_nested_func_with_neg("-abs( 10 - abs( -( 2 + 2 ) - 10 ) )", "-4")]
    #[case::evaluate11("!abs(-abs(2+3))", "-6")]
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
