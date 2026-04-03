use crate::value::Value;
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::ToPrimitive;
use std::cmp::Ordering;

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::UnsignedInt(l), Value::UnsignedInt(r)) => l.cmp(r),
            (Value::SignedInt(l), Value::SignedInt(r)) => l.cmp(r),
            (Value::UnsignedBigInt(l), Value::UnsignedBigInt(r)) => l.cmp(r),
            (Value::SignedBigInt(l), Value::SignedBigInt(r)) => l.cmp(r),
            (Value::Float(l), Value::Float(r)) => l.total_cmp(r),
            (Value::UnsignedInt(l), Value::SignedInt(r)) => (*l as i128).cmp(r),
            (Value::SignedInt(l), Value::UnsignedInt(r)) => l.cmp(&(*r as i128)),
            (Value::UnsignedInt(l), Value::UnsignedBigInt(r)) => BigUint::from(*l).cmp(r),
            (Value::UnsignedBigInt(l), Value::UnsignedInt(r)) => l.cmp(&(*r).into()),
            (Value::UnsignedBigInt(l), Value::SignedInt(r)) => {
                if *r < 0 {
                    std::cmp::Ordering::Greater
                } else {
                    l.cmp(&BigUint::from(*r as u128))
                }
            }
            (Value::SignedInt(l), Value::SignedBigInt(r)) => BigInt::from(*l).cmp(r),
            (Value::SignedBigInt(l), Value::SignedInt(r)) => l.cmp(&(*r).into()),
            (Value::SignedBigInt(l), Value::UnsignedInt(r)) => {
                if l.sign() == Sign::Minus {
                    Ordering::Less
                } else {
                    l.magnitude().cmp(&BigUint::from(*r))
                }
            }
            (Value::UnsignedBigInt(l), Value::SignedBigInt(r)) => {
                if r.sign() == Sign::Minus {
                    Ordering::Greater
                } else {
                    l.cmp(r.magnitude())
                }
            }
            (Value::SignedBigInt(l), Value::UnsignedBigInt(r)) => {
                if l.sign() == Sign::Minus {
                    Ordering::Less
                } else {
                    l.magnitude().cmp(r)
                }
            }
            (Value::UnsignedInt(l), Value::SignedBigInt(r)) => {
                if r.sign() == Sign::Minus {
                    Ordering::Greater
                } else {
                    BigUint::from(*l).cmp(r.magnitude())
                }
            }
            (Value::SignedInt(l), Value::UnsignedBigInt(r)) => {
                if *l < 0 {
                    Ordering::Less
                } else {
                    BigUint::from(*l as u128).cmp(r)
                }
            }
            (Value::SignedInt(l), Value::Float(r)) => (*l as f64).total_cmp(r),
            (Value::UnsignedInt(l), Value::Float(r)) => (*l as f64).total_cmp(r),
            (Value::UnsignedBigInt(l), Value::Float(r)) => {
                l.to_f64().unwrap_or(f64::INFINITY).total_cmp(r)
            }
            (Value::SignedBigInt(l), Value::Float(r)) => {
                l.to_f64().unwrap_or(f64::INFINITY).total_cmp(r)
            }
            (Value::Float(l), Value::UnsignedInt(r)) => l.total_cmp(&(*r as f64)),
            (Value::Float(l), Value::SignedInt(r)) => l.total_cmp(&(*r as f64)),
            (Value::Float(l), Value::UnsignedBigInt(r)) => {
                l.total_cmp(&r.to_f64().unwrap_or(f64::INFINITY))
            }
            (Value::Float(l), Value::SignedBigInt(r)) => {
                l.total_cmp(&r.to_f64().unwrap_or(f64::INFINITY))
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::UnsignedInt(l), Value::UnsignedInt(r)) => l == r,
            (Value::SignedInt(l), Value::SignedInt(r)) => l == r,
            (Value::UnsignedBigInt(l), Value::UnsignedBigInt(r)) => l == r,
            (Value::SignedBigInt(l), Value::SignedBigInt(r)) => l == r,
            (Value::Float(l), Value::Float(r)) => l == r,
            (Value::UnsignedInt(l), Value::SignedInt(r)) => (*l as i128) == *r,
            (Value::SignedInt(l), Value::UnsignedInt(r)) => *l == (*r as i128),
            (Value::UnsignedBigInt(l), Value::UnsignedInt(r)) => l == &(*r).into(),
            (Value::UnsignedInt(l), Value::UnsignedBigInt(r)) => &BigUint::from(*l) == r,
            (Value::SignedBigInt(l), Value::SignedInt(r)) => l == &(*r).into(),
            (Value::SignedInt(l), Value::SignedBigInt(r)) => &BigInt::from(*l) == r,
            (Value::UnsignedBigInt(l), Value::SignedBigInt(r)) => {
                r.sign() != num_bigint::Sign::Minus && l == r.magnitude()
            }
            (Value::SignedBigInt(l), Value::UnsignedBigInt(r)) => {
                l.sign() != num_bigint::Sign::Minus && l.magnitude() == r
            }
            (Value::Float(l), rhs) => {
                *l == match rhs {
                    Value::UnsignedInt(n) => *n as f64,
                    Value::SignedInt(n) => *n as f64,
                    Value::UnsignedBigInt(n) => n.to_f64().unwrap_or(f64::INFINITY),
                    Value::SignedBigInt(n) => n.to_f64().unwrap_or(f64::INFINITY),
                    Value::Float(n) => *n,
                }
            }
            (lhs, Value::Float(r)) => {
                *r == match lhs {
                    Value::UnsignedInt(n) => *n as f64,
                    Value::SignedInt(n) => *n as f64,
                    Value::UnsignedBigInt(n) => n.to_f64().unwrap_or(f64::INFINITY),
                    Value::SignedBigInt(n) => n.to_f64().unwrap_or(f64::INFINITY),
                    Value::Float(n) => *n,
                }
            }
            // if any other combination exists, they can't be equal
            _ => false,
        }
    }
}

impl Eq for Value {}

impl Value {
    /// Perform a strict equality comparison: this is equal if the values have equal value and order _without promotion_.
    pub fn strict_eq(self, other: Self) -> bool {
        match (self, other) {
            (Value::UnsignedInt(l), Value::UnsignedInt(r)) => l == r,
            (Value::UnsignedBigInt(l), Value::UnsignedBigInt(r)) => l == r,
            (Value::SignedInt(l), Value::SignedInt(r)) => l == r,
            (Value::SignedBigInt(l), Value::SignedBigInt(r)) => l == r,
            (Value::Float(l), Value::Float(r)) => l == r,
            // if any other combination exists, they can't be equal
            _ => false,
        }
    }

    /// Compute a strict ordering: this orders first by the [Order][super::Order], then by value only if the orders match
    pub fn strict_cmp(self, other: Self) -> Ordering {
        self.order()
            .cmp(&other.order())
            .then_with(|| self.cmp(&other))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;
    use std::cmp::Ordering;

    #[rstest]
    #[case::cmp_1(Value::UnsignedInt(10), Value::UnsignedInt(20), Ordering::Less)]
    #[case::cmp_2(Value::UnsignedInt(10), Value::SignedInt(10), Ordering::Equal)]
    #[case::cmp_3(Value::UnsignedInt(10), Value::SignedInt(9), Ordering::Greater)]
    #[case::cmp_4(Value::UnsignedInt(0), Value::SignedInt(-1), Ordering::Greater)]
    #[case::cmp_5(Value::SignedInt(-5), Value::UnsignedInt(0), Ordering::Less)]
    #[case::cmp_6(Value::UnsignedBigInt(1000_u128.into()), Value::UnsignedInt(500), Ordering::Greater)]
    #[case::cmp_7(Value::UnsignedBigInt(500_u128.into()), Value::UnsignedInt(500), Ordering::Equal)]
    #[case::cmp_8(Value::UnsignedBigInt(500_u128.into()), Value::SignedInt(-100), Ordering::Greater)]
    #[case::cmp_9(Value::UnsignedBigInt(500_u128.into()), Value::SignedInt(400), Ordering::Greater)]
    #[case::cmp_10(Value::UnsignedBigInt(400_u128.into()), Value::SignedInt(500), Ordering::Less)]
    #[case::cmp_11(Value::SignedBigInt(500_i128.into()), Value::UnsignedInt(400), Ordering::Greater)]
    #[case::cmp_12(Value::SignedBigInt((-100_i128).into()), Value::UnsignedInt(400), Ordering::Less)]
    #[case::cmp_13(Value::SignedBigInt((-10_i128).into()), Value::UnsignedBigInt(20_u128.into()), Ordering::Less)]
    #[case::cmp_14(Value::SignedBigInt(30_i128.into()), Value::UnsignedBigInt(20_u128.into()), Ordering::Greater)]
    #[case::cmp_15(Value::SignedBigInt(20_i128.into()), Value::UnsignedBigInt(20_u128.into()), Ordering::Equal)]
    #[case::cmp_16(Value::Float(1.5), Value::UnsignedInt(1), Ordering::Greater)]
    #[case::cmp_17(Value::Float(-2.0), Value::SignedInt(-1), Ordering::Less)]
    #[case::cmp_18(Value::Float(10.0), Value::Float(10.0), Ordering::Equal)]
    #[case::cmp_19(Value::Float(10.1), Value::Float(10.0), Ordering::Greater)]
    #[case::cmp_20(Value::Float(-1.0), Value::UnsignedInt(0), Ordering::Less)]
    #[case::cmp_21(Value::Float(100.0), Value::UnsignedBigInt(99_u128.into()), Ordering::Greater)]
    #[case::cmp_22(Value::Float(50.0), Value::SignedBigInt(50_i128.into()), Ordering::Equal)]
    fn compare(#[case] left: Value, #[case] right: Value, #[case] expected: Ordering) {
        let result = left.cmp(&right);
        assert_eq!(
            result, expected,
            "left = {left:?}, right = {right:?} | expected {expected:?} got {result:?}"
        );
    }
}
