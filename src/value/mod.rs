pub mod arithmetic;
pub mod comparison;
pub mod conversion;
pub mod error;
pub mod fmt;
//pub mod numeric;

pub(crate) mod dispatch_operation;
pub(crate) use dispatch_operation::*;

use num_bigint::{BigInt, BigUint};
use num_traits::{FromPrimitive, ToPrimitive};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum Value {
    UnsignedInt(u128),
    UnsignedBigInt(BigUint),
    SignedInt(i128),
    SignedBigInt(BigInt),
    Float(f64),
}

impl Value {
    /// Get the order of this value
    pub(crate) fn order(&self) -> Order {
        Order::from(self)
    }

    pub(crate) fn promote(&mut self) {
        *self = match self.clone() {
            Value::UnsignedInt(n) => Value::UnsignedBigInt(n.into()),
            Value::UnsignedBigInt(n) => {
                if let Some(v) = n.to_i128() {
                    Value::SignedInt(v)
                } else {
                    Value::SignedBigInt(n.into())
                }
            }
            Value::SignedInt(n) => Value::SignedBigInt(n.into()),
            Value::SignedBigInt(n) => Value::Float(n.to_f64().unwrap_or(f64::INFINITY)),
            Value::Float(n) => Value::Float(n),
        };
    }

    /// Promote this value until it is signed, according to its value.
    pub(crate) fn promote_to_signed(&mut self) {
        while self.order() <= Order::UnsignedBigInt {
            self.promote();
        }
    }

    /// Promote this value until it is a float.
    pub(crate) fn promote_to_float(&mut self) -> &mut f64 {
        // there is no case where an integer value produces NaN when converted to a float
        *self = match self.clone() {
            Value::UnsignedInt(n) => (n as f64).into(),
            Value::UnsignedBigInt(n) => (n.to_f64()).expect("no error").into(),
            Value::SignedInt(n) => (n as f64).into(),
            Value::SignedBigInt(n) => (n.to_f64()).expect("no error").into(),
            Value::Float(n) => n.into(),
        };
        let Self::Float(f) = self else {
            unreachable!("we just promoted up to float")
        };
        f
    }

    /// Demote this value to the narrowest valid container type
    pub(crate) fn demote(&mut self) {
        *self = match self.clone() {
            Value::UnsignedBigInt(n) => {
                if let Some(v) = n.to_u128() {
                    Value::UnsignedInt(v)
                } else {
                    Value::UnsignedBigInt(n)
                }
            }
            Value::SignedBigInt(n) => {
                if let Some(v) = n.to_i128() {
                    Value::SignedInt(v)
                } else {
                    Value::SignedBigInt(n)
                }
            }
            Value::Float(f) if f.fract() == 0.0 => {
                if let Some(bi) = num_bigint::BigInt::from_f64(f) {
                    if let Some(v) = bi.to_i128() {
                        Value::SignedInt(v)
                    } else {
                        Value::SignedBigInt(bi)
                    }
                } else {
                    Value::Float(f)
                }
            }
            other => other,
        };
    }

    /// Find the minimum compatible order for `self` and `other` by promoting the lesser until they match.
    pub(crate) fn match_orders(&mut self, other: &mut Self) {
        while self.order() != other.order() {
            match self.order().cmp(&other.order()) {
                Ordering::Equal => unreachable!("orders already known not to be equal"),
                Ordering::Less => self.promote(),
                Ordering::Greater => other.promote(),
            }
        }
    }
}

// ======
// Order
// ======

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Order {
    UnsignedInt,
    UnsignedBigInt,
    SignedInt,
    SignedBigInt,
    Float,
}

impl From<Value> for Order {
    fn from(value: Value) -> Self {
        match value {
            Value::UnsignedInt(_) => Self::UnsignedInt,
            Value::UnsignedBigInt(_) => Self::UnsignedBigInt,
            Value::SignedInt(_) => Self::SignedInt,
            Value::SignedBigInt(_) => Self::SignedBigInt,
            Value::Float(_) => Self::Float,
        }
    }
}

impl From<&Value> for Order {
    fn from(value: &Value) -> Self {
        match value {
            Value::UnsignedInt(_) => Self::UnsignedInt,
            Value::UnsignedBigInt(_) => Self::UnsignedBigInt,
            Value::SignedInt(_) => Self::SignedInt,
            Value::SignedBigInt(_) => Self::SignedBigInt,
            Value::Float(_) => Self::Float,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Value;
    use super::*;
    use rstest::*;

    #[rstest]
    #[case::promotion_1(Value::UnsignedInt(u128::MAX), Order::UnsignedBigInt)]
    #[case::promotion_2(Value::UnsignedBigInt(u128::MAX.into()), Order::SignedBigInt)]
    #[case::promotion_unsignedbigint_that_fits_in_unsignedint(Value::UnsignedBigInt((i128::MAX as u128).into()), Order::SignedInt)]
    #[case::promotion_3(Value::UnsignedInt((i128::MAX - 1) as u128), Order::UnsignedBigInt)]
    #[case::promotion_4(Value::UnsignedInt(u128::MAX), Order::UnsignedBigInt)]
    #[case::promotion_signedbigint_to_float(Value::SignedBigInt(i128::MAX.into()), Order::Float)] // this should realistically never happen
    fn promotion(#[case] value: Value, #[case] expected: Order) {
        let mut v = value.clone(); // Clone so we can write the original value in error if needed
        v.promote();
        assert_eq!(
            v.order(),
            expected,
            "expected {value:?} => to promote to => {expected:?} but got = {v:?}"
        );
    }
}
