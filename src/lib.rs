//! `calcinum` is an expression evaluator and arbitrary-precision numeric system supporting
//! integers, decimals, and binary-aware arithmetic.
//!
//! It provides:

//! - [`calcinum::eval`](crate::eval)
//!   Evaluate expressions directly
//!
//! - [`calcinum::Number`](crate::Number)
//!   Work with arbitrary numeric values
//!
//! - [`calcinum::Calculator`](crate::Calculator)
//!   Stateful calculator-style interface
//!
//! # Important Info
//!
//! - Uses `C`/`Rust`-style operator precedence, with added support for exponentiation (`**`).
//! - Arithmetic operators preserve decimal values.
//!   - `0.1 + 0.2 = 0.3`
//!   - `2 - 1.1 = 0.9`
//!   - `1 / 2 = 0.5`
//! - Bitwise operators operate on integers.
//!   - **Operands are coerced into integers before the operation.**
//!   - `2.2 << 2 = 8` (coerced into `2 << 2`)
//!
//! See [operations](#operators) for a full order of operations list.
//!
//! # CLI
//!
//! | Argument         | Shorthand | Description                                                                                                                             |
//! | ---------------- | --------- | --------------------------------------------------------------------------------------------------------------------------------------- |
//! |  *(none)*        |           | Enter [shell mode](#shell-mode)                                                                                 |
//! | `'<expression>'` |          | Evaluate expression ([command mode](#command-mode)) |
//! | `--version`      | `-v`     | Print version |                                                                      |
//!
//! ## Command Mode
//!
//! ```shell
//! $ calcinum --version
//! x.x.x
//! $ calcinum -v
//! x.x.x
//! $ calcinum '2 + 2'
//! 4
//! $ calcinum '2 + (10 / 5)'
//! 4
//! $ calcinum 'abs(-10)'
//! 10
//! $ calcinum '!abs(-10)'
//! -11
//! ```
//!
//! ## Shell Mode
//!
//! Run with no arguments.
//!
//! ```shell
//! $ calcinum
//! ```
//!
//! <img width="518" height="853" alt="calcinum_demo" src="https://github.com/user-attachments/assets/d9823c27-37ae-4027-a9cd-a47446bd805d" />
//!
//! # Operators
//!
//! Order of operations.
//!
//! | Operator | Operation      | Precedence  | Arity  | Associativity |
//! | -------- | -------------- | ----------- | ------ | ------------- |
//! | `-`      | Negation       | 8 (highest) | Unary  | Right         |
//! | `!`      | Bitwise NOT    | 8           | Unary  | Right         |
//! | `**`     | Exponentiation | 7           | Binary | Right         |
//! | `*`      | Multiplication | 6           | Binary | Left          |
//! | `/`      | Division       | 6           | Binary | Left          |
//! | `%`      | Remainder      | 6           | Binary | Left          |
//! | `+`      | Addition       | 5           | Binary | Left          |
//! | `-`      | Subtraction    | 5           | Binary | Left          |
//! | `<<`     | Shift Left     | 4           | Binary | Left          |
//! | `>>`     | Shift Right    | 4           | Binary | Left          |
//! | `&`      | Bitwise AND    | 3           | Binary | Left          |
//! | `^`      | Bitwise XOR    | 2           | Binary | Left          |
//! | `\|`     | Bitwise OR     | 1 (lowest)  | Binary | Left          |
//!
//! # Functions
//!
//! Functions are called using standard syntax:
//!
//! `abs(1 + ceil(100 / 33) - (12 + 13)) / 2`
//!
//! | Function | Description |
//! |----------|-------------|
//! | `abs`    | Absolute value |
//! | `floor`  | Round down |
//! | `ceil`   | Round up |
//! | `round`  | Half-even rounding |
//! | `sin`    | Sine (radians) |
//! | `cos`    | Cosine (radians) |
//! | `tan`    | Tangent (radians) |
//! | `sinh`   | Hyperbolic sine function |
//! | `cosh`   | Hyperbolic cosine function |
//! | `tanh`   | Hyperbolic tangent function |
//! | `rad`    | Radians conversion function |
//!
//! # Constants
//!
//! | Name | Description | Value |
//! |------|-------------|-------|
//! | `pi` | π constant  | `3.1415926535897932383` |
//!
//! # CLI Formatting
//!
//! The CLI supports inline format specifiers using the syntax:
//!
//! ```text
//! :<spec>
//! ```
//!
//! A format specifier controls how a value is displayed (base, padding,
//! width, grouping, etc.).
//!
//! ## Grammar
//!
//! ```text
//! :<zero_pad?> <width?> <kind> <group?>
//! ```
//!
//! | Component   | Required | Description                                      |
//! |-------------|----------|--------------------------------------------------|
//! | `0`         | No       | Enable zero-padding (only applies if width set)  |
//! | `width`     | No       | Minimum output width                             |
//! | `kind`      | Yes      | Output format (`b`, `x`, etc.)                   |
//! | `group`     | No       | Group digits in chunks of N                      |
//!
//! ## Kinds
//!
//! Kinds are case sensitive.
//!
//! | Kind | Description        |
//! |------|--------------------|
//! | `b`  | Binary             |
//! | `x`  | Hex (lowercase)    |
//! | `X`  | Hex (uppercase)    |
//! | `B`  | Base64             |
//!
//! ## Examples
//!
//! ```text
//! 101 :b4       → 0110 0101
//! 11110000 :b4  → 1010 1001 1000 0110 0111 0000
//! ```
//!
//! ## Grouping
//!
//! Grouping splits the output into chunks of N characters.
//!
//! If the output length is not a multiple of N, it is automatically left-padded
//! with `0`s until it is.
//!
//! Grouping is then applied from left to right.
//!
//! ```text
//! 11110000 :b4  → 1010 1001 1000 0110 0111 0000
//! ```
//!
//! ## Notes
//!
//! - Grouping is applied after conversion and padding.
//! - Grouping may introduce additional zero-padding.
//! - `kind` is required whenever `:` is present.
//! - Zero-padding (`0`) is ignored if no width is provided:
//! - Grouping is applied **after** padding.
//!
//! ```text
//! :0b → same as :b
//! ```
//!
//!

mod ast;
mod calculator;
mod number;

pub use bigdecimal;
pub use calculator::*;
pub use num_bigint;
pub use number::{Formatting, Number, NumberOrder, ToNumber, error::NumberError};

/// Evaluates expression.
///
/// ```rust
/// use calcinum::{eval, Number};
///
/// assert_eq!(eval("1+1"), Ok(Number::from(2)));
/// ```
pub fn eval(expression: &str) -> Result<Number, CalculatorError> {
    let tokens = ast::tokenize(expression)?;
    let rpn_tokens = ast::parse(tokens)?;
    let result = ast::eval(rpn_tokens)?;
    Ok(result)
}

/// This method returns a vec of
/// available functions for use within the CLI.
#[doc(hidden)]
pub fn cli_functions() -> Vec<String> {
    let mut all = vec![];
    all.extend(ast::Function::variants_debug());
    all
}

/// This method returns a vec of
/// available operators for use within the CLI.
#[doc(hidden)]
pub fn cli_operators() -> Vec<String> {
    let mut all = vec![];
    all.extend(ast::Unary::variants_debug());
    all.extend(ast::Binary::variants_debug());
    all
}

/// This method returns a vec of
/// available constants for use within the CLI.
#[doc(hidden)]
pub fn cli_constants() -> Vec<String> {
    let mut all = vec![];
    all.extend(ast::Constant::variants_debug());
    all
}
