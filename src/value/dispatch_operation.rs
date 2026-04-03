/// Dispatch an operation across the variants of a value.
///
/// Parameters:
///
/// - `$lhs`: expression results in either `mut Value` or `&mut Value`
/// - `$rhs`: expression results in `mut Value`
/// - `$n`: this identifier will be assigned with the value and type of the min-matching-value of `$lhs`
/// - `$op`: this should be a closure which captures `$n` and accepts a single parameter `rhs` of the matching type.
///   It should either return a Value or mutate `$n`.
///
/// ## Alternates
///
/// `INTS:` prefix dispatches only across the integers. This alternate returns the `Result` typedef, not a bare `Value`.
macro_rules! dispatch_operation {
    ($lhs:expr, $rhs:expr, $n:ident, $op:expr) => {{
        $lhs.match_orders(&mut $rhs);
        debug_assert_eq!(
            $lhs.order(),
            $rhs.order(),
            "orders must match after match_orders"
        );
        match $lhs {
            Value::UnsignedInt($n) => {
                let rhs = u128::try_from($rhs).expect("orders must match");
                $op(rhs)
            }
            Value::UnsignedBigInt($n) => {
                let rhs = num_bigint::BigUint::try_from($rhs).expect("orders must match");
                $op(rhs)
            }
            Value::SignedInt($n) => {
                let rhs = i128::try_from($rhs).expect("orders must match");
                $op(rhs)
            }
            Value::SignedBigInt($n) => {
                let rhs = num_bigint::BigInt::try_from($rhs).expect("orders must match");
                $op(rhs)
            }
            Value::Float($n) => {
                let rhs = f64::try_from($rhs).expect("orders must match");
                $op(rhs)
            }
        }
    }};
    // INTS variant for operations returning Result<Value, Error>
    (INTS: $lhs:expr, $rhs:expr, $n:ident, $op:expr) => {{
        $lhs.match_orders(&mut $rhs);
        debug_assert_eq!(
            $lhs.order(),
            $rhs.order(),
            "orders must match after match_orders"
        );

        match $lhs {
            Value::UnsignedInt(n) => {
                let rhs = u64::try_from($rhs).expect("orders must match");
                let $n = n;
                Ok($op(rhs))
            }
            Value::UnsignedBigInt(n) => {
                let rhs = u128::try_from($rhs).expect("orders must match");
                let $n = n;
                Ok($op(rhs))
            }
            Value::SignedInt(n) => {
                let rhs = i64::try_from($rhs).expect("orders must match");
                let $n = n;
                Ok($op(rhs))
            }
            Value::SignedBigInt(n) => {
                let rhs = i128::try_from($rhs).expect("orders must match");
                let $n = n;
                Ok($op(rhs))
            }
            Value::Float(_) => Err(Error::ImproperlyFloat),
        }
    }};
}

pub(crate) use dispatch_operation;
