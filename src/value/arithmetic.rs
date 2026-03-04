use crate::value::Value;
use std::ops::{self, AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

impl<Rhs> AddAssign<Rhs> for Value
where
    Rhs: Into<Value>,
{
    fn add_assign(&mut self, rhs: Rhs) {
        let rhs = rhs.into();

        let result = match self {
            Value::UnsignedInt(n) => {
                let rhs_val = u64::try_from(&rhs).expect("orders must match");
                (*n).checked_add(rhs_val).map(Value::from)
            }
            Value::UnsignedBigInt(n) => {
                let rhs_val = u128::try_from(&rhs).expect("orders must match");
                (*n).checked_add(rhs_val).map(Value::from)
            }
            Value::SignedInt(n) => {
                let rhs_val = i64::try_from(&rhs).expect("orders must match");
                (*n).checked_add(rhs_val).map(Value::from)
            }
            Value::SignedBigInt(n) => {
                let rhs_val = i128::try_from(&rhs).expect("orders must match");
                (*n).checked_add(rhs_val).map(Value::from)
            }
            Value::Float(n) => {
                let rhs_val = f64::try_from(&rhs).expect("orders must match");
                Some(Value::Float(*n + rhs_val))
            }
        };

        // If checked_add overflowed for integers, promote and add normally
        *self = result.unwrap_or_else(|| {
            self.promote();
            match self {
                Value::UnsignedInt(n) => {
                    let rhs_val = u64::try_from(&rhs).expect("orders must match");
                    Value::from(*n + rhs_val)
                }
                Value::UnsignedBigInt(n) => {
                    let rhs_val = u128::try_from(&rhs).expect("orders must match");
                    Value::from(*n + rhs_val)
                }
                Value::SignedInt(n) => {
                    let rhs_val = i64::try_from(&rhs).expect("orders must match");
                    Value::from(*n + rhs_val)
                }
                Value::SignedBigInt(n) => {
                    let rhs_val = i128::try_from(&rhs).expect("orders must match");
                    Value::from(*n + rhs_val)
                }
                Value::Float(n) => {
                    let rhs_val = f64::try_from(&rhs).expect("orders must match");
                    Value::Float(*n + rhs_val)
                }
            }
        });
    }
}
