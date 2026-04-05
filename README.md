# calculator-rs

Calculator capable of handling arbitrarily large numbers, trading speed for precision.

Parses infix string via the [Shunting yard](https://en.wikipedia.org/wiki/Shunting_yard_algorithm) algorithm, which is then evaluated and returned as custom `Number` type.

Examples:

```rust
// Order of operations
let result = parse_expression("3 + 4 * 2 / (1 - 5)").unwrap();
println!("{result}"); // Number::Int(1)

// Fractions
let result = parse_expression("1 / 2").unwrap();
println!("{result}"); // Number::Decimal(0.5)

// Exponentiation
let result = parse_expression("2 ^ 3").unwrap();
println!("{result}"); // Number::Int(8)

// Very large integers
let result = parse_expression("340282366920938463463374607431768211455 * 137").unwrap();
println!("{result}"); // Number::Int(46618684268168569494482321218152244969335)

// Very large decimals
let result = parse_expression("340282366920938463463374607431768211455 * 137.3367").unwrap();
println!("{result}"); // Number::Decimal(46733257341110849475130439448474521326131.8985)
```
