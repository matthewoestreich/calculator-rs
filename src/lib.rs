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
//! - No implicit multiplication
//! - Floating point operations can compound lossily. **No special efforts are made to guard against this kind of error**.
//!   - `$ calcinum 'sin(rad(45)) - (sqrt(2) / 2)'` → `-0.0000000000000000000676424327732749209...`
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
//! | `sinh`   | Hyperbolic sine (radians) |
//! | `cosh`   | Hyperbolic cosine (radians) |
//! | `tanh`   | Hyperbolic tangent (radians) |
//! | `rad`    | Radians conversion (degrees to radians) |
//! | `sqrt`   | Square root |
//!
//! # Constants
//!
//! | Name | Description | Value |
//! |------|-------------|-------|
//! | `pi` | π constant  | `3.1415926535897932383` |
//! | `e`  | Euler’s number | `2.7182818284590452352` |
//!
//! # Formatting
//!
//! Both the CLI and library use the same formatting spec.
//!
//! ## Library Usage
//!
//! The same formatting specifier used by the CLI is also supported in the library.
//!
//! Instead of prefixing with `:`, the spec is passed directly as a string:
//!
//! ```rust
//! use calcinum::Number;
//! let n = Number::from(1);
//! let formatted = n.format("036b4");
//! ```
//!
//! This is equivalent to the CLI usage:
//!
//! ```text
//! 1 :036b4
//! ```
//!
//! ### Notes
//!
//! - The format string follows the exact same grammar:
//!
//! ```text
//! <zero_pad?> <width?> <kind> <group?>
//! ```
//!
//! - The leading `:` is **only required in CLI input**, not in the library API.
//! - All semantics (width, padding, grouping, etc.) are identical between CLI and library usage.
//!
//! ## CLI Usage
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
//! ### Grammar
//!
//! ```text
//! :<zero_pad?> <width?> <kind> <group?>
//! ```
//!
//! | Component   | Required | Description                                      |
//! |-------------|----------|--------------------------------------------------|
//! | `0`         | No       | Enable zero-padding (only applies if width set)  |
//! | `width`     | No       | Minimum total digit width (excludes `-` and `.`) |
//! | `kind`      | Yes      | Output format (`b`, `x`, etc.)                   |
//! | `group`     | No       | Group digits in chunks of N                      |
//!
//! ### Kinds
//!
//! Kinds are case sensitive.
//!
//! **Note:** if `N` kind is specified everything else is ignored.
//!
//! | Kind | Description        |
//! |------|--------------------|
//! | `b`  | Binary             |
//! | `x`  | Hex (lowercase)    |
//! | `X`  | Hex (uppercase)    |
//! | `B`  | Base64             |
//! | `N`  | `Number`           |
//!
//! ### Width & Padding
//!
//! - When used with `group`, `width` specifies the **minimum total number of digits** in the output
//!   (excluding sign and decimal point).
//! - If the value has fewer digits than `width`, it is padded on the **left side
//!   of the integer portion**.
//! - The fractional portion is never padded due to width alone.
//!
//! ```text
//! 123.123 :024b
//! → 00000000001111011.1111011
//!
//! 123.123 :024b4
//! → 0000 0000 0111 1011.0111 1011
//! ```
//!
//! ### Grouping
//!
//! Grouping splits digits into chunks of size `N`.
//!
//! - Grouping is applied **after padding**.
//! - Grouping operates on a **group-aligned representation**, meaning:
//!   - The integer and fractional parts are each aligned to a multiple of `N`.
//!   - This alignment may introduce additional leading zeros on the integer side.
//!   - The fractional side is only expanded to satisfy grouping alignment,
//!     not width.
//!
//! ```text
//! 123.123 :024b4
//! → 0000 0000 0111 1011.0111 1011
//! ```
//!
//! ### Notes
//!
//! - `kind` is required whenever `:` is present.
//! - Zero-padding (`0`) is ignored if no width is provided:
//!
//! ```text
//! :0b → same as :b
//! ```
//!
//! - Grouping does **not** change the numeric value.
//! - When used with `group`, `width` is treated as a **minimum**, not an exact size.
//!

mod ast;
mod calculator;
mod number;

pub use bigdecimal;
pub use calculator::*;
pub use num_bigint;
pub use number::{Number, NumberOrder, ToNumber, error::NumberError};

use number::fmt as numfmt;
use varienum::VariantsVec;

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
pub fn cli_functions() -> Vec<(&'static str, &'static str)> {
    let mut all = Vec::<(&'static str, &'static str)>::new();
    all.extend(ast::Function::variants_desc());
    all
}

/// This method returns a vec of
/// available operators for use within the CLI.
#[doc(hidden)]
pub fn cli_operators() -> Vec<(&'static str, &'static str)> {
    let mut all = Vec::<(&'static str, &'static str)>::new();
    all.extend(ast::Unary::variants_desc());
    all.extend(ast::Binary::variants_desc());
    all
}

/// This method returns a vec of
/// available constants for use within the CLI.
#[doc(hidden)]
pub fn cli_constants() -> Vec<(&'static str, &'static str)> {
    let mut all = Vec::<(&'static str, &'static str)>::new();
    all.extend(ast::Constant::variants_desc());
    all
}

/// This method returns a vec of
/// available formatting Kind for use within the CLI.
#[doc(hidden)]
pub fn cli_format_kinds() -> Vec<(&'static str, &'static str)> {
    let mut all = Vec::<(&'static str, &'static str)>::new();
    all.extend(numfmt::Kind::variants_desc());
    all
}
