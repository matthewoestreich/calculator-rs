use crate::{
    Number,
    ast::{Binary, Constant, Function, Operator, Token, Unary, error::ParserError},
};
use std::{iter, str::Chars};

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
            c if c.is_ascii_alphabetic() => {
                let i = read_identifier(&c, &mut iter);
                let t = i
                    .parse::<Function>()
                    .map(Token::Function)
                    .or_else(|_| i.parse::<Constant>().map(Token::Constant))
                    .map_err(|_| ParserError::UnrecognizedIdentifier {
                        name: i.to_string(),
                    })?;
                tokens.push(t);
            }
            c if c.is_ascii_digit() || c == '.' => {
                let number = read_and_tokenize_number(&c, &mut iter)?;
                tokens.push(Token::Number(number));
            }
            '+' | '-' | '*' | '/' | '%' | '&' | '|' | '^' | '<' | '>' | '!' => {
                // `if` First check for unary operators based upon tokens context.
                // `else if` Next, we check for operators that have two characters.
                // `else` Finally, we fall-thru to the reamining binary operators.
                let t = Token::Operator(if Operator::is_unary_context(&tokens) {
                    Operator::Unary(match c {
                        '-' => Unary::Negate,
                        '!' => Unary::Not,
                        _ => return Err(ParserError::UnexpectedChar(c)),
                    })
                } else if iter.peek().is_some_and(|s| op_has_two_chars(&c, s)) {
                    let cs = iter.next().expect("just validated next via peek");
                    Operator::Binary(match cs {
                        '*' => Binary::Exponentiation,
                        '<' => Binary::ShiftLeft,
                        '>' => Binary::ShiftRight,
                        _ => return Err(ParserError::UnexpectedChar(c)),
                    })
                } else {
                    Operator::Binary(match c {
                        '+' => Binary::Add,
                        '-' => Binary::Subtract,
                        '*' => Binary::Multiply,
                        '/' => Binary::Divide,
                        '%' => Binary::Remainder,
                        '&' => Binary::And,
                        '|' => Binary::Or,
                        '^' => Binary::Xor,
                        _ => return Err(ParserError::UnexpectedChar(c)),
                    })
                });
                tokens.push(t);
            }
            _ => return Err(ParserError::InvalidExpression),
        }
    }

    Ok(tokens)
}

/// Verifies if an operator exists with these two characters, in first_second char order.
fn op_has_two_chars(first_char: &char, second_char: &char) -> bool {
    Operator::two_char_ops().contains(&(*first_char, *second_char))
}

/// Could be a function (like `sin`, `abs`, etc..),
/// or a constant (like `pi`) we don't know which yet.
fn read_identifier(c: &char, iter: &mut iter::Peekable<Chars>) -> String {
    let mut fn_name_str = String::from(*c);

    while let Some(&p) = iter.peek()
        && p.is_ascii_alphabetic()
    {
        fn_name_str.push(p);
        iter.next();
    }

    fn_name_str
}

fn read_and_tokenize_number(
    c: &char,
    iter: &mut iter::Peekable<Chars>,
) -> Result<Number, ParserError> {
    let mut num_str = String::from(*c);

    while let Some(&p) = iter.peek()
        && (p.is_ascii_digit() || p == '.')
    {
        num_str.push(p);
        iter.next();
    }

    num_str
        .parse::<Number>()
        .map_err(|_| ParserError::InvalidNumber(num_str))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::test::tokens_to_str;
    use rstest::*;

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
    #[case::tokenization4("pi", "pi")]
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
}
