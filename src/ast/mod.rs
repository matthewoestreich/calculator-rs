mod eval;
mod function;
mod operator;
mod parse;
mod token;
mod tokenize;

pub mod error;

pub use eval::eval;
pub use function::Function;
pub use operator::{Associativity, OperationOrder, Operator};
pub use parse::parse;
pub use token::Token;
pub use tokenize::tokenize;

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;
    use rstest::*;

    fn tokens_to_str(tokens: &[token::Token]) -> String {
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
    #[case::evaluate_floor("1 + floor(11.5 + 10.2)", "22.0")]
    #[case::evaluate_ceil("2 - ceil((10 ** 2) / 33)", "-2.0")]
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
}
