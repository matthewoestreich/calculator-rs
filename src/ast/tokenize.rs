use super::{Function, Operator, Token, error::ParserError};
use crate::Number;
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
                let func = tokenize_function(&c, &mut iter)?;
                tokens.push(Token::Function(func));
            }
            '-' | '!' if Operator::is_unary_context(&tokens) => {
                tokens.push(Token::Operator(match c {
                    '-' => Operator::Negate,
                    '!' => Operator::Not,
                    _ => return Err(ParserError::UnexpectedChar(c)),
                }));
            }
            '*' | '<' | '>' if Operator::has_two_chars(&c, &mut iter) => {
                let sc = iter.next().expect("just validated next via peek");
                tokens.push(Token::Operator(match sc {
                    '*' => Operator::Exponentiation,
                    '<' => Operator::ShiftLeft,
                    '>' => Operator::ShiftRight,
                    _ => return Err(ParserError::UnexpectedChar(c)),
                }));
            }
            '+' | '-' | '*' | '/' | '%' | '&' | '|' | '^' | '<' | '>' => {
                let op = tokenize_single_char_operator(&c)?;
                tokens.push(Token::Operator(op));
            }
            c if c.is_ascii_digit() || c == '.' => {
                let number = tokenize_number(&c, &mut iter)?;
                tokens.push(Token::Number(number));
            }
            _ => return Err(ParserError::InvalidExpression),
        }
    }

    Ok(tokens)
}

fn tokenize_function(c: &char, iter: &mut iter::Peekable<Chars>) -> Result<Function, ParserError> {
    let mut fn_name_str = String::from(*c);

    while let Some(&p) = iter.peek()
        && p.is_ascii_alphabetic()
    {
        fn_name_str.push(p);
        iter.next();
    }

    fn_name_str.parse::<Function>()
}

fn tokenize_single_char_operator(c: &char) -> Result<Operator, ParserError> {
    Ok(match c {
        '+' => Operator::Add,
        '-' => Operator::Subtract,
        '*' => Operator::Multiply,
        '/' => Operator::Divide,
        '%' => Operator::Remainder,
        '&' => Operator::And,
        '|' => Operator::Or,
        '^' => Operator::Xor,
        _ => return Err(ParserError::UnexpectedChar(*c)),
    })
}

fn tokenize_number(c: &char, iter: &mut iter::Peekable<Chars>) -> Result<Number, ParserError> {
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
