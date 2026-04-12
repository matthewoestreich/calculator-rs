mod constant;
mod eval;
mod function;
mod operator;
mod parse;
mod token;
mod tokenize;

pub mod error;

pub use constant::Constant;
pub use eval::eval;
pub use function::Function;
pub use operator::{Associativity, Binary, Operator, Unary};
pub use parse::parse;
pub use token::Token;
pub use tokenize::tokenize;

#[cfg(test)]
mod test {
    use super::*;

    pub fn tokens_to_str(tokens: &[token::Token]) -> String {
        tokens
            .iter()
            .fold(String::new(), |acc, x| format!("{acc} {x}"))
            .trim()
            .to_string()
    }
}
