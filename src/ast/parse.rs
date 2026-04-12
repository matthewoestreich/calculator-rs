use crate::ast::{Associativity, Token, error::ParserError};

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
            Token::Constant(_) => output.push(token),
            Token::Function(_) => stack.push(token),
            Token::ParenthesesOpen => stack.push(token),
            Token::ParenthesesClose => {
                let mut has_open_paren = false;

                while let Some(t) = stack.pop() {
                    if matches!(t, Token::ParenthesesOpen) {
                        has_open_paren = true;
                        break;
                    }

                    output.push(t);
                }

                if !has_open_paren {
                    return Err(ParserError::MissingOpeningParentheses);
                }

                if matches!(stack.last(), Some(Token::Function(_))) {
                    output.push(stack.pop().expect("just verified .last"));
                }
            }
            Token::Operator(ref operator) => {
                let precedence = operator.precedence();
                let associativity = operator.associativity();

                while let Some(top) = stack.last() {
                    if matches!(top, Token::ParenthesesOpen) {
                        break;
                    }

                    let top_precedence = top.precedence();
                    if match associativity {
                        Associativity::Left => precedence > top_precedence,
                        Associativity::Right => precedence >= top_precedence,
                    } {
                        break;
                    }

                    output.push(stack.pop().expect("valid while condition"));
                }

                stack.push(token);
            }
        }
    }

    while let Some(p) = stack.pop() {
        if matches!(p, Token::ParenthesesOpen) {
            return Err(ParserError::MissingClosingParentheses);
        }
        output.push(p);
    }

    Ok(output)
}

#[cfg(test)]
mod test {
    use crate::ast::test::tokens_to_str;
    use crate::ast::{parse, tokenize};
    use rstest::*;

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
    #[case::parsing_pi("pi", "pi")]
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
}
