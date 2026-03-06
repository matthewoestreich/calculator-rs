use std::str::FromStr;

use crate::value::{Value, error::ValueError};

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
impl_from!(u128 => UnsignedBigInt);
impl_from!(i8 => SignedInt);
impl_from!(i16 => SignedInt);
impl_from!(i32 => SignedInt);
impl_from!(i64 => SignedInt);
impl_from!(i128 => SignedBigInt);
impl_from!(f64 => Float);

// ===============================================================
// TryFrom<T>
// ===============================================================

macro_rules! impl_try_from {
    ($t:ty => $variant:ident) => {
        impl TryFrom<Value> for $t {
            type Error = ValueError;
            fn try_from(value: Value) -> Result<Self, Self::Error> {
                match value {
                    Value::$variant(n) => Ok(n),
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
                    Value::$variant(n) => Ok(*n),
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
impl_try_from!(u128 => UnsignedBigInt);
impl_try_from!(i64 => SignedInt);
impl_try_from!(i128 => SignedBigInt);
impl_try_from!(f64 => Float);

// ===============================================================
// FromStr
// ===============================================================

impl FromStr for Value {
    type Err = ValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u64>()
            .map(Self::UnsignedInt)
            .or_else(|_| s.parse::<u128>().map(Self::UnsignedBigInt))
            .or_else(|_| s.parse::<i64>().map(Self::SignedInt))
            .or_else(|_| s.parse::<i128>().map(Self::SignedBigInt))
            .or_else(|_| s.parse::<f64>().map(Self::Float))
            .map_err(|_| ValueError::Parsing {
                value: s.to_string(),
            })
    }
}

// ===============================================================
// ToPrimitive
// ===============================================================

#[allow(dead_code)]
pub(crate) trait ToPrimitive {
    fn to_i64(&self) -> Option<i64>;
    fn to_u64(&self) -> Option<u64>;
    fn to_f64(&self) -> Option<f64>;
}

impl ToPrimitive for u128 {
    fn to_i64(&self) -> Option<i64> {
        if *self <= i64::MAX as u128 {
            Some(*self as i64)
        } else {
            None
        }
    }

    fn to_u64(&self) -> Option<u64> {
        if *self <= u64::MAX as u128 {
            Some(*self as u64)
        } else {
            None
        }
    }

    fn to_f64(&self) -> Option<f64> {
        Some(*self as f64)
    }
}

impl ToPrimitive for i128 {
    fn to_i64(&self) -> Option<i64> {
        if *self <= i64::MAX as i128 && *self >= i64::MIN as i128 {
            Some(*self as i64)
        } else {
            None
        }
    }

    fn to_u64(&self) -> Option<u64> {
        if *self <= u64::MAX as i128 && *self >= u64::MIN as i128 {
            Some(*self as u64)
        } else {
            None
        }
    }

    fn to_f64(&self) -> Option<f64> {
        Some(*self as f64)
    }
}

// ===============================================================
// Tests
// ===============================================================

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_from {
        ($t:ty, $val:expr, $variant:ident) => {{
            let n: $t = $val;
            let value = Value::from(n);
            assert_eq!(value, Value::$variant(n.into()));
        }};
    }

    /// Tests <$t>::try_from(value) and <$t>::try_from(&value)
    /// test_try_from!( [`T` eg. u8], [value eg. 222], [`Value` variant eg UnsignedInt] )
    macro_rules! test_try_from {
        ($t:ty, $val:expr, $variant:ident) => {{
            let value = Value::$variant($val);
            let n: $t = <$t>::try_from(value).expect(concat!("no ", stringify!($t), " errors"));
            assert_eq!(n, $val);
        }};
        ($t:ty, $val:expr, $variant:ident) => {{
            let value = Value::$variant($val);
            let n: $t = <$t>::try_from(&value).expect(concat!("no ", stringify!($t), " errors"));
            assert_eq!(n, $val);
        }};
    }

    macro_rules! test_from_str {
        ($str_val:expr => $expected_value_enum:expr) => {{
            concat!($str_val, "");
            let v = $str_val;
            let r = Value::from_str(v).expect("no error for {v}");
            assert_eq!(r, $expected_value_enum);
        }};
    }

    #[test]
    fn from() {
        test_from!(u8, u8::MAX, UnsignedInt);
        test_from!(u16, u16::MAX, UnsignedInt);
        test_from!(u32, u32::MAX, UnsignedInt);
        test_from!(u64, u64::MAX, UnsignedInt);
        test_from!(u128, u128::MAX, UnsignedBigInt);
        test_from!(i8, i8::MIN, SignedInt);
        test_from!(i16, i16::MIN, SignedInt);
        test_from!(i32, i32::MIN, SignedInt);
        test_from!(i64, i64::MIN, SignedInt);
        test_from!(i128, i128::MIN, SignedBigInt);
        test_from!(f64, f64::MAX, Float);
        test_from!(f64, f64::MIN, Float);
    }

    #[test]
    fn try_from() {
        test_try_from!(u64, u64::MAX, UnsignedInt);
        test_try_from!(u128, u128::MAX, UnsignedBigInt);
        test_try_from!(i64, i64::MIN, SignedInt);
        test_try_from!(i128, i128::MIN, SignedBigInt);
        test_try_from!(f64, f64::MIN, Float);
        test_try_from!(f64, f64::MAX, Float);
    }

    #[test]
    #[should_panic]
    fn try_from_error() {
        _ = i64::try_from(Value::UnsignedInt(u64::MAX)).unwrap();
    }

    #[test]
    fn from_str() {
        test_from_str!("11.2" => Value::Float(11.2));
        test_from_str!("-11.2" => Value::Float(-11.2));
        test_from_str!("18446744073709551615" => Value::UnsignedInt(u64::MAX));
        test_from_str!("340282366920938463463374607431768211455" => Value::UnsignedBigInt(u128::MAX));
        test_from_str!("-9223372036854775808" => Value::SignedInt(i64::MIN));
        test_from_str!("-170141183460469231731687303715884105728" => Value::SignedBigInt(i128::MIN));
    }
}
