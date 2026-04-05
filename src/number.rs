use bigdecimal::{BigDecimal, ParseBigDecimalError};
use core::fmt;
use num_bigint::BigInt;
use std::{
    cmp::Ordering,
    error,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
    str::FromStr,
};

#[derive(Debug, Clone)]
pub enum Number {
    Int(BigInt),
    Decimal(BigDecimal),
}

impl Number {
    pub fn from_f64(n: f64) -> Result<Self, NumberError> {
        Self::try_from(n)
    }

    /// Sets the scale only on Number::Decimal
    pub fn set_scale(&mut self, scale: i64) {
        if let Self::Decimal(n) = self {
            *n = n.with_scale(scale);
        }
    }

    pub(crate) fn order(&self) -> NumberOrder {
        NumberOrder::from(self)
    }

    pub(crate) fn match_order(&mut self, other: &mut Self) {
        match self.order().cmp(&other.order()) {
            Ordering::Less => self.promote(),
            Ordering::Greater => other.promote(),
            Ordering::Equal => {}
        }
    }

    /// Converts Number::Int to Number::Decimal.
    /// Number::Decimal is already the highest 'order'.
    pub(crate) fn promote(&mut self) {
        if let Some(n) = self.take_int() {
            *self = Self::Decimal(BigDecimal::from(n));
        }
    }

    /// Takes the backing BigInt leaivng 0 in it's place.
    pub(crate) fn take_int(&mut self) -> Option<BigInt> {
        if let Self::Int(n) = self {
            return Some(std::mem::take(n));
        }
        None
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
// ========================== Add ============================================================
// ===========================================================================================

impl<Rhs> AddAssign<Rhs> for Number
where
    Rhs: Into<Number>,
{
    fn add_assign(&mut self, rhs: Rhs) {
        let mut rhs = rhs.into();
        self.match_order(&mut rhs);

        *self = match (&self, &rhs) {
            (Number::Decimal(x), Number::Decimal(y)) => Number::Decimal(x + y),
            (Number::Int(x), Number::Int(y)) => Number::Int(x + y),
            _ => unreachable!("we know orders match"),
        }
    }
}

impl AddAssign<&Number> for Number {
    fn add_assign(&mut self, rhs: &Number) {
        *self = &*self + rhs;
    }
}

impl<Rhs> Add<Rhs> for Number
where
    Rhs: Into<Number>,
{
    type Output = Number;

    fn add(mut self, rhs: Rhs) -> Self::Output {
        self.add_assign(rhs);
        self
    }
}

impl<'a> Add<&'a Number> for &Number {
    type Output = Number;

    fn add(self, rhs: &'a Number) -> Self::Output {
        let mut lhs = self.clone();
        lhs += rhs.clone();
        lhs
    }
}

// ===========================================================================================
// ========================== Sub ============================================================
// ===========================================================================================

impl<Rhs> SubAssign<Rhs> for Number
where
    Rhs: Into<Number>,
{
    fn sub_assign(&mut self, rhs: Rhs) {
        let mut rhs = rhs.into();
        self.match_order(&mut rhs);

        *self = match (&self, &rhs) {
            (Number::Decimal(x), Number::Decimal(y)) => Number::Decimal(x - y),
            (Number::Int(x), Number::Int(y)) => Number::Int(x - y),
            _ => unreachable!("we know orders match"),
        }
    }
}

impl SubAssign<&Number> for Number {
    fn sub_assign(&mut self, rhs: &Number) {
        *self = &*self - rhs;
    }
}

impl<Rhs> Sub<Rhs> for Number
where
    Rhs: Into<Number>,
{
    type Output = Number;

    fn sub(mut self, rhs: Rhs) -> Self::Output {
        self.sub_assign(rhs);
        self
    }
}

impl<'a> Sub<&'a Number> for &Number {
    type Output = Number;

    fn sub(self, rhs: &'a Number) -> Self::Output {
        let mut lhs = self.clone();
        lhs -= rhs.clone();
        lhs
    }
}

// ===========================================================================================
// ========================== Div ============================================================
// ===========================================================================================

impl<Rhs> DivAssign<Rhs> for Number
where
    Rhs: Into<Number>,
{
    fn div_assign(&mut self, rhs: Rhs) {
        let mut rhs = rhs.into();
        self.match_order(&mut rhs);

        *self = match (&self, &rhs) {
            (Number::Decimal(x), Number::Decimal(y)) => Number::Decimal(x / y),
            (Number::Int(x), Number::Int(y)) => {
                if x % y == BigInt::ZERO {
                    Number::Int(x / y)
                } else {
                    let l = BigDecimal::from_bigint(self.take_int().unwrap_or_default(), 0);
                    let r = BigDecimal::from_bigint(rhs.take_int().unwrap_or_default(), 0);
                    Number::Decimal(l / r)
                }
            }
            _ => unreachable!("we know orders match"),
        }
    }
}

impl DivAssign<&Number> for Number {
    fn div_assign(&mut self, rhs: &Number) {
        *self = &*self / rhs;
    }
}

impl<Rhs> Div<Rhs> for Number
where
    Rhs: Into<Number>,
{
    type Output = Number;

    fn div(mut self, rhs: Rhs) -> Self::Output {
        self.div_assign(rhs);
        self
    }
}

impl<'a> Div<&'a Number> for &Number {
    type Output = Number;

    fn div(self, rhs: &'a Number) -> Self::Output {
        let mut lhs = self.clone();
        lhs /= rhs.clone();
        lhs
    }
}

// ===========================================================================================
// ========================== Mul ============================================================
// ===========================================================================================

impl<Rhs> MulAssign<Rhs> for Number
where
    Rhs: Into<Number>,
{
    fn mul_assign(&mut self, rhs: Rhs) {
        let mut rhs = rhs.into();
        self.match_order(&mut rhs);

        *self = match (&self, &rhs) {
            (Number::Decimal(x), Number::Decimal(y)) => Number::Decimal(x * y),
            (Number::Int(x), Number::Int(y)) => Number::Int(x * y),
            _ => unreachable!("we know orders match"),
        }
    }
}

impl MulAssign<&Number> for Number {
    fn mul_assign(&mut self, rhs: &Number) {
        *self = &*self * rhs;
    }
}

impl<Rhs> Mul<Rhs> for Number
where
    Rhs: Into<Number>,
{
    type Output = Number;

    fn mul(mut self, rhs: Rhs) -> Self::Output {
        self.mul_assign(rhs);
        self
    }
}

impl<'a> Mul<&'a Number> for &Number {
    type Output = Number;

    fn mul(self, rhs: &'a Number) -> Self::Output {
        let mut lhs = self.clone();
        lhs *= rhs.clone();
        lhs
    }
}

// ===========================================================================================
// ========================== Rem ============================================================
// ===========================================================================================

impl<Rhs> RemAssign<Rhs> for Number
where
    Rhs: Into<Number>,
{
    fn rem_assign(&mut self, rhs: Rhs) {
        let mut rhs = rhs.into();
        self.match_order(&mut rhs);

        *self = match (&self, &rhs) {
            (Number::Decimal(x), Number::Decimal(y)) => Number::Decimal(x % y),
            (Number::Int(x), Number::Int(y)) => Number::Int(x % y),
            _ => unreachable!("we know orders match"),
        }
    }
}

impl RemAssign<&Number> for Number {
    fn rem_assign(&mut self, rhs: &Number) {
        *self = &*self % rhs;
    }
}

impl<Rhs> Rem<Rhs> for Number
where
    Rhs: Into<Number>,
{
    type Output = Number;

    fn rem(mut self, rhs: Rhs) -> Self::Output {
        self.rem_assign(rhs);
        self
    }
}

impl<'a> Rem<&'a Number> for &Number {
    type Output = Number;

    fn rem(self, rhs: &'a Number) -> Self::Output {
        let mut lhs = self.clone();
        lhs %= rhs.clone();
        lhs
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
// ========================== NumberError ====================================================
// ===========================================================================================

#[derive(Debug, Clone)]
pub enum NumberError {
    Parsing { value: String },
}

impl fmt::Display for NumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumberError::Parsing { value } => write!(f, "Error parsing value : {value}"),
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
// ========================== Tests ==========================================================
// ===========================================================================================

#[cfg(test)]
mod test {
    use super::*;

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

    #[test]
    fn add_decimals() {
        let x = Number::from_f64(1.1).unwrap();
        assert_eq!(x.order(), NumberOrder::Decimal);
        let y = Number::from_f64(2.2).unwrap();
        assert_eq!(y.order(), NumberOrder::Decimal);
        let r = x + y;
        assert_eq!(r.order(), NumberOrder::Decimal);
        let e = Number::from_f64(3.3).unwrap();
        assert_eq!(r, e, "expected {e} got {r}");
    }

    #[test]
    fn sub_decimals() {
        let x = Number::from_f64(5.5).unwrap();
        assert_eq!(x.order(), NumberOrder::Decimal);
        let y = Number::from_f64(2.2).unwrap();
        assert_eq!(y.order(), NumberOrder::Decimal);
        let r = x - y;
        assert_eq!(r.order(), NumberOrder::Decimal);
        let e = Number::from_f64(3.3).unwrap();
        assert_eq!(r, e, "expected {e} got {r}");
    }

    #[test]
    fn mul_decimals() {
        let x = Number::from_f64(5.5).unwrap();
        assert_eq!(x.order(), NumberOrder::Decimal);
        let y = Number::from_f64(2.2).unwrap();
        assert_eq!(y.order(), NumberOrder::Decimal);
        let r = x * y;
        assert_eq!(r.order(), NumberOrder::Decimal);
        let e = Number::from_f64(12.1).unwrap();
        assert_eq!(r, e, "expected {e} got {r}");
    }

    #[test]
    // If two integers div produces a decimal, output should be Number::Decimal
    fn div_result_is_decimal() {
        let x = Number::Int(1.into());
        let y = Number::Int(2.into());
        let r = x / y;
        assert_eq!(r.order(), NumberOrder::Decimal);
        let e = Number::from_f64(0.5).unwrap();
        assert_eq!(r, e, "expected {e} got {r}");
    }

    #[test]
    fn div_int_by_decimal() {
        let x = Number::Int(1.into());
        let y = Number::from_f64(2.2).unwrap();
        assert_eq!(y.order(), NumberOrder::Decimal);
        let r = x / y;
        assert_eq!(r.order(), NumberOrder::Decimal);
        let estr = "0.4545454545454545454545454545454545454545454545454545454545454545454545454545454545454545454545454545";
        let e = Number::from_str(estr).unwrap();
        assert_eq!(r, e, "expected {e} got {r}",);
    }

    #[test]
    // modulo
    fn rem_decimals() {
        let x = Number::from_f64(5.6).unwrap();
        assert_eq!(x.order(), NumberOrder::Decimal);
        let y = Number::from_f64(3.2).unwrap();
        assert_eq!(x.order(), NumberOrder::Decimal);
        let r = x % y;
        let e = Number::from_f64(2.4).unwrap();
        assert_eq!(r, e, "expected {e} got {r}");
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
