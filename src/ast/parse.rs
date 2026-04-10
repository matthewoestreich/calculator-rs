use super::{Associativity, OperationOrder, Operator, Token, error::ParserError};

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
            Token::ParenthesesClose => parse_closed_paren(&mut stack, &mut output)?,
            Token::Operator(ref op) => {
                parse_operator(op, &mut stack, &mut output)?;
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

fn parse_closed_paren(stack: &mut Vec<Token>, output: &mut Vec<Token>) -> Result<(), ParserError> {
    let mut found_open_paren = false;

    while let Some(t) = stack.pop() {
        if matches!(t, Token::ParenthesesOpen) {
            found_open_paren = true;
            break;
        }
        output.push(t);
    }

    if !found_open_paren {
        return Err(ParserError::MissingOpeningParentheses);
    }

    if matches!(stack.last(), Some(Token::Function(_))) {
        output.push(stack.pop().expect("just verified .last"));
    }

    Ok(())
}

fn parse_operator(
    op: &Operator,
    stack: &mut Vec<Token>,
    output: &mut Vec<Token>,
) -> Result<(), ParserError> {
    let precedence = op.precedence();
    let associativity = op.associativity();

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

        output.push(stack.pop().ok_or(ParserError::InvalidExpression)?);
    }

    Ok(())
}
