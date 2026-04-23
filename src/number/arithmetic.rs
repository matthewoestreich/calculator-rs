use crate::{Number, NumberError};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{ConstZero, Num, One, Signed, Zero};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

impl Number {
    /// Fallible API for [`impl DivAssign<&Number> for Number`](crate::Number#impl-DivAssign<%26Number>-for-Number).
    ///
    /// ```rust
    /// use calcinum::{Number, NumberError};
    ///
    /// let mut x = Number::from(1);
    /// let y = Number::ZERO;
    /// assert_eq!(x.try_div_assign(&y), Err(NumberError::DivisionByZero));
    /// ```
    pub fn try_div_assign(&mut self, rhs: &Number) -> Result<(), NumberError> {
        if rhs.is_zero() {
            return Err(NumberError::DivisionByZero);
        }

        *self = match (&self, rhs) {
            // Delegate to `impl Div<&Number> for &Number` since we would call the same code regardless
            (Number::Decimal(_), Number::Decimal(_)) | (Number::Decimal(_), Number::Int(_)) => {
                &*self / rhs
            }
            (Number::Int(_), Number::Decimal(_)) => {
                self.promote(); // Both sides must be Number::Decimal
                &*self / rhs
            }
            (Number::Int(x), Number::Int(y)) => {
                if x % y != BigInt::ZERO {
                    // There is a remainder, need to convert both sides to `Number::Decimal`
                    // so we perform decimal division, not integer division.
                    self.promote();
                    &*self / rhs
                } else {
                    // Integer division would not produce a remainder, ok to use integer division.
                    Number::Int(x / y)
                }
            }
        };

        Ok(())
    }

    /// Fallible API for [`impl Div<&Number> for &Number`](crate::Number#impl-Div<%26Number>-for-%26Number).
    ///
    /// ```rust
    /// use calcinum::{Number, NumberError};
    ///
    /// let mut x = Number::from(1);
    /// let y = Number::ZERO;
    /// assert_eq!(x.try_div(&y), Err(NumberError::DivisionByZero));
    /// ```
    pub fn try_div(&self, rhs: &Number) -> Result<Number, NumberError> {
        if rhs.is_zero() {
            return Err(NumberError::DivisionByZero);
        }

        let result = match (self, rhs) {
            (Number::Decimal(x), Number::Decimal(y)) => Number::Decimal(x / y),
            (Number::Int(x), Number::Int(y)) => {
                if x % y != BigInt::ZERO {
                    // There is a remainder, need to convert both sides to
                    // `Number::Decimal` so we perform decimal division, not integer division.
                    let l = BigDecimal::from(x.clone());
                    let r = BigDecimal::from(y.clone());
                    Number::Decimal(l / r)
                } else {
                    Number::Int(x / y)
                }
            }
            (Number::Int(x), Number::Decimal(y)) => {
                let x = BigDecimal::from(x.clone());
                Number::Decimal(x / y)
            }
            (Number::Decimal(x), Number::Int(y)) => {
                let y = BigDecimal::from(y.clone());
                Number::Decimal(x / y)
            }
        };

        Ok(result)
    }

    /// Returns the quotient and remainder as a tuple, e.g., `(quotient, remainder)`
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let dividend = Number::from(10);
    /// let divisor = 7;
    /// let (expect_quotient, expect_remainder) = (Number::from(1), Number::from(3));
    /// let (result_quotient, result_remainder) = dividend.div_mod(divisor);
    /// assert_eq!(
    ///     (expect_quotient, expect_remainder),
    ///     (result_quotient, result_remainder),
    /// );
    /// ```
    pub fn div_mod<T>(&self, rhs: T) -> (Number, Number)
    where
        T: Into<Number>,
    {
        let rhs = rhs.into();
        let remainder = self % &rhs;
        let mut quotient = (self / &rhs).floor();
        // If the quotient is a whole number, this will change the variant
        // from `Number::Decimal(_)` to `Number::Int(_)`.
        quotient.demote();
        (quotient, remainder)
    }
}

// ===========================================================================================
// ========================== AddAssign/Add ==================================================
// ===========================================================================================

impl_add!(u8);
impl_add!(u16);
impl_add!(u32);
impl_add!(u64);
impl_add!(u128);
impl_add!(i8);
impl_add!(i16);
impl_add!(i32);
impl_add!(i64);
impl_add!(i128);

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

impl_sub!(u8);
impl_sub!(u16);
impl_sub!(u32);
impl_sub!(u64);
impl_sub!(u128);
impl_sub!(i8);
impl_sub!(i16);
impl_sub!(i32);
impl_sub!(i64);
impl_sub!(i128);

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

impl_div!(u8);
impl_div!(u16);
impl_div!(u32);
impl_div!(u64);
impl_div!(u128);
impl_div!(i8);
impl_div!(i16);
impl_div!(i32);
impl_div!(i64);
impl_div!(i128);

impl DivAssign<Number> for Number {
    fn div_assign(&mut self, rhs: Number) {
        self.div_assign(&rhs);
    }
}

impl DivAssign<&Number> for Number {
    fn div_assign(&mut self, rhs: &Number) {
        self.try_div_assign(rhs).expect("div_assign");
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
        self.try_div(rhs).expect("div")
    }
}

// ===========================================================================================
// ========================== MulAssign/Mul ==================================================
// ===========================================================================================

impl_mul!(u8);
impl_mul!(u16);
impl_mul!(u32);
impl_mul!(u64);
impl_mul!(u128);
impl_mul!(i8);
impl_mul!(i16);
impl_mul!(i32);
impl_mul!(i64);
impl_mul!(i128);

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

impl_rem!(u8);
impl_rem!(u16);
impl_rem!(u32);
impl_rem!(u64);
impl_rem!(u128);
impl_rem!(i8);
impl_rem!(i16);
impl_rem!(i32);
impl_rem!(i64);
impl_rem!(i128);

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
// ========================== Neg ============================================================
// ===========================================================================================

impl Neg for Number {
    type Output = Number;

    fn neg(self) -> Self::Output {
        match self {
            Number::Int(i) => Number::Int(-i),
            Number::Decimal(d) => Number::Decimal(-d),
        }
    }
}

impl Neg for &Number {
    type Output = Number;

    fn neg(self) -> Self::Output {
        match self {
            Number::Int(i) => Number::Int(-i),
            Number::Decimal(d) => Number::Decimal(-d),
        }
    }
}

// ===========================================================================================
// ========================== num_traits Impls ===============================================
// ===========================================================================================

impl Num for Number {
    type FromStrRadixErr = NumberError;

    /// The following radicies require a special prefix :
    ///
    /// 2  (binary)  => `0b`
    /// 6  (hex)     => '0x'
    /// 8  (octal)   => '0o'
    /// 64 (base64)  => 'b64'
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        match radix {
            2 => Number::from_binary_str(str),
            6 => Number::from_hexadecimal_str(str),
            8 => Number::from_octal_str(str),
            10 => str.parse::<Number>(),
            64 => Number::from_base64_str(str),
            _ => Err(NumberError::UnsupportedRadix(radix)),
        }
    }
}

impl Zero for Number {
    fn zero() -> Self {
        Self::ZERO
    }

    fn is_zero(&self) -> bool {
        self.is_zero()
    }
}

impl ConstZero for Number {
    const ZERO: Self = Self::ZERO;
}

impl One for Number {
    fn one() -> Self {
        Self::from(1)
    }
}

impl Signed for Number {
    fn abs(&self) -> Self {
        self.abs()
    }

    fn abs_sub(&self, other: &Self) -> Self {
        self.sub(other).abs()
    }

    fn signum(&self) -> Self {
        if self.is_zero() {
            Self::ZERO
        } else if self.is_positive() {
            Self::one()
        } else {
            Self::one().neg()
        }
    }

    fn is_positive(&self) -> bool {
        self.is_positive()
    }

    fn is_negative(&self) -> bool {
        self.is_negative()
    }
}

// ===========================================================================================
// ========================== Tests ==========================================================
// ===========================================================================================

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;
    use std::str::FromStr as _;

    #[rstest]
    #[case::add1("1", "1", "2")]
    #[case::add2("1.1", "2.2", "3.3")]
    #[case::add3("1.1", "2", "3.1")]
    #[case::add4("2", "1.1", "3.1")]
    fn add(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        let r = x + y;
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::add_assign1("1", "1", "2")]
    #[case::add_assign2("1.1", "2.2", "3.3")]
    #[case::add_assign3("1.1", "2", "3.1")]
    #[case::add_assign4("2", "1.1", "3.1")]
    fn add_assign(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let mut x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        x += y;
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[rstest]
    #[case::sub1("1", "1", "0")]
    #[case::sub2("1.1", "2.2", "-1.1")]
    #[case::sub3("2", "1.1", "0.9")]
    #[case::sub4("100", "47.4567", "52.5433")]
    #[case::sub5("5.5", "2.2", "3.3")]
    fn sub(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        let r = x - y;
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::sub_assign1("1", "1", "0")]
    #[case::sub_assign2("1.1", "2.2", "-1.1")]
    #[case::sub_assign3("2", "1.1", "0.9")]
    #[case::sub_assign4("100", "47.4567", "52.5433")]
    #[case::sub_assign5("5.5", "2.2", "3.3")]
    fn sub_assign(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let mut x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        x -= y;
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[rstest]
    #[case::mul1("1", "1", "1")]
    #[case::mul2("1.1", "2.2", "2.42")]
    #[case::mul3("2", "1.1", "2.2")]
    #[case::mul4("47.4567", "100", "4745.67")]
    #[case::mul5("55", "22", "1210")]
    #[case::mul6("5.7", "2", "11.4")]
    fn mul(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        let r = x * y;
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::mul_assign1("1", "1", "1")]
    #[case::mul_assign2("1.1", "2.2", "2.42")]
    #[case::mul_assign3("2", "1.1", "2.2")]
    #[case::mul_assign4("47.4567", "100", "4745.67")]
    #[case::mul_assign5("55", "22", "1210")]
    #[case::mul_assign6("5.7", "2", "11.4")]
    fn mul_assign(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let mut x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        x *= y;
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[rstest]
    #[case::div1("1", "1", "1")]
    #[case::div2("1.1", "2.2", "0.5")]
    #[case::div3("2", "1.1", "1.81818181818")]
    #[case::div4("100", "47", "2.12765957447")]
    #[case::div5("55", "5", "11")]
    #[case::div6("5.7", "2", "2.85")]
    fn div(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        let mut r = x / y;
        r.set_scale_round(11, bigdecimal::RoundingMode::HalfUp);
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::div_assign1("1", "1", "1")]
    #[case::div_assign2("1.1", "2.2", "0.5")]
    #[case::div_assign3("2", "1.1", "1.81818181818")]
    #[case::div_assign4("100", "47", "2.12765957447")]
    #[case::div_assign5("55", "5", "11")]
    #[case::div_assign6("5.7", "2", "2.85")]
    fn div_assign(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let mut x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        x /= y;
        x.set_scale_round(11, bigdecimal::RoundingMode::HalfUp);
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[rstest]
    #[case::rem1("1", "1", "0")]
    #[case::rem2("1.1", "2.2", "1.1")]
    #[case::rem3("2", "1.1", "0.9")]
    #[case::rem4("100", "47", "6")]
    #[case::rem5("55", "5", "0")]
    #[case::rem6("5.7", "2", "1.7")]
    #[case::rem7("5.6", "3.2", "2.4")]
    #[case::rem8("5.6", "2", "1.6")]
    fn rem(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        let r = x % y;
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::rem_assign1("1", "1", "0")]
    #[case::rem_assign2("1.1", "2.2", "1.1")]
    #[case::rem_assign3("2", "1.1", "0.9")]
    #[case::rem_assign4("100", "47", "6")]
    #[case::rem_assign5("55", "5", "0")]
    #[case::rem_assign6("5.7", "2", "1.7")]
    #[case::rem_assign7("5.6", "3.2", "2.4")]
    #[case::rem_assign8("5.6", "2", "1.6")]
    fn rem_assign(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let mut x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        x %= y;
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[rstest]
    #[case::neg1("55", "-55")]
    #[case::neg2("55.55", "-55.55")]
    fn neg(#[case] number: &str, #[case] expect: &str) {
        let n = Number::from_str(number).unwrap();
        let e = Number::from_str(expect).unwrap();
        let r = -n;
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    // //////////////////////////////////////////////////////////////
    // Macros + test for primitives
    // //////////////////////////////////////////////////////////////

    macro_rules! test_maths {
        ($name:ident, $op:ident, $t:ty, [$( [$lhs:expr, $rhs:expr, $expect:expr] ),* $(,)?]) => {
            #[rstest]
            $(#[case($lhs, $rhs as $t, $expect)])*
            fn $name(#[case] number: &str, #[case] rhs: $t, #[case] expect: &str) {
                let lhs = number.parse::<Number>().expect("Number");
                let e = expect.parse::<Number>().expect("Number");
                let r = lhs.$op(rhs);
                assert_eq!(e, r, "expected '{e:?}' got '{r:?}'");
            }
        };
    }

    macro_rules! generate_maths_tests {
        ($op:ident, {$($name:ident : $t:ty => $cases:tt),* $(,)?}) => {
            $(test_maths!($name, $op, $t, $cases);)*
        };
    }

    generate_maths_tests!(add, {
        add_u8  : u8  => [["1", 1, "2"], ["2", 2, "4"], ["2", u8::MAX, "257"]],
        add_i8  : i8  => [["5", -5, "0"], ["-5", -5, "-10"], ["2", i8::MAX, "129"]],
        add_u16 : u16 => [["1", 1, "2"], ["2", 2, "4"], ["2", u16::MAX, "65537"]],
        add_i16 : i16 => [["5", -5, "0"], ["-5", -5, "-10"], ["2", i16::MAX, "32769"]],
        add_u32 : u32 => [["1", 1, "2"], ["2", 2, "4"], ["2", u32::MAX, "4294967297"]],
        add_i32 : i32 => [["5", -5, "0"], ["-5", -5, "-10"], ["2", i32::MAX, "2147483649"]],
        add_u64 : u64 => [["1", 1, "2"], ["2", 2, "4"], ["2", u64::MAX, "18446744073709551617"]],
        add_i64 : i64 => [["5", -5, "0"], ["-5", -5, "-10"], ["2", i64::MAX, "9223372036854775809"]],
        add_u128 : u128 => [["1", 1, "2"], ["2", 2, "4"], ["2", u128::MAX, "340282366920938463463374607431768211457"]],
        add_i128 : i128 => [["5", -5, "0"], ["-5", -5, "-10"], ["2", i128::MAX, "170141183460469231731687303715884105729"]],
    });

    generate_maths_tests!(sub, {
        sub_u8  : u8  => [["1", 1, "0"], ["2", 2, "0"], ["-2", u8::MIN, "-2"]],
        sub_i8  : i8  => [["5", -5, "10"], ["-5", -5, "0"], ["-2", i8::MAX, "-129"]],
        sub_u16 : u16 => [["1", 1, "0"], ["2", 2, "0"], ["-2", u16::MAX, "-65537"]],
        sub_i16 : i16 => [["5", -5, "10"], ["-5", -5, "0"], ["-2", i16::MAX, "-32769"]],
        sub_u32 : u32 => [["1", 1, "0"], ["2", 2, "0"], ["-2", u32::MAX, "-4294967297"]],
        sub_i32 : i32 => [["5", -5, "10"], ["-5", -5, "0"], ["-2", i32::MAX, "-2147483649"]],
        sub_u64 : u64 => [["1", 1, "0"], ["2", 2, "0"], ["-2", u64::MAX, "-18446744073709551617"]],
        sub_i64 : i64 => [["5", -5, "10"], ["-5", -5, "0"], ["-2", i64::MAX, "-9223372036854775809"]],
        sub_u128 : u128 => [["1", 1, "0"], ["2", 2, "0"], ["-2", u128::MAX, "-340282366920938463463374607431768211457"]],
        sub_i128 : i128 => [["5", -5, "10"], ["-5", -5, "0"], ["-2", i128::MAX, "-170141183460469231731687303715884105729"]],
    });

    generate_maths_tests!(mul, {
        mul_u8  : u8  => [["12", 12, "144"], ["2", 2, "4"], [&u8::MAX.to_string(), u8::MAX, "65025"]],
        mul_i8  : i8  => [["5", -5, "-25"], ["-5", -5, "25"], [&i8::MAX.to_string(), i8::MAX, "16129"]],
        mul_u16 : u16 => [["1", 1, "1"], ["2", 2, "4"], [&u16::MAX.to_string(), u16::MAX, "4294836225"]],
        mul_i16 : i16 => [["10", -5, "-50"], ["-3454239", -5, "17271195"], [&i16::MAX.to_string(), i16::MAX, "1073676289"]],
        mul_u32 : u32 => [["1", 1, "1"], ["10", 20, "200"], [&u32::MAX.to_string(), u32::MAX, "18446744065119617025"]],
        mul_i32 : i32 => [["5", -5, "-25"], ["-75", -30, "2250"], [&i32::MAX.to_string(), i32::MAX, "4611686014132420609"]],
        mul_u64 : u64 => [["1", 1, "1"], ["2", 2, "4"], [&u64::MAX.to_string(), u64::MAX, "340282366920938463426481119284349108225"]],
        mul_i64 : i64 => [["5", -5, "-25"], ["-5", -5, "25"], [&i64::MAX.to_string(), i64::MAX, "85070591730234615847396907784232501249"]],
        mul_u128 : u128 => [["1", 1, "1"], ["2", 2, "4"], [&u128::MAX.to_string(), u128::MAX, "115792089237316195423570985008687907852589419931798687112530834793049593217025"]],
        mul_i128 : i128 => [["5", -5, "-25"], ["-5", -5, "25"], [&i128::MAX.to_string(), i128::MAX, "28948022309329048855892746252171976962977213799489202546401021394546514198529"]],
    });

    generate_maths_tests!(div, {
        div_u8  : u8  => [["-12", 6, "-2"], ["1", 2, "0.5"], ["0", 1, "0"]],
        div_i8  : i8  => [["-12", 6, "-2"], ["1", 2, "0.5"], [&i8::MIN.to_string(), -1, "128"]],
        div_u16 : u16 => [["-12", 6, "-2"], ["1", 2, "0.5"], ["0", 1, "0"]],
        div_i16 : i16 => [["-12", 6, "-2"], ["1", 2, "0.5"], [&i16::MIN.to_string(), -1, "32768"]],
        div_u32 : u32 => [["-12", 6, "-2"], ["1", 2, "0.5"], ["0", 1, "0"]],
        div_i32 : i32 => [["-12", 6, "-2"], ["1", 2, "0.5"], [&i32::MIN.to_string(), -1, "2147483648"]],
        div_u64 : u64 => [["-12", 6, "-2"], ["1", 2, "0.5"], ["0", 1, "0"]],
        div_i64 : i64 => [["-12", 6, "-2"], ["1", 2, "0.5"], [&i64::MIN.to_string(), -1, "9223372036854775808"]],
        div_u128 : u128 => [["-12", 6, "-2"], ["1", 2, "0.5"], ["0", 1, "0"]],
        div_i128 : i128 => [["-12", 6, "-2"], ["1", 2, "0.5"], [&i128::MIN.to_string(), -1, "170141183460469231731687303715884105728"]],
    });

    generate_maths_tests!(rem, {
        rem_u8  : u8  => [["255.12", 3, "0.12"]],
        rem_i8  : i8  => [["255.12", 3, "0.12"]],
        rem_u16 : u16 => [["255.12", 3, "0.12"]],
        rem_i16 : i16 => [["255.12", 3, "0.12"]],
        rem_u32 : u32 => [["255.12", 3, "0.12"]],
        rem_i32 : i32 => [["255.12", 3, "0.12"]],
        rem_u64 : u64 => [["255.12", 3, "0.12"]],
        rem_i64 : i64 => [["255.12", 3, "0.12"]],
        rem_u128 : u128 => [["255.12", 3, "0.12"]],
        rem_i128 : i128 => [["255.12", 3, "0.12"]],
    });
}
