use regex::Regex;

// Shunting-Yard algorithm.
// Returns expression in reverse polish notation.
pub fn parse(infix: &str) -> Option<String> {
    if infix.is_empty() {
        return None;
    }

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
            t if t.parse::<i128>().is_ok() || t.parse::<f64>().is_ok() => output.push(t),
            t => {
                while let Some(&top) = stack.last() {
                    if top == "(" || precedence(top) < precedence(t) {
                        break;
                    }
                    output.push(stack.pop().expect("stack not empty"));
                }
                stack.push(t);
            }
        }
    }

    while let Some(p) = stack.pop() {
        output.push(p);
    }

    Some(output.join(" "))
}

fn tokenize(expression: &str) -> Vec<&str> {
    // Matches numbers (\d+), variables ([a-zA-Z]+), or single operators/parentheses ([()+\-*/^])
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
    fn shunting_yard_reverse_polish_notation() {
        let rpn_str = parse("3 +4* 2 /(1 - 5)").unwrap();
        let expected = "3 4 2 * 1 5 - / +";
        assert_eq!(
            rpn_str, expected,
            "expected '{expected:?}' got '{rpn_str:?}'"
        );

        let rpn_str = parse("33 +44* 22 /(11 - 55)").unwrap();
        let expected = "33 44 22 * 11 55 - / +";
        assert_eq!(
            rpn_str, expected,
            "expected '{expected:?}' got '{rpn_str:?}'"
        );

        let rpn_str = parse("33 +44* 22 /(11 - 55)").unwrap();
        let expected = "3 4 2 * 1 5 - / +";
        assert_eq!(
            rpn_str, expected,
            "expected '{expected:?}' got '{rpn_str:?}'"
        );
    }
}
