use bigdecimal::{BigDecimal, ParseBigDecimalError};
use core::fmt;
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use std::{
    cmp::Ordering,
    error,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
    str::FromStr,
};

#[derive(Clone)]
pub enum Number {
    Int(BigInt),
    Decimal(BigDecimal),
}

impl Number {
    pub fn from_f64(n: f64) -> Result<Self, NumberError> {
        Self::try_from(n)
    }

    pub fn to_i64(&self) -> Option<i64> {
        match self {
            Number::Int(big_int) => big_int.to_i64(),
            Number::Decimal(_) => None,
        }
    }

    pub fn to_i32(&self) -> Option<i32> {
        match self {
            Number::Int(big_int) => big_int.to_i32(),
            Number::Decimal(_) => None,
        }
    }

    pub fn pow(&self, exponent: i64) -> Result<Self, NumberError> {
        match self {
            Number::Decimal(big_decimal) => Ok(Number::Decimal(big_decimal.powi(exponent))),
            Number::Int(big_int) => {
                let exponent_u32: u32 = exponent.try_into().map_err(|_| {
                    let m = format!("Number::Int exponent must fit in u32: {exponent} does not!");
                    NumberError::InvalidExponent { message: m }
                })?;
                Ok(Number::Int(big_int.pow(exponent_u32)))
            }
        }
    }

    /// Sets the scale only on Number::Decimal
    pub fn set_scale(&mut self, scale: i64) {
        if let Self::Decimal(n) = self {
            *n = n.with_scale(scale);
        }
    }

    /// Sets the scale and rounding mode only on Number::Decimal
    pub fn set_scale_round(&mut self, scale: i64, rounding_mode: bigdecimal::RoundingMode) {
        if let Self::Decimal(n) = self {
            *n = n.with_scale_round(scale, rounding_mode);
        }
    }

    pub fn order(&self) -> NumberOrder {
        NumberOrder::from(self)
    }

    pub fn match_order(&mut self, other: &mut Self) {
        match self.order().cmp(&other.order()) {
            Ordering::Less => self.promote(),
            Ordering::Greater => other.promote(),
            Ordering::Equal => {}
        }
    }

    /// Converts Number::Int to Number::Decimal.
    /// Number::Decimal is already the highest 'order'.
    pub fn promote(&mut self) {
        if let Some(n) = self.take_int() {
            *self = Self::Decimal(BigDecimal::from(n));
        }
    }

    /// Takes the backing BigInt leaivng 0 in it's place.
    pub fn take_int(&mut self) -> Option<BigInt> {
        if let Self::Int(n) = self {
            return Some(std::mem::take(n));
        }
        None
    }
}

// ===========================================================================================
// ========================== ToNumber =======================================================
// ===========================================================================================

pub trait ToNumber {
    fn to_number(&self) -> Number;
}

macro_rules! impl_to_number {
    ($t:ty) => {
        impl ToNumber for $t {
            fn to_number(&self) -> Number {
                Number::from(*self)
            }
        }
    };
}

impl_to_number!(u8);
impl_to_number!(u16);
impl_to_number!(u32);
impl_to_number!(u64);
impl_to_number!(u128);
impl_to_number!(i8);
impl_to_number!(i16);
impl_to_number!(i32);
impl_to_number!(i64);
impl_to_number!(i128);

impl ToNumber for f64 {
    fn to_number(&self) -> Number {
        Number::from_f64(*self).expect("Number")
    }
}

impl ToNumber for BigInt {
    fn to_number(&self) -> Number {
        Number::from(self)
    }
}

impl ToNumber for BigDecimal {
    fn to_number(&self) -> Number {
        Number::from(self)
    }
}

// ===========================================================================================
// ========================== From ===========================================================
// ===========================================================================================

macro_rules! impl_number_from {
    ($t:ty) => {
        impl From<$t> for Number {
            fn from(value: $t) -> Self {
                Number::Int(BigInt::from(value))
            }
        }

        impl From<&$t> for Number
        where
            $t: Copy,
        {
            fn from(value: &$t) -> Self {
                Number::Int(BigInt::from(*value))
            }
        }
    };
}

impl_number_from!(u8);
impl_number_from!(u16);
impl_number_from!(u32);
impl_number_from!(u64);
impl_number_from!(u128);
impl_number_from!(i8);
impl_number_from!(i16);
impl_number_from!(i32);
impl_number_from!(i64);
impl_number_from!(i128);

impl From<BigDecimal> for Number {
    fn from(value: BigDecimal) -> Self {
        Number::Decimal(value)
    }
}

/// Clones the value!!
impl From<&BigDecimal> for Number {
    fn from(value: &BigDecimal) -> Self {
        Number::Decimal(value.clone())
    }
}

impl From<BigInt> for Number {
    fn from(value: BigInt) -> Self {
        Number::Int(value)
    }
}

/// Clones the value!!
impl From<&BigInt> for Number {
    fn from(value: &BigInt) -> Self {
        Number::Int(value.clone())
    }
}

// ===========================================================================================
// ========================== TryFrom ========================================================
// ===========================================================================================

impl TryFrom<f64> for Number {
    type Error = NumberError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        let bd = BigDecimal::from_str(&value.to_string())?;
        Ok(Number::Decimal(bd))
    }
}

// ===========================================================================================
// ========================== FromStr ========================================================
// ===========================================================================================

impl FromStr for Number {
    type Err = NumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(".") {
            return s
                .parse::<BigDecimal>()
                .map(Self::Decimal)
                .map_err(|_| NumberError::Parsing {
                    value: s.to_string(),
                });
        }
        s.parse::<BigInt>()
            .map(Self::Int)
            .map_err(|_| NumberError::Parsing {
                value: s.to_string(),
            })
    }
}

// ===========================================================================================
// ========================== Debug ==========================================================
// ===========================================================================================

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(i) => write!(f, "Number::Int({i})"),
            Self::Decimal(d) => write!(f, "Number::Decimal({d})"),
        }
    }
}

// ===========================================================================================
// ========================== Display ========================================================
// ===========================================================================================

impl fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Int(big_int) => write!(f, "{big_int}"),
            Number::Decimal(big_decimal) => write!(f, "{big_decimal}"),
        }
    }
}

// ===========================================================================================
// ========================== Macros for implementing arithmetic =============================
// ===========================================================================================

/// Expects `$lhs` to be `&Number`
/// Expects `$rhs` to be `&Number`
/// Expects `$op` to be an operator (+, -, /, *, %)
macro_rules! match_arithmetic {
    ($lhs:expr, $rhs:expr, $op:tt) => {
        match ($lhs, $rhs) {
            (Number::Int(x), Number::Int(y)) => Number::Int(x $op y),
            (Number::Decimal(x), Number::Decimal(y)) => Number::Decimal(x $op y),
            (Number::Int(x), Number::Decimal(y)) => {
                let x = BigDecimal::from_bigint(x.clone(), 0);
                Number::Decimal(x $op y)
            }
            (Number::Decimal(x), Number::Int(y)) => {
                let y = BigDecimal::from_bigint(y.clone(), 0);
                Number::Decimal(x $op y)
            }
        }
    };
}

/// Expects `$lhs` to be `&mut Number`
/// Expects `$rhs` to be `&Number`
/// Expects `$op` to be an operator (+, -, /, *, %)
macro_rules! match_arithmetic_assign {
    ($lhs:expr, $rhs:expr, $op:tt) => {
        *$lhs = match (&$lhs, $rhs) {
            (Number::Int(x), Number::Int(y)) => Number::Int(x $op y),
            (Number::Decimal(x), Number::Decimal(y)) => Number::Decimal(x $op y),
            (Number::Decimal(x), Number::Int(y)) => {
                let y = BigDecimal::from_bigint(y.clone(), 0);
                Number::Decimal(x $op y)
            }
            (Number::Int(_), Number::Decimal(_)) => {
                $lhs.promote();
                &*$lhs $op $rhs
            }
        }
    };
}

// ===========================================================================================
// ========================== AddAssign/Add ==================================================
// ===========================================================================================

impl AddAssign<Number> for Number {
    fn add_assign(&mut self, rhs: Number) {
        self.add_assign(&rhs);
    }
}

impl AddAssign<&Number> for Number {
    fn add_assign(&mut self, rhs: &Number) {
        match_arithmetic_assign!(self, rhs, +);
    }
}

impl Add<Number> for Number {
    type Output = Number;

    fn add(mut self, rhs: Number) -> Self::Output {
        self.add_assign(&rhs);
        self
    }
}

impl Add<&Number> for &Number {
    type Output = Number;

    fn add(self, rhs: &Number) -> Self::Output {
        match_arithmetic!(self, rhs, +)
    }
}

// ===========================================================================================
// ========================== SubAssign/Sub ==================================================
// ===========================================================================================

impl SubAssign<Number> for Number {
    fn sub_assign(&mut self, rhs: Number) {
        self.sub_assign(&rhs);
    }
}

impl SubAssign<&Number> for Number {
    fn sub_assign(&mut self, rhs: &Number) {
        match_arithmetic_assign!(self, rhs, -);
    }
}

impl Sub<Number> for Number {
    type Output = Number;

    fn sub(mut self, rhs: Number) -> Self::Output {
        self.sub_assign(&rhs);
        self
    }
}

impl Sub<&Number> for &Number {
    type Output = Number;

    fn sub(self, rhs: &Number) -> Self::Output {
        match_arithmetic!(self, rhs, -)
    }
}

// ===========================================================================================
// ========================== DivAssign/Div ==================================================
// ===========================================================================================

impl DivAssign<Number> for Number {
    fn div_assign(&mut self, rhs: Number) {
        self.div_assign(&rhs);
    }
}

impl DivAssign<&Number> for Number {
    fn div_assign(&mut self, rhs: &Number) {
        *self = match (&self, rhs) {
            (Number::Decimal(x), Number::Decimal(y)) => Number::Decimal(x / y),
            // If integer division does not produce a decimal.
            (Number::Int(x), Number::Int(y)) if x % y == BigInt::ZERO => Number::Int(x / y),
            // If integer division would produce a decimal, convert result to Decimal.
            (Number::Int(_), Number::Int(y)) => {
                let l = BigDecimal::from_bigint(self.take_int().expect("Number::Int"), 0);
                let r = BigDecimal::from_bigint(y.clone(), 0);
                Number::Decimal(l / r)
            }
            (Number::Decimal(x), Number::Int(y)) => {
                let y = BigDecimal::from_bigint(y.clone(), 0);
                Number::Decimal(x / y)
            }
            (Number::Int(_), Number::Decimal(_)) => {
                self.promote();
                &*self / rhs
            }
        }
    }
}

impl Div<Number> for Number {
    type Output = Number;

    fn div(mut self, rhs: Number) -> Self::Output {
        self.div_assign(&rhs);
        self
    }
}

impl Div<&Number> for &Number {
    type Output = Number;

    fn div(self, rhs: &Number) -> Self::Output {
        match (self, rhs) {
            (Number::Decimal(x), Number::Decimal(y)) => Number::Decimal(x / y),
            // If integer division does not produce a decimal.
            (Number::Int(x), Number::Int(y)) if x % y == BigInt::ZERO => Number::Int(x / y),
            // If integer division would produce a decimal, convert result to Decimal
            (Number::Int(x), Number::Int(y)) => {
                let l = BigDecimal::from_bigint(x.clone(), 0);
                let r = BigDecimal::from_bigint(y.clone(), 0);
                Number::Decimal(l / r)
            }
            (Number::Int(x), Number::Decimal(y)) => {
                let x = BigDecimal::from_bigint(x.clone(), 0);
                Number::Decimal(x / y)
            }
            (Number::Decimal(x), Number::Int(y)) => {
                let y = BigDecimal::from_bigint(y.clone(), 0);
                Number::Decimal(x / y)
            }
        }
    }
}

// ===========================================================================================
// ========================== MulAssign/Mul ==================================================
// ===========================================================================================

impl MulAssign<Number> for Number {
    fn mul_assign(&mut self, rhs: Number) {
        self.mul_assign(&rhs);
    }
}

impl MulAssign<&Number> for Number {
    fn mul_assign(&mut self, rhs: &Number) {
        match_arithmetic_assign!(self, rhs, *);
    }
}

impl Mul<Number> for Number {
    type Output = Number;

    fn mul(mut self, rhs: Number) -> Self::Output {
        self.mul_assign(&rhs);
        self
    }
}

impl Mul<&Number> for &Number {
    type Output = Number;

    fn mul(self, rhs: &Number) -> Self::Output {
        match_arithmetic!(self, rhs, *)
    }
}

// ===========================================================================================
// ========================== RemAssign/Rem ==================================================
// ===========================================================================================

impl RemAssign<Number> for Number {
    fn rem_assign(&mut self, rhs: Number) {
        self.rem_assign(&rhs);
    }
}

impl RemAssign<&Number> for Number {
    fn rem_assign(&mut self, rhs: &Number) {
        match_arithmetic_assign!(self, rhs, %);
    }
}

impl Rem<Number> for Number {
    type Output = Number;

    fn rem(mut self, rhs: Number) -> Self::Output {
        self.rem_assign(&rhs);
        self
    }
}

impl Rem<&Number> for &Number {
    type Output = Number;

    fn rem(self, rhs: &Number) -> Self::Output {
        match_arithmetic!(self, rhs, %)
    }
}

// ===========================================================================================
// ========================== PartialEq/Eq ===================================================
// ===========================================================================================

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l), Self::Int(r)) => l == r,
            (Self::Decimal(l), Self::Decimal(r)) => l == r,
            _ => false,
        }
    }
}

impl Eq for Number {}

// ===========================================================================================
// ========================== NumberError ====================================================
// ===========================================================================================

#[derive(Debug, Clone)]
pub enum NumberError {
    Parsing { value: String },
    InvalidExponent { message: String },
}

impl fmt::Display for NumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumberError::Parsing { value } => write!(f, "Error parsing value : {value}"),
            NumberError::InvalidExponent { message } => write!(f, "{message}"),
        }
    }
}

impl From<ParseBigDecimalError> for NumberError {
    fn from(value: ParseBigDecimalError) -> Self {
        Self::Parsing {
            value: value.to_string(),
        }
    }
}

impl error::Error for NumberError {}

// ===========================================================================================
// ========================== NumberOrder ====================================================
// ===========================================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NumberOrder {
    Int,
    Decimal,
}

impl From<Number> for NumberOrder {
    fn from(value: Number) -> Self {
        match value {
            Number::Int(_) => Self::Int,
            Number::Decimal(_) => Self::Decimal,
        }
    }
}

impl From<&Number> for NumberOrder {
    fn from(value: &Number) -> Self {
        match value {
            Number::Int(_) => Self::Int,
            Number::Decimal(_) => Self::Decimal,
        }
    }
}

// ===========================================================================================
// ========================== Tests ==========================================================
// ===========================================================================================

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;
    use rstest::*;

    #[test]
    fn from_str() {
        let a = Number::from_str("1.1").unwrap();
        assert_eq!(a.order(), NumberOrder::Decimal);
    }

    #[test]
    fn from_f64() {
        let a = Number::from_f64(1.1).unwrap();
        assert_eq!(a.order(), NumberOrder::Decimal);
    }

    #[rstest]
    #[case::add1(1, 1, 2)]
    #[case::add2(1.1, 2.2, 3.3)]
    #[case::add3(1.1, 2, 3.1)]
    #[case::add4(2, 1.1, 3.1)]
    fn add(#[case] lhs: impl ToNumber, #[case] rhs: impl ToNumber, #[case] expect: impl ToNumber) {
        let x = lhs.to_number();
        let y = rhs.to_number();
        let e = expect.to_number();
        let r = x + y;
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::add_assign1(1, 1, 2)]
    #[case::add_assign2(1.1, 2.2, 3.3)]
    #[case::add_assign3(1.1, 2, 3.1)]
    #[case::add_assign4(2, 1.1, 3.1)]
    fn add_assign(
        #[case] lhs: impl ToNumber,
        #[case] rhs: impl ToNumber,
        #[case] expect: impl ToNumber,
    ) {
        let mut x = lhs.to_number();
        let y = rhs.to_number();
        let e = expect.to_number();
        x += y;
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[rstest]
    #[case::sub1(1, 1, 0)]
    #[case::sub2(1.1, 2.2, -1.1)]
    #[case::sub3(2, 1.1, 0.9)]
    #[case::sub4(100, 47.4567, 52.5433)]
    #[case::sub5(5.5, 2.2, 3.3)]
    fn sub(#[case] lhs: impl ToNumber, #[case] rhs: impl ToNumber, #[case] expect: impl ToNumber) {
        let x = lhs.to_number();
        let y = rhs.to_number();
        let e = expect.to_number();
        let r = x - y;
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::sub_assign1(1, 1, 0)]
    #[case::sub_assign2(1.1, 2.2, -1.1)]
    #[case::sub_assign3(2, 1.1, 0.9)]
    #[case::sub_assign4(100, 47.4567, 52.5433)]
    #[case::sub_assign5(5.5, 2.2, 3.3)]
    fn sub_assign(
        #[case] lhs: impl ToNumber,
        #[case] rhs: impl ToNumber,
        #[case] expect: impl ToNumber,
    ) {
        let mut x = lhs.to_number();
        let y = rhs.to_number();
        let e = expect.to_number();
        x -= y;
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[rstest]
    #[case::mul1(1, 1, 1)]
    #[case::mul2(1.1, 2.2, 2.42)]
    #[case::mul3(2, 1.1, 2.2)]
    #[case::mul4(47.4567, 100, 4745.67)]
    #[case::mul5(55, 22, 1210)]
    #[case::mul6(5.7, 2, 11.4)]
    fn mul(#[case] lhs: impl ToNumber, #[case] rhs: impl ToNumber, #[case] expect: impl ToNumber) {
        let x = lhs.to_number();
        let y = rhs.to_number();
        let e = expect.to_number();
        let r = x * y;
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::mul_assign1(1, 1, 1)]
    #[case::mul_assign2(1.1, 2.2, 2.42)]
    #[case::mul_assign3(2, 1.1, 2.2)]
    #[case::mul_assign4(47.4567, 100, 4745.67)]
    #[case::mul_assign5(55, 22, 1210)]
    #[case::mul_assign6(5.7, 2, 11.4)]
    fn mul_assign(
        #[case] lhs: impl ToNumber,
        #[case] rhs: impl ToNumber,
        #[case] expect: impl ToNumber,
    ) {
        let mut x = lhs.to_number();
        let y = rhs.to_number();
        let e = expect.to_number();
        x *= y;
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[rstest]
    #[case::div1(1, 1, 1)]
    #[case::div2(1.1, 2.2, 0.5)]
    #[case::div3(2, 1.1, 1.81818181818)]
    #[case::div4(100, 47, 2.12765957447)]
    #[case::div5(55, 5, 11)]
    #[case::div6(5.7, 2, 2.85)]
    fn div(#[case] lhs: impl ToNumber, #[case] rhs: impl ToNumber, #[case] expect: impl ToNumber) {
        let x = lhs.to_number();
        let y = rhs.to_number();
        let e = expect.to_number();
        let mut r = x / y;
        r.set_scale_round(11, bigdecimal::RoundingMode::HalfUp);
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::div_assign1(1, 1, 1)]
    #[case::div_assign2(1.1, 2.2, 0.5)]
    #[case::div_assign3(2, 1.1, 1.81818181818)]
    #[case::div_assign4(100, 47, 2.12765957447)]
    #[case::div_assign5(55, 5, 11)]
    #[case::div_assign6(5.7, 2, 2.85)]
    fn div_assign(
        #[case] lhs: impl ToNumber,
        #[case] rhs: impl ToNumber,
        #[case] expect: impl ToNumber,
    ) {
        let mut x = lhs.to_number();
        let y = rhs.to_number();
        let e = expect.to_number();
        x /= y;
        x.set_scale_round(11, RoundingMode::HalfUp);
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[rstest]
    #[case::rem1(1, 1, 0)]
    #[case::rem2(1.1, 2.2, 1.1)]
    #[case::rem3(2, 1.1, 0.9)]
    #[case::rem4(100, 47, 6)]
    #[case::rem5(55, 5, 0)]
    #[case::rem6(5.7, 2, 1.7)]
    #[case::rem7(5.6, 3.2, 2.4)]
    #[case::rem8(5.6, 2, 1.6)]
    fn rem(#[case] lhs: impl ToNumber, #[case] rhs: impl ToNumber, #[case] expect: impl ToNumber) {
        let x = lhs.to_number();
        let y = rhs.to_number();
        let e = expect.to_number();
        let r = x % y;
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::rem_assign1(1, 1, 0)]
    #[case::rem_assign2(1.1, 2.2, 1.1)]
    #[case::rem_assign3(2, 1.1, 0.9)]
    #[case::rem_assign4(100, 47, 6)]
    #[case::rem_assign5(55, 5, 0)]
    #[case::rem_assign6(5.7, 2, 1.7)]
    #[case::rem_assign7(5.6, 3.2, 2.4)]
    #[case::rem_assign8(5.6, 2, 1.6)]
    fn rem_assign(
        #[case] lhs: impl ToNumber,
        #[case] rhs: impl ToNumber,
        #[case] expect: impl ToNumber,
    ) {
        let mut x = lhs.to_number();
        let y = rhs.to_number();
        let e = expect.to_number();
        x %= y;
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[test]
    fn very_large_ints() {
        let astr = "57896044618658097711785492504343953926634992332820282019728792003956564819968";
        let a = Number::from_str(astr).unwrap();
        let b = Number::Int((-1).into());
        let r = a / b;
        let estr = "-57896044618658097711785492504343953926634992332820282019728792003956564819968";
        let e = Number::from_str(estr).unwrap();
        assert_eq!(r, e, "expected {e} got {r}");
    }
}
