use super::dispatch_operation;
use crate::value::Value;
use std::ops::{self, AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

// shim for the missing method on f64
trait CheckedAdd: Sized + ops::Add<Output = Self> {
    fn checked_add(self, rhs: Self) -> Option<Self>;
}

impl CheckedAdd for f64 {
    fn checked_add(self, rhs: Self) -> Option<Self> {
        Some(self + rhs)
    }
}

impl<Rhs> AddAssign<Rhs> for Value
where
    Rhs: Into<Value>,
{
    fn add_assign(&mut self, rhs: Rhs) {
        let mut rhs = rhs.into();
        *self = dispatch_operation!(self, rhs, n, |rhs| (*n).checked_add(rhs).map(Value::from))
            .unwrap_or_else(|| {
                self.promote();
                dispatch_operation!(self, rhs, n, |rhs| Value::from(*n + rhs))
            });
    }
}

impl<Rhs> ops::Add<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Value;
    fn add(mut self, rhs: Rhs) -> Value {
        self.add_assign(rhs);
        self
    }
}

impl<Rhs> ops::SubAssign<Rhs> for Value
where
    Rhs: Into<Value>,
{
    fn sub_assign(&mut self, rhs: Rhs) {
        let mut rhs = rhs.into();
        if rhs.order() > self.order() {
            self.promote_to_signed();
        }
        dispatch_operation!(self, rhs, n, |rhs| *n -= rhs);
    }
}

impl<Rhs> ops::Sub<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Value;

    fn sub(mut self, rhs: Rhs) -> Self::Output {
        self.sub_assign(rhs);
        self
    }
}

impl<Rhs> ops::MulAssign<Rhs> for Value
where
    Rhs: Into<Value>,
{
    fn mul_assign(&mut self, rhs: Rhs) {
        let mut rhs = rhs.into();
        dispatch_operation!(self, rhs, n, |rhs| *n *= rhs);
    }
}

impl<Rhs> ops::Mul<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Value;

    fn mul(mut self, rhs: Rhs) -> Self::Output {
        self.mul_assign(rhs);
        self
    }
}

impl<Rhs> ops::DivAssign<Rhs> for Value
where
    Rhs: Into<Value>,
{
    fn div_assign(&mut self, rhs: Rhs) {
        self.promote_to_float();
        let mut rhs = rhs.into();
        rhs.promote_to_float();
        dispatch_operation!(self, rhs, n, |rhs| *n /= rhs);
    }
}

impl<Rhs> ops::Div<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Value;

    fn div(mut self, rhs: Rhs) -> Self::Output {
        self.div_assign(rhs);
        self
    }
}

impl<Rhs> ops::RemAssign<Rhs> for Value
where
    Rhs: Into<Value>,
{
    fn rem_assign(&mut self, rhs: Rhs) {
        let mut rhs = rhs.into();
        dispatch_operation!(self, rhs, n, |rhs| *n %= rhs);
    }
}

impl<Rhs> ops::Rem<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Value;

    fn rem(mut self, rhs: Rhs) -> Self::Output {
        self.rem_assign(rhs);
        self
    }
}

impl ops::Neg for Value {
    type Output = Value;

    fn neg(mut self) -> Self::Output {
        self.promote_to_signed();
        match self {
            Value::UnsignedInt(_) | Value::UnsignedBigInt(_) => {
                unreachable!("we have already promoted out of unsigned territory")
            }
            // these integers cannot represent the negative of their minima
            Value::SignedInt(n) if n == i64::MIN => {
                self.promote();
                self.neg()
            }
            Value::SignedBigInt(n) if n == i128::MIN => {
                self.promote();
                self.neg()
            }
            // everything else is simple negation
            Value::SignedInt(n) => (-n).into(),
            Value::SignedBigInt(n) => (-n).into(),
            Value::Float(n) => (-n).into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_value_eq {
        ($lhs:expr, $rhs:expr) => {{
            match ($lhs, $rhs) {
                (Value::Float(l), Value::Float(r)) => {
                    let diff = (l - r).abs();
                    let tol = f64::EPSILON * l.abs().max(r.abs()).max(1.0);
                    assert!(diff <= tol, "lhs: {:?}, rhs: {:?}, diff: {:?}", l, r, diff);
                }
                (l, r) => assert_eq!(l, r),
            }
        }};
    }

    macro_rules! test_add_assign {
        ($lhs_variant:ident => $lhs_val:expr, $rhs_variant:ident => $rhs_val:expr, $expected_variant:ident => $expected_val:expr) => {{
            let mut lhs = Value::$lhs_variant($lhs_val);
            let rhs = Value::$rhs_variant($rhs_val);
            let expected = Value::$expected_variant($expected_val);
            lhs += rhs;
            assert_value_eq!(lhs, expected);
        }};
    }

    #[test]
    fn add_assign() {
        test_add_assign!(UnsignedInt => 200, UnsignedInt => 200, UnsignedInt => 400);
        test_add_assign!(SignedInt => -10, UnsignedInt => 10, SignedInt => 0);
        test_add_assign!(UnsignedInt => 10, Float => 1.5, Float => 11.5);
        test_add_assign!(UnsignedInt => u64::MAX, UnsignedInt => u64::MAX, UnsignedBigInt => u64::MAX as u128 + u64::MAX as u128);
        test_add_assign!(UnsignedInt => 1, UnsignedInt => 2, UnsignedInt => 3);
        test_add_assign!(SignedInt => -5, SignedInt => 10, SignedInt => 5);
        test_add_assign!(UnsignedBigInt => 1_000_000_000_000, SignedInt => -1_000_000_000_000, SignedInt => 0);
        test_add_assign!(UnsignedInt => u64::MAX, UnsignedInt => 1, UnsignedBigInt => u64::MAX as u128 + 1);
        test_add_assign!(SignedInt => i64::MAX, SignedInt => 1, SignedBigInt => i64::MAX as i128 + 1);
        test_add_assign!(UnsignedBigInt => u128::MAX, UnsignedInt => 1, Float => u128::MAX as f64 + 1.0);
        test_add_assign!(SignedBigInt => i128::MAX, SignedInt => 1, Float => i128::MAX as f64 + 1.0);
        test_add_assign!(UnsignedInt => 10, SignedInt => -5, SignedInt => 5);
        test_add_assign!(UnsignedBigInt => 100, SignedBigInt => -50, SignedBigInt => 50);
        test_add_assign!(UnsignedInt => 10, Float => 2.5, Float => 12.5);
        test_add_assign!(SignedBigInt => -100, Float => 0.5, Float => -99.5);
        test_add_assign!(Float => 1.1, Float => 2.2, Float => 3.3);
        test_add_assign!(Float => -1.5, Float => 0.5, Float => -1.0);
        test_add_assign!(UnsignedInt => 0, SignedInt => i64::MIN, SignedInt => i64::MIN);
        test_add_assign!(SignedBigInt => i128::MAX, Float => 0.1, Float => i128::MAX as f64 + 0.1);
        test_add_assign!(UnsignedBigInt => u128::MAX, Float => 1.0, Float => u128::MAX as f64 + 1.0);
    }
}
