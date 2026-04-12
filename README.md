# calcinum

[![Crates.io](https://img.shields.io/crates/v/calcinum.svg)](https://crates.io/crates/calcinum) [![docs.rs](https://img.shields.io/docsrs/calcinum?style=flat-square)](https://docs.rs/calcinum/latest/calcinum/)

Calculator capable of handling arbitrarily large numbers, trading speed for precision - we use [`BigInt`](https://github.com/rust-num/num-bigint) and [`BigDecimal`](https://github.com/akubera/bigdecimal-rs) under the hood.

Parses infix string via the [shunting yard](https://en.wikipedia.org/wiki/Shunting_yard_algorithm) algorithm, which is then evaluated and returned as custom `Number` type.

# Important Info

- We use `C`/`Rust`-style operator precedence, with added support for exponentiation (`**`).
- Parentheses (`(`, `)`) are considered control tokens and do not participate in precedence.
- Arithmetic operators (`+`, `-` <sub>(subtraction)</sub>, `*`, `/`, `%`, `**`, `-` <sub>(negation)</sub>) preserve decimal values.
  - `0.1 + 0.2 = 0.3`
  - `2 - 1.1 = 0.9`
  - `1 / 2 = 0.5`
- Bitwise operators (`&`, `|`, `^`, `<<`, `>>`, `!`) operate on integers. **Operands are coerced into integers before the operation.**
  - `2.2 << 2 = 8` (coerced into `2 << 2`)

# Operators

Operators with order of operations.

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

# Functions

You can provide functions within an infix expression.

To call a function, type the function name, followed by an open parentheses, then the expression you'd like to evaluate, and finally a closing parentheses.

For example: `abs(1 + ceil(100 / 33) - (12 + 13)) / 2`

| Function | Definition                                                                         |
| -------- | ---------------------------------------------------------------------------------- |
| `abs`    | Non-negative distance of a number from zero.                                       |
| `floor`  | Greatest integer less than or equal to a given number.                             |
| `ceil`   | Smallest integer greater than or equal to a given number.                          |
| `sin`    | Sine function. Computes the unit-circle y-coordinate for a given angle in radians. |
| `tan`    | Tangent function. Computes the unit-circle y/x ratio for a given angle in radians. |

# Constants

You can use constants within expressions. We simply replace the constant with its value.

| Constant | Definition                                                  | Value                   | Library Usage                                                     |
| -------- | ----------------------------------------------------------- | ----------------------- | ----------------------------------------------------------------- |
| `pi`     | Mathematical constant π (pi). Default precision is 64-bits. | `3.1415926535897932383` | `let precision: usize = 64;`<br>`Number::pi(precision).unwrap();` |

# CLI Usage

| Description                                                                                      | Argument         | Shorthand |
| ------------------------------------------------------------------------------------------------ | ---------------- | --------- |
| Provide no arguments to enter [shell mode](#shell-mode)                                          |                  |           |
| Provide an expression enclosed in quotes (**single quotes reccommended**) for instant evaluation | `'<expression>'` |           |
| Display current version.                                                                         | `--version`      | `-v`      |

## Command Mode

Command mode operates as a standard CLI interface, accepting a command and writing its output to the terminal.

**Command mode examples:**

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

Shell mode behaves like a REPL. Previous results can be interpolated into new expressions using @N, where N denotes the line number of the referenced result.

Shell mode comes with a few extra commands, just type `commands` to view them.

**Shell mode examples:**

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

You can either work with the exposed `Number` enum directly, or use the `Calculator` struct. We recommend using the `Calculator` since it handles order of operations natively.

## Number

Create `Number::Int` where calculation produces `Number::Decimal`

```rust
use calcinum::Number;

let a = Number::Int(1.into());
let b = Number::Int(2.into());
let result = a / b;
println!("{result:?}"); // Number::Decimal(0.5)
```

Create `Number::Decimal`

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

Convenience

```rust
use calcinum::{Number, ToNumber};

12.to_number(); // Number::Int(12)
1.1.to_number(); // Number::Decimal(1.1)
u128::MAX.to_number(); // Number::Int(340282366920938463463374607431768211455)
i128::MIN.to_number(); // Number::Int(-170141183460469231731687303715884105728)
```

## Calculator

### Simulate Key Press

You can simulate pressing keys on a calculator.

```rust
use calcinum::{Calculator, Key};

let mut c = Calculator::new();
c.press(Key::Two); // 2
c.press(Key::Add); // +
c.press(Key::Two); // 2

// View current infix representation
println!("{}", c.infix()); // "2+2"

// Evaluate/calculate expression.
let result = c.calculate().unwrap();
println!("{result:?}"); // Number::Int(4)

// We store the result as the new infix
// expression so you can continue using
// it in calculations.
println!("{}", c.infix()); // "4"
c.press(Key::Add);
c.press(Key::Four);
println!("{}", c.infix()); // "4+4"

// Get new result
let result = c.calculate().unwrap();
println!("{result:?}"); // Number::Int(8)

// Clear calculator (clears stored infix expression)
c.clear();
```

Create decimals using key press

```rust
use calcinum::{Calculator, Key};

let mut c = Calculator::new();
c.press(Key::One);
c.press(Key::Period);
c.press(Key::One);
println!("{}", c.infix()); // "1.1"
```

### Infix Expression

Create instance with infix expression in one line

```rust
use calcinum::Calculator;

let mut c = Calculator::new_with_infix("(2+8)/2");
let result = c.calculate().unwrap();
println!("{result:?}"); // Number::Int(5)
```

Append infix expression to current infix expression.

```rust
use calcinum::Calculator;

let mut c = Calculator::new();
c.expression("(1+1)");
// Appended to current expression; does not replace it.
c.expression("*2/12-5*99");
println!("{}", c.infix()); // "(1+1)*2/12-5*99"
```

You can combine `expression("...")` with key `press(Key::_)` in any order.

```rust
use calcinum::{Calculator, Key};

let mut c = Calculator::new();

// Build infix expression
c.press(Key::ParenthesesOpen);
c.press(Key::Two);
c.press(Key::Add);
c.expression("8)/2");

// View current infix
println!("{}", c.infix()); // "(2+8)/2"

let result = c.calculate().unwrap();
println!("{result:?}"); // Number::Int(5)
```

## Parse Infix Expression Helper

You can acheive the same thing via `Calculator`, granted it will be more lines of code, hence the helper..

```rust
use calcinum::{Calculator, parse_expression};

// Order of operations
let result = parse_expression("3 + 4 * 2 / (1 - 5)").unwrap();
println!("{result:?}"); // Number::Int(1)
// ~~ Equivalent to ~~
let mut c = Calculator::new_with_infix("3 + 4 * 2 / (1 - 5)");
let result = c.calculate().unwrap();
println!("{result:?}"); // Number::Int(1)

// Fractions
let result = parse_expression("1 / 2").unwrap();
println!("{result:?}"); // Number::Decimal(0.5)

// Exponentiation
let result = parse_expression("2 ^ 3").unwrap();
println!("{result:?}"); // Number::Int(8)

// Very large integers
let result = parse_expression("340282366920938463463374607431768211455 * 137").unwrap();
println!("{result:?}"); // Number::Int(46618684268168569494482321218152244969335)

// Very large decimals
let result = parse_expression("340282366920938463463374607431768211455 * 137.3367").unwrap();
println!("{result:?}"); // Number::Decimal(46733257341110849475130439448474521326131.8985)
```
