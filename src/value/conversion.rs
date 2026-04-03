use crate::value::{Order, Value, error::ValueError};
use num_bigint::{BigInt, BigUint};
use std::str::FromStr;

impl From<Order> for Value {
    fn from(order: Order) -> Self {
        match order {
            Order::UnsignedInt => Value::UnsignedInt(u128::default()),
            Order::UnsignedBigInt => Value::UnsignedBigInt(BigUint::default()),
            Order::SignedInt => Value::SignedInt(i128::default()),
            Order::SignedBigInt => Value::SignedBigInt(BigInt::default()),
            Order::Float => Value::Float(f64::default()),
        }
    }
}

// ===============================================================
// From<T>
// ===============================================================

macro_rules! impl_from {
    ($t:ty => $variant:ident) => {
        impl From<$t> for Value {
            fn from(value: $t) -> Self {
                Value::$variant(value as _)
            }
        }
    };
}

impl_from!(u8 => UnsignedInt);
impl_from!(u16 => UnsignedInt);
impl_from!(u32 => UnsignedInt);
impl_from!(u64 => UnsignedInt);
impl_from!(u128 => UnsignedInt);
impl_from!(i8 => SignedInt);
impl_from!(i16 => SignedInt);
impl_from!(i32 => SignedInt);
impl_from!(i64 => SignedInt);
impl_from!(i128 => SignedInt);
impl_from!(f64 => Float);

impl From<BigInt> for Value {
    fn from(value: BigInt) -> Self {
        Value::SignedBigInt(value)
    }
}

impl From<BigUint> for Value {
    fn from(value: BigUint) -> Self {
        Value::UnsignedBigInt(value)
    }
}

// ===============================================================
// TryFrom<T>
// ===============================================================

macro_rules! impl_try_from {
    ($t:ty => $variant:ident) => {
        impl TryFrom<Value> for $t {
            type Error = ValueError;
            fn try_from(value: Value) -> Result<Self, Self::Error> {
                match value {
                    Value::$variant(n) => Ok(n as _),
                    other => Err(ValueError::Converting {
                        from: other,
                        to: String::from(stringify!($t)),
                    }),
                }
            }
        }
        impl TryFrom<&Value> for $t {
            type Error = ValueError;
            fn try_from(value: &Value) -> Result<Self, Self::Error> {
                match value {
                    Value::$variant(n) => Ok(*n as _),
                    other => Err(ValueError::Converting {
                        from: other.clone(),
                        to: String::from(stringify!($t)),
                    }),
                }
            }
        }
        impl<'a> TryFrom<&'a mut Value> for $t {
            type Error = ValueError;
            fn try_from(value: &'a mut Value) -> Result<Self, Self::Error> {
                match value {
                    Value::$variant(n) => Ok(*n as _),
                    other => Err(ValueError::Converting {
                        from: other.clone(),
                        to: String::from(stringify!($t)),
                    }),
                }
            }
        }
    };
}

impl_try_from!(u64 => UnsignedInt);
impl_try_from!(u128 => UnsignedInt);
impl_try_from!(i64 => SignedInt);
impl_try_from!(i128 => SignedInt);
impl_try_from!(f64 => Float);

impl TryFrom<Value> for BigInt {
    type Error = ValueError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::SignedBigInt(n) => Ok(n),
            other => Err(ValueError::Converting {
                from: other.clone(),
                to: String::from("BigInt"),
            }),
        }
    }
}

impl TryFrom<&Value> for BigInt {
    type Error = ValueError;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::SignedBigInt(n) => Ok(n.clone()),
            other => Err(ValueError::Converting {
                from: other.clone(),
                to: String::from("BigInt"),
            }),
        }
    }
}

impl<'a> TryFrom<&'a mut Value> for BigInt {
    type Error = ValueError;
    fn try_from(value: &'a mut Value) -> Result<Self, Self::Error> {
        match value {
            Value::SignedBigInt(n) => Ok(n.clone()),
            other => Err(ValueError::Converting {
                from: other.clone(),
                to: String::from("BigInt"),
            }),
        }
    }
}

impl TryFrom<Value> for BigUint {
    type Error = ValueError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::UnsignedBigInt(n) => Ok(n),
            other => Err(ValueError::Converting {
                from: other.clone(),
                to: String::from("BigUint"),
            }),
        }
    }
}

impl TryFrom<&Value> for BigUint {
    type Error = ValueError;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::UnsignedBigInt(n) => Ok(n.clone()),
            other => Err(ValueError::Converting {
                from: other.clone(),
                to: String::from("BigUint"),
            }),
        }
    }
}

impl<'a> TryFrom<&'a mut Value> for BigUint {
    type Error = ValueError;
    fn try_from(value: &'a mut Value) -> Result<Self, Self::Error> {
        match value {
            Value::UnsignedBigInt(n) => Ok(n.clone()),
            other => Err(ValueError::Converting {
                from: other.clone(),
                to: String::from("BigUint"),
            }),
        }
    }
}

// ===============================================================
// FromStr
// ===============================================================

impl FromStr for Value {
    type Err = ValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(".") {
            return s
                .parse::<f64>()
                .map(Self::Float)
                .map_err(|_| ValueError::Parsing {
                    value: s.to_string(),
                });
        }
        s.parse::<u128>()
            .map(|i| Self::UnsignedInt(i as _))
            .or_else(|_| s.parse::<BigUint>().map(Self::UnsignedBigInt))
            .or_else(|_| s.parse::<i128>().map(Self::SignedInt))
            .or_else(|_| s.parse::<BigInt>().map(Self::SignedBigInt))
            .or_else(|_| s.parse::<f64>().map(Self::Float))
            .map_err(|_| ValueError::Parsing {
                value: s.to_string(),
            })
    }
}

// ===============================================================
// Tests
// ===============================================================

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_from {
        ($name:ident, $t:ty, $val:expr, $variant:ident) => {
            #[test]
            fn $name() {
                let n: $t = $val;
                let value = Value::from(n);
                assert_eq!(value, Value::$variant(n.into()));
            }
        };
    }

    macro_rules! test_try_from {
        ($name:ident, $t:ty, $val:expr, $variant:ident) => {
            #[test]
            fn $name() {
                let value = Value::$variant($val as _);
                let nn: $t =
                    <$t>::try_from(&value).expect(concat!("no ", stringify!($t), " errors"));
                assert_eq!(nn, $val);
                let n: $t = <$t>::try_from(value).expect(concat!("no ", stringify!($t), " errors"));
                assert_eq!(n, $val);
            }
        };
    }

    macro_rules! test_from_str {
        ($name:ident, $str_val:expr => $expected_order:expr) => {
            #[test]
            fn $name() {
                concat!($str_val, "");
                let v = $str_val;
                let r = Value::from_str(v).expect("no error for {v}");
                assert_eq!(
                    r.order(),
                    $expected_order,
                    "[test_from_str] expected {:?} | got {r:?}",
                    $expected_order
                );
            }
        };
    }

    test_from!(from_1, u8, u8::MAX, UnsignedInt);
    test_from!(from_2, u16, u16::MAX, UnsignedInt);
    test_from!(from_3, u32, u32::MAX, UnsignedInt);
    test_from!(from_4, u64, u64::MAX, UnsignedInt);
    test_from!(from_5, u128, u128::MAX, UnsignedBigInt);
    test_from!(from_6, i8, i8::MIN, SignedInt);
    test_from!(from_7, i16, i16::MIN, SignedInt);
    test_from!(from_8, i32, i32::MIN, SignedInt);
    test_from!(from_9, i64, i64::MIN, SignedInt);
    test_from!(from_10, i128, i128::MIN, SignedBigInt);
    test_from!(from_11, f64, f64::MAX, Float);
    test_from!(from_12, f64, f64::MIN, Float);

    test_try_from!(try_from_1, u64, u64::MAX, UnsignedInt);
    test_try_from!(try_from_2, u128, u128::MAX, UnsignedInt);
    test_try_from!(try_from_3, i64, i64::MIN, SignedInt);
    test_try_from!(try_from_4, i128, i128::MIN, SignedInt);
    test_try_from!(try_from_5, f64, f64::MIN, Float);
    test_try_from!(try_from_6, f64, f64::MAX, Float);

    #[test]
    #[should_panic]
    fn try_from_error() {
        _ = i64::try_from(Value::UnsignedInt(u128::MAX)).unwrap();
    }

    test_from_str!(from_str_1, "11.2" => Order::Float);
    test_from_str!(from_str_2, "-11.2" => Order::Float);
    test_from_str!(from_str_3a, "10" => Order::UnsignedInt);
    test_from_str!(from_str_99, "-10" => Order::SignedInt);
    test_from_str!(from_str_3, "18446744073709551615" => Order::UnsignedInt);
    test_from_str!(from_str_unsignedbigint, "340282366920938463463374607431768211456" => Order::UnsignedBigInt);
    test_from_str!(from_str_4, "340282366920938463463374607431768211455" => Order::UnsignedInt);
    test_from_str!(from_str_5, "-9223372036854775808" => Order::SignedInt);
    test_from_str!(from_str_6, "-170141183460469231731687303715884105729" => Order::SignedBigInt);
    test_from_str!(from_str_7, "-170141183460469231731687303715884105728" => Order::SignedInt);
}
