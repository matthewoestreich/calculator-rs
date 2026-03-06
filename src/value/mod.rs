pub mod arithmetic;
pub mod comparison;
pub mod conversion;
pub mod error;
pub mod fmt;
pub mod numeric;

pub(crate) mod dispatch_operation;
use std::cmp::Ordering;

pub(crate) use dispatch_operation::*;

use crate::value::conversion::ToPrimitive as _;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    UnsignedInt(u64),
    UnsignedBigInt(u128),
    SignedInt(i64),
    SignedBigInt(i128),
    Float(f64),
}

impl Value {
    /// Get the order of this value
    pub(crate) fn order(&self) -> Order {
        Order::from(*self)
    }

    pub(crate) fn promote(&mut self) {
        *self = match *self {
            Value::UnsignedInt(n) => Self::UnsignedBigInt(n as _),
            Value::UnsignedBigInt(n) => {
                const SI_MAX: u128 = i64::MAX as _;
                const SBI_MIN: u128 = SI_MAX + 1;
                const SBI_MAX: u128 = i128::MAX as _;

                match n {
                    0..=SI_MAX => Self::SignedInt(n as _),
                    SBI_MIN..=SBI_MAX => Self::SignedBigInt(n as _),
                    _ => Self::Float(n.to_f64().expect("all u128 convert to f64")),
                }
            }
            Value::SignedInt(n) => Self::SignedBigInt(n as _),
            Value::SignedBigInt(n) => Self::Float(n.to_f64().expect("all i128 convert to f64")),
            Value::Float(n) => Self::Float(n),
        }
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
        *self = match *self {
            Value::UnsignedInt(n) => (n as f64).into(),
            Value::UnsignedBigInt(n) => (n as f64).into(),
            Value::SignedInt(n) => (n as f64).into(),
            Value::SignedBigInt(n) => (n as f64).into(),
            Value::Float(n) => n.into(),
        };
        let Self::Float(f) = self else {
            unreachable!("we just promoted up to float")
        };
        f
    }

    /// Demote this value to the narrowest valid container type
    pub(crate) fn demote(&mut self) {
        const ZERO: f64 = 0.0;
        const UI_MAX: f64 = u64::MAX as _;
        const UBI_MAX: f64 = u128::MAX as _;
        const SI_MIN: f64 = i64::MIN as _;
        const SI_MAX: f64 = i64::MAX as _;
        const SBI_MIN: f64 = i128::MIN as _;
        const SBI_MAX: f64 = i128::MAX as _;

        let value = *self.clone().promote_to_float();
        debug_assert!(
            value.fract().abs() < f64::EPSILON,
            "we should never demote values not already known to be integral"
        );

        let narrowest_order = [
            (ZERO..=UI_MAX, Order::UnsignedInt),
            (ZERO..=UBI_MAX, Order::UnsignedBigInt),
            (SI_MIN..=SI_MAX, Order::SignedInt),
            (SBI_MIN..=SBI_MAX, Order::SignedBigInt),
        ]
        .into_iter()
        .find_map(|(range, order)| range.contains(&value).then_some(order))
        .unwrap_or(Order::Float);

        // rhs isn't really necessary, except structurally, for the `dispatch_operation` macro
        // maybe it would just vanish under optimization?
        let mut rhs = *self;

        *self = dispatch_operation!(self, rhs, n, |_rhs| {
            // due to the nature of the macro we're 100% going to perform at least one unnecessary
            // cast in every expansion branch of this macro; can't be helped
            #[expect(clippy::unnecessary_cast)]
            match narrowest_order {
                Order::UnsignedInt => (*n as u64).into(),
                Order::UnsignedBigInt => (*n as u128).into(),
                Order::SignedInt => (*n as i64).into(),
                Order::SignedBigInt => (*n as i128).into(),
                Order::Float => (*n as f64).into(),
            }
        });
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
