use super::dispatch_operation;
use crate::value::Value;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

trait CheckedMaths:
    Sized + Add<Output = Self> + Sub<Output = Self> + Div<Output = Self> + Rem<Output = Self>
{
    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    fn checked_div(self, rhs: Self) -> Option<Self>;
    fn checked_mul(self, rhs: Self) -> Option<Self>;
    fn checked_rem(self, rhs: Self) -> Option<Self>;
}

impl CheckedMaths for f64 {
    fn checked_add(self, rhs: Self) -> Option<Self> {
        Some(self + rhs)
    }
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        Some(self - rhs)
    }
    fn checked_div(self, rhs: Self) -> Option<Self> {
        Some(self / rhs)
    }
    fn checked_mul(self, rhs: Self) -> Option<Self> {
        Some(self * rhs)
    }
    fn checked_rem(self, rhs: Self) -> Option<Self> {
        Some(self % rhs)
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

impl<Rhs> Add<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Value;
    fn add(mut self, rhs: Rhs) -> Value {
        self.add_assign(rhs);
        self
    }
}

impl<Rhs> SubAssign<Rhs> for Value
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

impl<Rhs> Sub<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Value;

    fn sub(mut self, rhs: Rhs) -> Self::Output {
        self.sub_assign(rhs);
        self
    }
}

impl<Rhs> MulAssign<Rhs> for Value
where
    Rhs: Into<Value>,
{
    fn mul_assign(&mut self, rhs: Rhs) {
        let mut rhs = rhs.into();
        *self = dispatch_operation!(self, rhs, n, |rhs| (*n).checked_mul(rhs).map(Value::from))
            .unwrap_or_else(|| {
                self.promote();
                dispatch_operation!(self, rhs, n, |rhs| Value::from(*n * rhs))
            });
    }
}

impl<Rhs> Mul<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Value;

    fn mul(mut self, rhs: Rhs) -> Self::Output {
        self.mul_assign(rhs);
        self
    }
}

impl<Rhs> DivAssign<Rhs> for Value
where
    Rhs: Into<Value>,
{
    fn div_assign(&mut self, rhs: Rhs) {
        let mut rhs = rhs.into();
        *self = dispatch_operation!(self, rhs, n, |rhs| (*n).checked_div(rhs).map(Value::from))
            .unwrap_or_else(|| {
                self.promote();
                dispatch_operation!(self, rhs, n, |rhs| Value::from(*n / rhs))
            });
    }
}

impl<Rhs> Div<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Value;

    fn div(mut self, rhs: Rhs) -> Self::Output {
        self.div_assign(rhs);
        self
    }
}

impl<Rhs> RemAssign<Rhs> for Value
where
    Rhs: Into<Value>,
{
    fn rem_assign(&mut self, rhs: Rhs) {
        let mut rhs = rhs.into();
        *self = dispatch_operation!(self, rhs, n, |rhs| (*n).checked_rem(rhs).map(Value::from))
            .unwrap_or_else(|| {
                self.promote();
                dispatch_operation!(self, rhs, n, |rhs| Value::from(*n % rhs))
            });
    }
}

impl<Rhs> Rem<Rhs> for Value
where
    Rhs: Into<Value>,
{
    type Output = Value;

    fn rem(mut self, rhs: Rhs) -> Self::Output {
        self.rem_assign(rhs);
        self
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        match self {
            Self::UnsignedInt(_) | Self::UnsignedBigInt(_) => {
                self.promote_to_signed();
                self.neg()
            }
            // these integers cannot represent the negative of their minima
            Self::SignedInt(n) if n == i64::MIN => {
                self.promote();
                self.neg()
            }
            Self::SignedBigInt(n) if n == i128::MIN => {
                self.promote();
                self.neg()
            }
            // everything else is simple negation
            Self::SignedInt(n) => (-n).into(),
            Self::SignedBigInt(n) => (-n).into(),
            Self::Float(n) => (-n).into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::value::Order;

    macro_rules! make_test_macro {
        ($macro_name:ident, $operation:tt, $assign_operation:tt) => {
            macro_rules! $macro_name {
                ($test_name:ident, $lhs_variant:ident => $lhs_val:expr, $rhs_variant:ident => $rhs_val:expr, $expected_variant:ident) => {
                    #[test]
                    fn $test_name() {
                        let expected = Order::$expected_variant;

                        let lhs = Value::$lhs_variant($lhs_val);
                        let rhs = Value::$rhs_variant($rhs_val);
                        let result = lhs $operation rhs;
                        assert_eq!(
                            result.order(),
                            expected,
                            "result.order()={:?} | expected={expected:?} | result={result:?}",
                            result.order()
                        );

                        let mut lhs = Value::$lhs_variant($lhs_val);
                        let rhs = Value::$rhs_variant($rhs_val);
                        lhs $assign_operation rhs;
                        assert_eq!(
                            lhs.order(),
                            expected,
                            "lhs.order()={:?} | expected={expected:?} | lhs={lhs:?}",
                            lhs.order()
                        );
                    }
                };
            }
        }
    }

    make_test_macro!(test_add, +, +=);
    make_test_macro!(test_sub, -, -=);
    make_test_macro!(test_mul, *, *=);
    make_test_macro!(test_div, /, /=);
    make_test_macro!(test_rem, %, %=);

    // Addition
    test_add!(add1, UnsignedInt => u64::MAX, UnsignedInt => u64::MAX, UnsignedBigInt);
    test_add!(add2, UnsignedInt => 200, UnsignedInt => 200, UnsignedInt);
    test_add!(add3, SignedInt => -10, UnsignedInt => 10, SignedInt);
    test_add!(add4, UnsignedInt => 10, Float => 1.5, Float);
    test_add!(add7, UnsignedInt => u64::MAX, UnsignedInt => u64::MAX, UnsignedBigInt);
    test_add!(add8, UnsignedInt => 1, UnsignedInt => 2, UnsignedInt);
    test_add!(add9, SignedInt => -5, SignedInt => 10, SignedInt);
    test_add!(add01, UnsignedBigInt => 1_000_000_000_000, SignedInt => -1_000_000_000_000, SignedInt);
    test_add!(add11, UnsignedInt => u64::MAX, UnsignedInt => 1, UnsignedBigInt);
    test_add!(add12, SignedInt => i64::MAX, SignedInt => 1, SignedBigInt);
    test_add!(add13, UnsignedBigInt => u128::MAX, UnsignedInt => 1, Float);
    test_add!(add14, SignedBigInt => i128::MAX, SignedInt => 1, Float);
    test_add!(add15, UnsignedInt => 10, SignedInt => -5, SignedInt);
    test_add!(add16, UnsignedBigInt => 100, SignedBigInt => -50, SignedBigInt);
    test_add!(add17, UnsignedInt => 10, Float => 2.5, Float);
    test_add!(add18, SignedBigInt => -100, Float => 0.5, Float);
    test_add!(add19, Float => 1.1, Float => 2.2, Float);
    test_add!(add20, Float => -1.5, Float => 0.5, Float);
    test_add!(add21, UnsignedInt => 0, SignedInt => i64::MIN, SignedInt);
    test_add!(add22, SignedBigInt => i128::MAX, Float => 0.1, Float);
    test_add!(add23, UnsignedBigInt => u128::MAX, Float => 1.0, Float);

    // Subtraction
    test_sub!(sub1, SignedInt => i64::MIN, SignedInt => i64::MAX, SignedBigInt);
    test_sub!(sub2, SignedBigInt => i128::MIN, SignedBigInt => i128::MAX, Float);
    test_sub!(sub3, UnsignedInt => 200, UnsignedInt => 200, UnsignedInt);
    test_sub!(sub4, SignedInt => -10, UnsignedInt => 10, SignedInt);
    test_sub!(sub5, UnsignedInt => 10, UnsignedInt => 20, SignedInt);
    test_sub!(sub6, UnsignedInt => 10, Float => 1.5, Float);
    test_sub!(sub7, UnsignedInt => u64::MAX, UnsignedInt => u64::MAX, UnsignedInt);
    test_sub!(sub8, UnsignedInt => 1, UnsignedInt => 2, SignedInt);
    test_sub!(sub9, SignedInt => -5, SignedInt => 10, SignedInt);
    test_sub!(sub10, UnsignedBigInt => 1_000_000_000_000, SignedBigInt => -1_000_000_000_000, SignedBigInt);
    test_sub!(sub11, SignedBigInt => i128::MAX, UnsignedBigInt => 170141183460469231722463931679029329921, SignedBigInt);
    test_sub!(sub12, UnsignedInt => u64::MAX, UnsignedInt => 1, UnsignedInt);
    test_sub!(sub13, SignedInt => i64::MAX, SignedInt => 1, SignedInt);
    test_sub!(sub14, Float => u128::MAX as f64 + 1.0, UnsignedInt => 10, Float);
    test_sub!(sub15, UnsignedInt => 10, SignedInt => -5, SignedInt);
    test_sub!(sub16, UnsignedBigInt => 100, SignedBigInt => -50, SignedBigInt);
    test_sub!(sub_uint_float, UnsignedInt => 10, Float => 2.5, Float);
    test_sub!(sub_neg_int_float, SignedBigInt => -100, Float => 0.5, Float);
    test_sub!(sub_pos_float_pos_float, Float => 1.1, Float => 2.2, Float);
    test_sub!(sub_neg_float_pos_float, Float => -1.5, Float => 0.5, Float);
    test_sub!(sub_overflow_uint_int, UnsignedInt => 0, SignedInt => i64::MIN, SignedBigInt);
    test_sub!(sub_overflow_int_int, SignedInt => 0, SignedInt => i64::MIN, SignedBigInt);
    test_sub!(sub_overflow_bigint_float, SignedBigInt => i128::MAX, Float => 0.1, Float);
    test_sub!(sub_overflow_ubigint_float, UnsignedBigInt => u128::MAX, Float => 1.0, Float);

    // Multiplication
    test_mul!(mul1, UnsignedInt => 10, UnsignedInt => 2, UnsignedInt);
    test_mul!(mul2, UnsignedInt => u64::MAX, UnsignedInt => u64::MAX, UnsignedBigInt);

    // Division
    test_div!(div1, UnsignedInt => 10, UnsignedInt => 2, UnsignedInt);
    test_div!(div2, SignedInt => i64::MIN, SignedInt => -1, SignedBigInt); // This overflows i64
    test_div!(div3, SignedInt => 0, SignedInt => -1, SignedInt);
    test_div!(div4, UnsignedInt => 0, UnsignedInt => 10, UnsignedInt);
    test_div!(div5, UnsignedInt => 0, SignedInt => 10, SignedInt);
    test_div!(div6, Float => 10.0, Float => 2.0, Float);
    test_div!(div7, Float => 10.0, UnsignedInt => 2, Float);
    test_div!(div8, Float => 10.0, UnsignedInt => u64::MAX, Float);
    test_div!(div9, Float => -f64::MAX, UnsignedInt => 2, Float);

    #[test]
    #[should_panic]
    fn divide_by_zero_panics() {
        _ = Value::SignedInt(-1) / Value::SignedInt(0);
    }

    // Remainder
    test_rem!(rem1, SignedInt => 10, SignedInt => 2, SignedInt);
    test_rem!(rem2, SignedInt => 10, SignedInt => 2, SignedInt);
}
