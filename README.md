# calcinum

[![Crates.io](https://img.shields.io/crates/v/calcinum.svg)](https://crates.io/crates/calcinum) [![docs.rs](https://img.shields.io/docsrs/calcinum?style=flat-square)](https://docs.rs/calcinum/latest/calcinum/)

`calcinum` is an expression evaluator and arbitrary-precision numeric system supporting integers, decimals, and binary-aware arithmetic. In addition to a CLI, it provides both a high-level calculator interface and a low-level Number type for direct manipulation.

# Getting Started

Please see [here to read more about our design decisions](#design), including operators, order of operations, expression functions, expression constants, and more.

## Library

Please see [Library Usage](#library-usage) for examples

- [`calcinum::eval`](#evaluate-expressions) : Evaluates expressions while correctly handling operator precedence without any bells or whistles
- [`calcinum::Number`](#number) : Work with arbitrary numeric values with support for arithmetic, bitwise operations, and more
- [`calcinum::Calculator`](#calculator) : Traditional calculator behavior - simulate pressing buttons or entering expressions - it evaluates expressions while correctly handling operator precedence

## CLI

Please see [CLI Usage](#cli-usage) for examples.

| Argument         | Shorthand | Description                                                                                                                             |
| ---------------- | --------- | --------------------------------------------------------------------------------------------------------------------------------------- |
|                  |           | Provide no arguments to enter [shell mode](#shell-mode)                                                                                 |
| `'<expression>'` |           | Provide an expression enclosed in quotes (**single quotes recommended**) for instant evaluation - used in [command mode](#command-mode) |
| `--version`      | `-v`      | Display current version - used in [command mode](#command-mode)                                                                         |

---

# Design

## Important Info

- We use `C`/`Rust`-style operator precedence, with added support for exponentiation (`**`). Please see [here for more info on order of operations](#operators)
- Parentheses (`(`, `)`) are considered control tokens and do not participate in precedence.
- Arithmetic operators (`+`, `-` <sub>(subtraction)</sub>, `*`, `/`, `%`, `**`, `-` <sub>(negation)</sub>) preserve decimal values.
  - `0.1 + 0.2 = 0.3`
  - `2 - 1.1 = 0.9`
  - `1 / 2 = 0.5`
- Bitwise operators (`&`, `|`, `^`, `<<`, `>>`, `!`) operate on integers. **Operands are coerced into integers before the operation.**
  - `2.2 << 2 = 8` (coerced into `2 << 2`)

## Operators

| Operator | Operation      | Precedence  | Arity  | Associativity |
| -------- | -------------- | ----------- | ------ | ------------- |
| `-`      | Negation       | 8 (highest) | Unary  | Right         |
| `!`      | Bitwise NOT    | 8           | Unary  | Right         |
| `**`     | Exponentiation | 7           | Binary | Right         |
| `*`      | Multiplication | 6           | Binary | Left          |
| `/`      | Division       | 6           | Binary | Left          |
| `%`      | Remainder      | 6           | Binary | Left          |
| `+`      | Addition       | 5           | Binary | Left          |
| `-`      | Subtraction    | 5           | Binary | Left          |
| `<<`     | Shift Left     | 4           | Binary | Left          |
| `>>`     | Shift Right    | 4           | Binary | Left          |
| `&`      | Bitwise AND    | 3           | Binary | Left          |
| `^`      | Bitwise XOR    | 2           | Binary | Left          |
| `\|`     | Bitwise OR     | 1 (lowest)  | Binary | Left          |

## Functions

You can provide functions within an expression. To call a function, type the function name, followed by an open parentheses, then the expression you'd like to evaluate, and finally a closing parentheses.

For example: `abs(1 + ceil(100 / 33) - (12 + 13)) / 2`

| Function | Definition                                                                                                                                                                             |
| -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `abs`    | Non-negative distance of a number from zero.                                                                                                                                           |
| `floor`  | Greatest integer less than or equal to a given number.                                                                                                                                 |
| `ceil`   | Smallest integer greater than or equal to a given number.                                                                                                                              |
| `sin`    | Sine function. Computes the unit-circle y-coordinate for a given angle in radians.                                                                                                     |
| `tan`    | Tangent function. Computes the unit-circle y/x ratio for a given angle in radians.                                                                                                     |
| `round`  | Rounds a number to the nearest integer value (0 decimal places). If the value is equidistant between two integers, it is rounded toward the nearest even integer (half-even rounding). |

## Constants

You can use constants within expressions. We simply replace the constant with its value.

| Constant | Definition                                                  | Value                   |
| -------- | ----------------------------------------------------------- | ----------------------- |
| `pi`     | Mathematical constant π (pi). Default precision is 64-bits. | `3.1415926535897932383` |

# CLI Usage

## Formatting

The formatting delimiter is a colon `:` and must be placed at the end of the line. Any expression to the right of the format delimiter will be treated as formatting syntax!

| Output        | Syntax                      | Example                                                                  |
| ------------- | --------------------------- | ------------------------------------------------------------------------ |
| Binary string | `:b<separator><group by n>` | - `:b 4` -> `1001 1010 1110 0101` <br> - `:b_4` -> `1001_1010_1110_0101` |

## Command Mode

Command mode operates as a standard CLI interface, accepting a command and writing its output to the terminal.

```
$ calcinum --version
x.x.x
$ calcinum -v
x.x.x
$ calcinum '2 + 2'
4
$ calcinum '2 + (10 / 5)'
4
$ calcinum 'abs(-10)'
10
$ calcinum '!abs(-10)'
-11
```

## Shell Mode

Shell mode behaves like a REPL. Previous results can be interpolated into new expressions using `@N`, where `N` denotes the line number of the referenced result.

Shell mode comes with a few extra commands, just type `commands` to view them.

<img width="552" height="803" alt="Screenshot 2026-04-10 at 7 38 53 PM" src="https://github.com/user-attachments/assets/5144295a-400f-432d-80cf-ccf8206c7fff" />

<details>
  <summary>Click to view raw text of screenshot above</summary>

```
$ calcinum

Commands:

clear         clears the screen
reset         resets history
exit          exits the repl
history       prints available history
commands      prints this message

[@1]> 1+1
2
[@2]> floor(112.134)
112
[@3]> 3*3-(@1+10)
-3
[@4]> abs(@3)
3
[@5]> @10+1
Line '10' does not exist.
[@6]> history
@1
  expression = '1+1'
  result     = '2'
@2
  expression = 'floor(112.134)'
  result     = '112'
@3
  expression = '3*3-(2+10)'
  result     = '-3'
@4
  expression = 'abs(-3)'
  result     = '3'
@5
  expression = '@10+1'
  result     = 'ERROR'
[@6]> reset
--- HISTORY RESET ---
[@1]>
```

</details>

# Library Usage

## Number

**Create `Number::Int` where calculation produces `Number::Decimal`**

```rust
use calcinum::Number;

let a = Number::Int(1.into());
let b = Number::Int(2.into());
let result = a / b;
println!("{result:?}"); // Number::Decimal(0.5)
```

**Create `Number::Decimal`**

```rust
use calcinum::Number;

let x = Number::Int(10.into());
let y = Number::from_f64(2.2).unwrap(); // Will be Number::Decimal(2.2)
// Also acceptable:
let y = Number::from_str("2.2").unwrap(); // Will be Number::Decimal(2.2)

let mut result = x / y;
result.set_scale(11); // Truncate scale, otherwise scale will be ~100 digits
println!("{result:?}"); // Number::Decimal(4.54545454545)
```

**Convenience**

```rust
use calcinum::{Number, ToNumber};

12.to_number(); // Number::Int(12)
1.1.to_number(); // Number::Decimal(1.1)
u128::MAX.to_number(); // Number::Int(340282366920938463463374607431768211455)
i128::MIN.to_number(); // Number::Int(-170141183460469231731687303715884105728)
```

**Converting to, and from, binary strings**

`Number::Int`

We format `Number::Int` as traditional binary - simply convert to a binary string. **To parse a binary string into a `Number::Int` we expect:**

- The binary string to start with `0b` or `-0b` for negative numbers

```rust
let i = 123.to_number(); // Number::Int(123)
let bs = format!("{i:b}"); // "1111011"
// Parse binary string back into `Number` - needs "0b" prefix.
let s = format!("0b{bs}");
let n = s.parse::<Number>().unwrap(); // Number::Int(123)
```

`Number::Decimal`

We format `Number::Decimal` by literally converting the integer part and fractional part into standalone binary strings, then joining them with a decimal. **To parse a binary string into a `Number::Decimal` we expect:**

- The binary string to start with `0b` or `-0b` for negative numbers
- A decimal separating the integer part from the fractional part

```rust
let i = 382.619.to_number(); // Number::Decimal(382.619)
let bs = format!("{i:b}"); // "101111110.1001101011"
// Parse binary string back into `Number` - needs "0b" prefix.
let s = format!("0b{bs}");
let n = s.parse::<Number>().unwrap(); // Number::Decimal(382.619)
```

## Calculator

**You can simulate pressing keys on a calculator.**

```rust
use calcinum::{Calculator, Key};

let mut c = Calculator::new();
c.press(Key::Two); // 2
c.press(Key::Add); // +
c.press(Key::Two); // 2

// View current expression
println!("{}", c.expression()); // "2+2"

// Evaluate/calculate expression.
let result = c.calculate().unwrap();
println!("{result:?}"); // Number::Int(4)

// We store the result as the new expression
// expression so you can continue using
// it in calculations.
println!("{}", c.expression()); // "4"
c.press(Key::Add);
c.press(Key::Four);
println!("{}", c.expression()); // "4+4"

// Get new result
let result = c.calculate().unwrap();
println!("{result:?}"); // Number::Int(8)

// Clear calculator (clears stored expression)
c.clear();
```

**Create decimals using key press**

```rust
use calcinum::{Calculator, Key};

let mut c = Calculator::new();
c.press(Key::One);
c.press(Key::Period);
c.press(Key::One);
println!("{}", c.expression()); // "1.1"
```

**Create instance with expression in one line**

```rust
use calcinum::Calculator;

let mut c = Calculator::new_with_expression("(2+8)/2");
let result = c.calculate().unwrap();
println!("{result:?}"); // Number::Int(5)
```

**Append expression to current expression**

```rust
use calcinum::Calculator;

let mut c = Calculator::new();
c.append("(1+1)");
// Appended to current expression; does not replace it.
c.append("*2/12-5*99");
println!("{}", c.expression()); // "(1+1)*2/12-5*99"
```

**You can combine `append("...")` with key `press(Key::_)` in any order**

```rust
use calcinum::{Calculator, Key};

let mut c = Calculator::new();

// Build expression
c.press(Key::ParenthesesOpen);
c.press(Key::Two);
c.press(Key::Add);
c.append("8)/2");

// View current expression
println!("{}", c.expression()); // "(2+8)/2"

let result = c.calculate().unwrap();
println!("{result:?}"); // Number::Int(5)
```

## Evaluate Expressions

You can acheive the same thing via `Calculator`, granted it will be more lines of code, hence the helper.

```rust
use calcinum::Calculator;

// Order of operations
let result = calcinum::eval("3 + 4 * 2 / (1 - 5)").unwrap();
println!("{result:?}"); // Number::Int(1)
// ~~ Equivalent to ~~
let mut c = Calculator::new_with_expression("3 + 4 * 2 / (1 - 5)");
let result = c.calculate().unwrap();
println!("{result:?}"); // Number::Int(1)

// Fractions
let result = calcinum::eval("1 / 2").unwrap();
println!("{result:?}"); // Number::Decimal(0.5)

// Exponentiation
let result = calcinum::eval("2 ^ 3").unwrap();
println!("{result:?}"); // Number::Int(8)

// Very large integers
let result = calcinum::eval("340282366920938463463374607431768211455 * 137").unwrap();
println!("{result:?}"); // Number::Int(46618684268168569494482321218152244969335)

// Very large decimals
let result = calcinum::eval("340282366920938463463374607431768211455 * 137.3367").unwrap();
println!("{result:?}"); // Number::Decimal(46733257341110849475130439448474521326131.8985)
```
