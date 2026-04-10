use super::{Associativity, Token, error::ParserError};

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
