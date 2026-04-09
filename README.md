# calcinum

Calculator capable of handling arbitrarily large numbers, trading speed for precision - we use [`BigInt`](https://github.com/rust-num/num-bigint) and [`BigDecimal`](https://github.com/akubera/bigdecimal-rs) under the hood.

Parses infix string via the [shunting yard](https://en.wikipedia.org/wiki/Shunting_yard_algorithm) algorithm, which is then evaluated and returned as custom `Number` type.

# Important Info

- We use `C`/`Rust`-style operator precedence, with added support for exponentiation (`**`).
- Parentheses (`(`, `)`) are considered control tokens and do not participate in precedence.
- Arithmetic operators (`+`, `-` <sub>(subtraction)</sub>, `*`, `/`, `%`, `**`, `-` <sub>(negation)</sub>) preserve decimal values.
  - e.g., `0.1 + 0.2 = 0.3`, `2 - 1.1 = 0.9`, `1 / 2 = 0.5`
- Bitwise operators (`&`, `|`, `^`, `<<`, `>>`, `!`) operate on integers. Operands are coerced into integers before the operation.
  - e.g., `2.2 << 2 = 8` (coerced into `2 << 2`)

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
c.press(Key::ParenthesisOpen);
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
