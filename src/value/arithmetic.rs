use super::dispatch_operation;
use crate::value::Value;
use num_bigint::{BigInt, BigUint};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

trait CheckedMaths:
    Sized
    + Add<Output = Self>
    + Sub<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Rem<Output = Self>
{
    fn checked_add(&self, rhs: Self) -> Option<Self>;
    fn checked_sub(&self, rhs: Self) -> Option<Self>;
    fn checked_div(&self, rhs: Self) -> Option<Self>;
    fn checked_mul(&self, rhs: Self) -> Option<Self>;
    fn checked_rem(&self, rhs: Self) -> Option<Self>;
}

macro_rules! impl_checked_maths {
    ($t:ty) => {
        impl CheckedMaths for $t {
            fn checked_add(&self, rhs: Self) -> Option<Self> {
                <$t>::checked_add(*self, rhs)
            }

            fn checked_sub(&self, rhs: Self) -> Option<Self> {
                <$t>::checked_sub(*self, rhs)
            }

            fn checked_mul(&self, rhs: Self) -> Option<Self> {
                <$t>::checked_mul(*self, rhs)
            }

            fn checked_div(&self, rhs: Self) -> Option<Self> {
                <$t>::checked_div(*self, rhs)
            }

            fn checked_rem(&self, rhs: Self) -> Option<Self> {
                <$t>::checked_rem(*self, rhs)
            }
        }
    };
}

impl_checked_maths!(u128);
impl_checked_maths!(i128);
impl_checked_maths!(u64);
impl_checked_maths!(i64);

impl CheckedMaths for f64 {
    fn checked_add(&self, rhs: Self) -> Option<Self> {
        Some(*self + rhs)
    }
    fn checked_sub(&self, rhs: Self) -> Option<Self> {
        Some(*self - rhs)
    }
    fn checked_mul(&self, rhs: Self) -> Option<Self> {
        Some(*self * rhs)
    }
    fn checked_div(&self, rhs: Self) -> Option<Self> {
        Some(*self / rhs)
    }
    fn checked_rem(&self, rhs: Self) -> Option<Self> {
        Some(*self % rhs)
    }
}

impl CheckedMaths for BigInt {
    fn checked_add(&self, rhs: Self) -> Option<Self> {
        Some(self + rhs)
    }
    fn checked_sub(&self, rhs: Self) -> Option<Self> {
        Some(self - rhs)
    }
    fn checked_div(&self, rhs: Self) -> Option<Self> {
        Some(self / rhs)
    }
    fn checked_mul(&self, rhs: Self) -> Option<Self> {
        Some(self * rhs)
    }
    fn checked_rem(&self, rhs: Self) -> Option<Self> {
        Some(self % rhs)
    }
}

impl CheckedMaths for BigUint {
    fn checked_add(&self, rhs: Self) -> Option<Self> {
        Some(self + rhs)
    }
    fn checked_sub(&self, rhs: Self) -> Option<Self> {
        Some(self - rhs)
    }
    fn checked_div(&self, rhs: Self) -> Option<Self> {
        Some(self / rhs)
    }
    fn checked_mul(&self, rhs: Self) -> Option<Self> {
        Some(self * rhs)
    }
    fn checked_rem(&self, rhs: Self) -> Option<Self> {
        Some(self % rhs)
    }
}

impl<Rhs> AddAssign<Rhs> for Value
where
    Rhs: Into<Value>,
{
    fn add_assign(&mut self, rhs: Rhs) {
        let mut rhs = rhs.into();
        *self = dispatch_operation!(self, &mut rhs, n, |rhs| CheckedMaths::checked_add(n, rhs)
            .map(Value::from))
        .unwrap_or_else(|| {
            self.promote();
            dispatch_operation!(self, &mut rhs, n, |rhs| Value::from(n.clone() + rhs))
        });
    }
}

impl AddAssign<&Value> for Value {
    fn add_assign(&mut self, rhs: &Value) {
        *self = &*self + rhs;
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

impl<'a> Add<&'a Value> for &Value {
    type Output = Value;

    fn add(self, rhs: &'a Value) -> Value {
        let mut lhs = self.clone();
        lhs += rhs.clone();
        lhs
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

        *self = dispatch_operation!(self, &mut rhs, n, |rhs| CheckedMaths::checked_sub(n, rhs)
            .map(Value::from))
        .unwrap_or_else(|| {
            self.promote();
            dispatch_operation!(self, rhs, n, |rhs| Value::from(n.clone() - rhs))
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
        *self = dispatch_operation!(self, &mut rhs, n, |rhs| CheckedMaths::checked_mul(n, rhs)
            .map(Value::from))
        .unwrap_or_else(|| {
            self.promote();
            dispatch_operation!(self, rhs, n, |rhs| Value::from(n.clone() * rhs))
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
        *self = dispatch_operation!(self, &mut rhs, n, |rhs| CheckedMaths::checked_div(n, rhs)
            .map(Value::from))
        .unwrap_or_else(|| {
            self.promote();
            dispatch_operation!(self, &mut rhs, n, |rhs| Value::from(n.clone() / rhs))
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
        *self = dispatch_operation!(self, &mut rhs, n, |rhs| n.checked_rem(rhs).map(Value::from))
            .unwrap_or_else(|| {
                self.promote();
                dispatch_operation!(self, rhs, n, |rhs| Value::from(n.clone() % rhs))
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
            Self::SignedInt(n) if n == i128::MIN => {
                self.promote();
                self.neg()
            }
            /*Self::SignedBigInt(n) if n == i128::MIN => {
                self.promote();
                self.neg()
            }*/
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
    use num_bigint::{BigInt, BigUint};
    use rstest::*;

    #[rstest]
    #[case(
        Value::UnsignedInt(u128::MAX),
        Value::UnsignedInt(u128::MAX),
        Order::UnsignedBigInt
    )]
    #[case(Value::UnsignedInt(200), Value::UnsignedInt(200), Order::UnsignedInt)]
    #[case(
        Value::SignedInt(-10),
        Value::UnsignedInt(10),
        Order::SignedInt
    )]
    #[case(Value::UnsignedInt(10), Value::Float(1.5), Order::Float)]
    #[case::overflow_u128(
        Value::UnsignedInt(u128::MAX),
        Value::UnsignedInt(u128::MAX),
        Order::UnsignedBigInt
    )]
    #[case(Value::UnsignedInt(1), Value::UnsignedInt(2), Order::UnsignedInt)]
    #[case(
        Value::SignedInt(-5),
        Value::SignedInt(10),
        Order::SignedInt
    )]
    #[case(
        Value::UnsignedBigInt(BigUint::from(1_000_000_000_000_u64)),
        Value::SignedInt(-1_000_000_000_000),
        Order::SignedInt
    )]
    #[case(
        Value::UnsignedInt(u128::MAX),
        Value::UnsignedInt(1),
        Order::UnsignedBigInt
    )]
    #[case(Value::SignedInt(i128::MAX), Value::SignedInt(1), Order::SignedBigInt)]
    #[case(
        Value::UnsignedBigInt(BigUint::from(u128::MAX)),
        Value::UnsignedInt(1),
        Order::UnsignedBigInt
    )]
    #[case(
        Value::SignedBigInt(BigInt::from(i128::MAX)),
        Value::SignedInt(1),
        Order::SignedBigInt
    )]
    #[case(
        Value::UnsignedInt(10),
        Value::SignedInt(-5),
        Order::SignedInt
    )]
    #[case(
        Value::UnsignedBigInt(BigUint::from(100_u32)),
        Value::SignedBigInt(BigInt::from(-50)),
        Order::SignedBigInt
    )]
    #[case(Value::UnsignedInt(10), Value::Float(2.5), Order::Float)]
    #[case(
        Value::SignedBigInt(BigInt::from(-100)),
        Value::Float(0.5),
        Order::Float
    )]
    #[case(Value::Float(1.1), Value::Float(2.2), Order::Float)]
    #[case(
        Value::Float(-1.5),
        Value::Float(0.5),
        Order::Float
    )]
    #[case(Value::UnsignedInt(0), Value::SignedInt(i128::MIN), Order::SignedInt)]
    #[case(
        Value::SignedBigInt(BigInt::from(i128::MAX)),
        Value::Float(0.1),
        Order::Float
    )]
    #[case::last(
        Value::UnsignedBigInt(BigUint::from(u128::MAX)),
        Value::Float(1.0),
        Order::Float
    )]
    fn addition(#[case] mut left: Value, #[case] right: Value, #[case] expect: Order) {
        let r = &left + &right;
        assert_eq!(
            r.order(),
            expect,
            "left = {left:?} right = {right:?} | expected {expect:?} got {r:?}"
        );
        left += &right;
        println!("right={right:?}");
        assert_eq!(
            left.order(),
            expect,
            "right = {right:?} | expected {expect:?} got {left:?}"
        );
    }

    /*
    #[rstest]
    #[case(
        Value::SignedInt(i64::MIN),
        Value::SignedInt(i64::MAX),
        Order::SignedBigInt
    )]
    #[case(
        Value::SignedBigInt(i128::MIN),
        Value::SignedBigInt(i128::MAX),
        Order::Float
    )]
    #[case(Value::UnsignedInt(200), Value::UnsignedInt(200), Order::UnsignedInt)]
    #[case(
        Value::SignedInt(-10),
        Value::UnsignedInt(10),
        Order::SignedInt
    )]
    #[case(Value::UnsignedInt(10), Value::UnsignedInt(20), Order::SignedInt)]
    #[case(Value::UnsignedInt(10), Value::Float(1.5), Order::Float)]
    #[case(
        Value::UnsignedInt(u64::MAX),
        Value::UnsignedInt(u64::MAX),
        Order::UnsignedInt
    )]
    #[case(Value::UnsignedInt(1), Value::UnsignedInt(2), Order::SignedInt)]
    #[case(
        Value::SignedInt(-5),
        Value::SignedInt(10),
        Order::SignedInt
    )]
    #[case(
        Value::UnsignedBigInt(1_000_000_000_000),
        Value::SignedBigInt(-1_000_000_000_000),
        Order::SignedBigInt
    )]
    #[case(
        Value::SignedBigInt(i128::MAX),
        Value::UnsignedBigInt(170141183460469231722463931679029329921),
        Order::SignedBigInt
    )]
    #[case(
        Value::UnsignedInt(u64::MAX),
        Value::UnsignedInt(1),
        Order::UnsignedInt
    )]
    #[case(Value::SignedInt(i64::MAX), Value::SignedInt(1), Order::SignedInt)]
    #[case(
        Value::Float(u128::MAX as f64 + 1.0),
        Value::UnsignedInt(10),
        Order::Float
    )]
    #[case(
        Value::UnsignedInt(10),
        Value::SignedInt(-5),
        Order::SignedInt
    )]
    #[case(
        Value::UnsignedBigInt(100),
        Value::SignedBigInt(-50),
        Order::SignedBigInt
    )]
    #[case(Value::UnsignedInt(10), Value::Float(2.5), Order::Float)]
    #[case(
        Value::SignedBigInt(-100),
        Value::Float(0.5),
        Order::Float
    )]
    #[case(Value::Float(1.1), Value::Float(2.2), Order::Float)]
    #[case(
        Value::Float(-1.5),
        Value::Float(0.5),
        Order::Float
    )]
    #[case(Value::UnsignedInt(0), Value::SignedInt(i64::MIN), Order::SignedBigInt)]
    #[case(Value::SignedInt(0), Value::SignedInt(i64::MIN), Order::SignedBigInt)]
    #[case(Value::SignedBigInt(i128::MAX), Value::Float(0.1), Order::Float)]
    #[case(Value::UnsignedBigInt(u128::MAX), Value::Float(1.0), Order::Float)]
    fn subtraction(#[case] mut left: Value, #[case] right: Value, #[case] expect: Order) {
        assert_eq!((left - right).order(), expect);
        left -= right;
        assert_eq!(left.order(), expect);
    }

    #[rstest]
    #[case(Value::UnsignedInt(10), Value::UnsignedInt(2), Order::UnsignedInt)]
    #[case(
        Value::UnsignedInt(u64::MAX),
        Value::UnsignedInt(u64::MAX),
        Order::UnsignedBigInt
    )]
    fn multiplication(#[case] mut left: Value, #[case] right: Value, #[case] expect: Order) {
        assert_eq!((left * right).order(), expect);
        left *= right;
        assert_eq!(left.order(), expect);
    }

    #[rstest]
    #[case(Value::UnsignedInt(10), Value::UnsignedInt(2), Order::UnsignedInt)]
    #[case(
        Value::SignedInt(i64::MIN),
        Value::SignedInt(-1),
        Order::SignedBigInt
    )]
    #[case::i128_overflows_to_float(
        Value::SignedBigInt(i128::MIN),
        Value::SignedBigInt(-1),
        Order::Float,
    )]
    #[case(
        Value::SignedInt(0),
        Value::SignedInt(-1),
        Order::SignedInt
    )]
    #[case(Value::UnsignedInt(0), Value::UnsignedInt(10), Order::UnsignedInt)]
    #[case(Value::UnsignedInt(0), Value::SignedInt(10), Order::SignedInt)]
    #[case(Value::Float(10.0), Value::Float(2.0), Order::Float)]
    #[case(Value::Float(10.0), Value::UnsignedInt(2), Order::Float)]
    #[case(Value::Float(10.0), Value::UnsignedInt(u64::MAX), Order::Float)]
    #[case(
        Value::Float(-f64::MAX),
        Value::UnsignedInt(2),
        Order::Float
    )]
    fn division(#[case] mut left: Value, #[case] right: Value, #[case] expect: Order) {
        let result = left / right;
        assert_eq!(
            result.order(),
            expect,
            "expected {expect:?} got = {result:?}",
        );
        left /= right;
        assert_eq!(left.order(), expect, "expected {expect:?} got = {left:?}");
    }

    #[rstest]
    #[case(Value::SignedInt(10), Value::SignedInt(2), Order::SignedInt)]
    #[case(Value::SignedInt(i64::MIN), Value::SignedInt(-1), Order::SignedBigInt)]
    #[case(Value::SignedBigInt(i128::MIN), Value::SignedBigInt(-1), Order::Float)]
    fn remainder(#[case] mut left: Value, #[case] right: Value, #[case] expect: Order) {
        let result = left % right;
        assert_eq!(result.order(), expect, "expected {expect:?} got {result:?}");
        left %= right;
        assert_eq!(left.order(), expect, "expected {expect:?} got {left:?}");
    }

    #[test]
    #[should_panic]
    fn divide_by_zero_panics() {
        _ = Value::SignedInt(-1) / Value::SignedInt(0);
    }
    */
}
