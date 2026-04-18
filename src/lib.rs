//! `calcinum` is an expression evaluator and arbitrary-precision numeric system supporting
//! integers, decimals, and binary-aware arithmetic. In addition to a CLI, it provides both a
//! high-level calculator interface and a low-level Number type for direct manipulation.
//!
//! # Important Info
//!
//! - Please see [here for more info on order of operations](#operators)
//! - We use `C`/`Rust`-style operator precedence, with added support for exponentiation (`**`).
//! - Arithmetic operators (`+`, `-` <sub>(subtraction)</sub>, `*`, `/`, `%`, `**`, `-` <sub>(negation)</sub>) preserve decimal values.
//!   - `0.1 + 0.2 = 0.3`
//!   - `2 - 1.1 = 0.9`
//!   - `1 / 2 = 0.5`
//! - Bitwise operators (`&`, `|`, `^`, `<<`, `>>`, `!`) operate on integers. **Operands are coerced into integers before the operation.**
//!   - `2.2 << 2 = 8` (coerced into `2 << 2`)
//!
//! # Getting Started
//!
//! - [`calcinum::eval`](crate::eval) : Evaluates expressions while correctly handling operator precedence without any bells or whistles
//! - [`calcinum::Number`](crate::Number) : Work with arbitrary numeric values with support for arithmetic, bitwise operations, and more
//! - [`calcinum::Calculator`](crate::Calculator) : Traditional calculator behavior - simulate pressing buttons or entering expressions - it evaluates expressions while correctly handling operator precedence
//!
//! # CLI
//!
//! | Argument         | Shorthand | Description                                                                                                                             |
//! | ---------------- | --------- | --------------------------------------------------------------------------------------------------------------------------------------- |
//! |                  |           | Provide no arguments to enter [shell mode](#shell-mode)                                                                                 |
//! | `'<expression>'` |           | Provide an expression enclosed in quotes (**single quotes recommended**) for instant evaluation - used in [command mode](#command-mode) |
//! | `--version`      | `-v`      | Display current version - used in [command mode](#command-mode)                                                                         |
//!
//! ## Operators
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
//! ## Functions
//!
//! You can provide functions within an expression. To call a function, type the function name, followed by an open parentheses, then the expression you'd like to evaluate, and finally a closing parentheses.
//!
//! For example: `abs(1 + ceil(100 / 33) - (12 + 13)) / 2`
//!
//! | Function | Definition                                                                                                                                                                             |
//! | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
//! | `abs`    | Non-negative distance of a number from zero.                                                                                                                                           |
//! | `floor`  | Greatest integer less than or equal to a given number.                                                                                                                                 |
//! | `ceil`   | Smallest integer greater than or equal to a given number.                                                                                                                              |
//! | `sin`    | Sine function. Computes the unit-circle y-coordinate for a given angle in radians.                                                                                                     |
//! | `cos`    | Cosine function. Computes the unit-circle x-coordinate for a given angle in radians.                                                                                                   |
//! | `tan`    | Tangent function. Computes the unit-circle y/x ratio for a given angle in radians.                                                                                                     |
//! | `round`  | Rounds a number to the nearest integer value (0 decimal places). If the value is equidistant between two integers, it is rounded toward the nearest even integer (half-even rounding). |
//!
//! ## Constants
//!
//! You can use constants within expressions. We simply replace the constant with its value.
//!
//! | Constant | Definition                                                  | Value                   |
//! | -------- | ----------------------------------------------------------- | ----------------------- |
//! | `pi`     | Mathematical constant π (pi). Default precision is 64-bits. | `3.1415926535897932383` |
//!
//! ## Formatting
//!
//! **For the CLI**, the formatting delimiter is a colon `:` and must be placed at the end of the line. Any expression to the right of the format delimiter will be treated as formatting syntax!
//!
//! | Output        | Syntax                      | Example                                                                  |
//! | ------------- | --------------------------- | ------------------------------------------------------------------------ |
//! | Binary string | `:b<separator><group by n>` | - `:b 4` -> `1001 1010 1110 0101` <br> - `:b_4` -> `1001_1010_1110_0101` |
//!
//! ## Command Mode
//!
//! Command mode operates as a standard CLI interface, accepting a command and writing its output to the terminal.
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
//! **To enter shell mode do not provide any arguments to the CLI**
//!
//! ```shell
//! $ calcinum
//! ```
//!
//! Shell mode behaves like a REPL. Previous results can be interpolated into new expressions using `@N`, where `N` denotes the line number of the referenced result.
//!
//! Shell mode comes with a few extra commands, just type `commands` to view them.
//!
//! <img width="552" height="803" alt="Screenshot 2026-04-10 at 7 38 53 PM" src="https://github.com/user-attachments/assets/5144295a-400f-432d-80cf-ccf8206c7fff" />
//!
//! <details>
//!   <summary>Click to view raw text of screenshot above</summary>
//!
//! ```shell
//! $ calcinum
//!
//! Commands:
//!
//! clear         clears the screen
//! reset         resets history
//! exit          exits the repl
//! history       prints available history
//! commands      prints this message
//!
//! [@1]> 1+1
//! 2
//! [@2]> floor(112.134)
//! 112
//! [@3]> 3*3-(@1+10)
//! -3
//! [@4]> abs(@3)
//! 3
//! [@5]> @10+1
//! Line '10' does not exist.
//! [@6]> history
//! @1
//!   expression = '1+1'
//!   result     = '2'
//! @2
//!   expression = 'floor(112.134)'
//!   result     = '112'
//! @3
//!   expression = '3*3-(2+10)'
//!   result     = '-3'
//! @4
//!   expression = 'abs(-3)'
//!   result     = '3'
//! @5
//!   expression = '@10+1'
//!   result     = 'ERROR'
//! [@6]> reset
//! --- HISTORY RESET ---
//! [@1]>
//! ```
//!
//! </details>
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
