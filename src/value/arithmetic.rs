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

// shim for the missing method on f64
trait CheckedSub: Sized + ops::Sub<Output = Self> {
    fn checked_sub(self, rhs: Self) -> Option<Self>;
}

impl CheckedSub for f64 {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        Some(self - rhs)
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
        if rhs > *self {
            self.promote_to_signed();
        }
        *self = dispatch_operation!(self, rhs, n, |rhs| (*n).checked_sub(rhs).map(Value::from))
            .unwrap_or_else(|| {
                self.promote();
                dispatch_operation!(self, rhs, n, |rhs| Value::from(*n - rhs))
            });
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
    use crate::value::Order;

    macro_rules! test_adds {
        ($lhs_variant:ident => $lhs_val:expr, $rhs_variant:ident => $rhs_val:expr, $expected_variant:ident) => {{
            let expected = Order::$expected_variant;
            // Test Add
            let lhs = Value::$lhs_variant($lhs_val);
            let rhs = Value::$rhs_variant($rhs_val);
            let result = lhs + rhs;
            assert_eq!(
                result.order(),
                expected,
                "[Add] result.order()={:?} | expected={expected:?}",
                result.order()
            );
            // Test AddAssign
            let mut lhs = Value::$lhs_variant($lhs_val);
            let rhs = Value::$rhs_variant($rhs_val);
            lhs += rhs;
            assert_eq!(
                lhs.order(),
                expected,
                "[AddAssign] lhs.order()={:?} | expected={expected:?}",
                lhs.order()
            );
        }};
    }

    macro_rules! test_subs {
        ($lhs_variant:ident => $lhs_val:expr, $rhs_variant:ident => $rhs_val:expr, $expected_variant:ident) => {{
            let expected = Order::$expected_variant;
            // Test Sub
            let lhs = Value::$lhs_variant($lhs_val);
            let rhs = Value::$rhs_variant($rhs_val);
            let result = lhs - rhs;
            assert_eq!(
                result.order(),
                expected,
                "[Sub] result.order()={:?} | expected={expected:?}",
                result.order()
            );
            // Test SubAssign
            let mut lhs = Value::$lhs_variant($lhs_val);
            let rhs = Value::$rhs_variant($rhs_val);
            lhs -= rhs;
            assert_eq!(
                lhs.order(),
                expected,
                "[SubAssign] lhs.order()={:?} | expected={expected:?}",
                lhs.order()
            );
        }};
    }

    #[test]
    fn addition() {
        test_adds!(UnsignedInt => 200, UnsignedInt => 200, UnsignedInt);
        test_adds!(SignedInt => -10, UnsignedInt => 10, SignedInt);
        test_adds!(UnsignedInt => 10, Float => 1.5, Float);
        test_adds!(UnsignedInt => u64::MAX, UnsignedInt => u64::MAX, UnsignedBigInt);
        test_adds!(UnsignedInt => 1, UnsignedInt => 2, UnsignedInt);
        test_adds!(SignedInt => -5, SignedInt => 10, SignedInt);
        test_adds!(UnsignedBigInt => 1_000_000_000_000, SignedInt => -1_000_000_000_000, SignedInt);
        test_adds!(UnsignedInt => u64::MAX, UnsignedInt => 1, UnsignedBigInt);
        test_adds!(SignedInt => i64::MAX, SignedInt => 1, SignedBigInt);
        test_adds!(UnsignedBigInt => u128::MAX, UnsignedInt => 1, Float);
        test_adds!(SignedBigInt => i128::MAX, SignedInt => 1, Float);
        test_adds!(UnsignedInt => 10, SignedInt => -5, SignedInt);
        test_adds!(UnsignedBigInt => 100, SignedBigInt => -50, SignedBigInt);
        test_adds!(UnsignedInt => 10, Float => 2.5, Float);
        test_adds!(SignedBigInt => -100, Float => 0.5, Float);
        test_adds!(Float => 1.1, Float => 2.2, Float);
        test_adds!(Float => -1.5, Float => 0.5, Float);
        test_adds!(UnsignedInt => 0, SignedInt => i64::MIN, SignedInt);
        test_adds!(SignedBigInt => i128::MAX, Float => 0.1, Float);
        test_adds!(UnsignedBigInt => u128::MAX, Float => 1.0, Float);
    }

    #[test]
    fn subtraction() {
        test_subs!(UnsignedInt => 200, UnsignedInt => 200, UnsignedInt);
        test_subs!(SignedInt => -10, UnsignedInt => 10, SignedInt);
        test_subs!(UnsignedInt => 10, UnsignedInt => 20, SignedInt);
        test_subs!(UnsignedInt => 10, Float => 1.5, Float);
        test_subs!(UnsignedInt => u64::MAX, UnsignedInt => u64::MAX, UnsignedInt);
        test_subs!(UnsignedInt => 1, UnsignedInt => 2, SignedInt);
        test_subs!(SignedInt => -5, SignedInt => 10, SignedInt);
        test_subs!(UnsignedBigInt => 1_000_000_000_000, SignedBigInt => -1_000_000_000_000, SignedBigInt);
        test_subs!(SignedBigInt => i128::MAX, UnsignedBigInt => 170141183460469231722463931679029329921, SignedBigInt);
        test_subs!(UnsignedInt => u64::MAX, UnsignedInt => 1, UnsignedInt);
        test_subs!(SignedInt => i64::MAX, SignedInt => 1, SignedInt);
        test_subs!(Float => u128::MAX as f64 + 1.0, UnsignedInt => 10, Float);
        test_subs!(UnsignedInt => 10, SignedInt => -5, SignedInt);
        test_subs!(UnsignedBigInt => 100, SignedBigInt => -50, SignedBigInt);
        test_subs!(UnsignedInt => 10, Float => 2.5, Float);
        test_subs!(SignedBigInt => -100, Float => 0.5, Float);
        test_subs!(Float => 1.1, Float => 2.2, Float);
        test_subs!(Float => -1.5, Float => 0.5, Float);
        test_subs!(UnsignedInt => 0, SignedInt => i64::MIN, SignedBigInt);
        test_subs!(SignedInt => 0, SignedInt => i64::MIN, SignedBigInt);
        test_subs!(SignedBigInt => i128::MAX, Float => 0.1, Float);
        test_subs!(UnsignedBigInt => u128::MAX, Float => 1.0, Float);
    }
}
