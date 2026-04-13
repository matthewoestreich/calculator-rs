use crate::{Number, NumberError, number::ASTRO_CONSTS};
use astro_float::{BigFloat, Radix as AstroRadix, RoundingMode as AstroRoundingMode};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{Num, Signed, ToPrimitive};
use std::str::FromStr;

impl Number {
    pub fn from_f64(n: f64) -> Result<Self, NumberError> {
        Self::try_from(n)
    }

    pub fn to_i64(&self) -> Option<i64> {
        match self {
            Number::Int(i) => i.to_i64(),
            Number::Decimal(d) => d.to_i64(),
        }
    }

    /// If `self` is `Number::Decimal` calling this method may result in data loss!
    /// This is due to how decimal to integer conversion works.
    /// IMPORTANT: if your number does not fit into an `i64`, it will be saturated,
    /// eg. clamped to `i64` bounds, which may result in data loss!
    pub fn to_i64_saturating(&self) -> i64 {
        match self {
            Number::Int(i) => Self::saturating_i64(i),
            Number::Decimal(d) => Self::saturating_i64(d),
        }
    }

    pub fn to_i32(&self) -> Option<i32> {
        match self {
            Number::Int(i) => i.to_i32(),
            Number::Decimal(d) => d.to_i32(),
        }
    }

    pub fn to_i128(&self) -> Option<i128> {
        match self {
            Number::Int(i) => i.to_i128(),
            Number::Decimal(d) => d.to_i128(),
        }
    }

    /// If `self` is `Number::Decimal` calling this method may result in data loss!
    /// This is due to how decimal to integer conversion works.
    /// IMPORTANT: if your number does not fit into an `i128`, it will be saturated,
    /// eg. clamped to `i128` bounds, which may result in data loss!
    pub fn to_i128_saturating(&self) -> i128 {
        match self {
            Number::Int(i) => Self::saturating_i128(i),
            Number::Decimal(d) => Self::saturating_i128(d),
        }
    }

    /// If the underlying value for `T` does not fit within an
    /// `i128`, we truncate it to fit within `i128` bounds, which
    /// may result in data/precision/scale loss!
    fn saturating_i128<T>(x: &T) -> i128
    where
        T: ToPrimitive + Signed,
    {
        x.to_i128().unwrap_or_else(|| {
            if x.signum().is_negative() {
                i128::MIN
            } else {
                i128::MAX
            }
        })
    }

    /// If the underlying value for `T` does not fit within an
    /// `i64`, we truncate it to fit within `i64` bounds, which
    /// may result in data/precision/scale loss!
    fn saturating_i64<T>(x: &T) -> i64
    where
        T: ToPrimitive + Signed,
    {
        x.to_i64().unwrap_or_else(|| {
            if x.signum().is_negative() {
                i64::MIN
            } else {
                i64::MAX
            }
        })
    }
}

// ===========================================================================================
// ========================== ToNumber =======================================================
// ===========================================================================================

#[allow(dead_code)]
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

impl TryFrom<BigFloat> for Number {
    type Error = NumberError;

    fn try_from(value: BigFloat) -> Result<Self, Self::Error> {
        let bfstr = value.to_string();
        let bd = bfstr.parse::<BigDecimal>()?;
        Ok(Self::Decimal(bd))
    }
}

// ===========================================================================================
// ========================== FromStr ========================================================
// ===========================================================================================

fn bigfloat_from_bin_str(s: &str) -> BigFloat {
    ASTRO_CONSTS.with(|cc| {
        BigFloat::parse(
            s,
            AstroRadix::Bin,
            usize::MAX,
            AstroRoundingMode::None,
            &mut cc.borrow_mut(),
        )
    })
}

impl FromStr for Number {
    type Err = NumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Number::is_binary(s) {
            if let Ok(i) = BigInt::from_str_radix(s, 2) {
                return Ok(Number::Int(i));
            }
            if let Ok(d) = Number::try_from(bigfloat_from_bin_str(s)) {
                return Ok(d);
            }
        }
        if let Ok(i) = s.parse::<BigInt>() {
            return Ok(Number::Int(i));
        }
        if let Ok(d) = s.parse::<BigDecimal>() {
            return Ok(Number::Decimal(d));
        }
        Err(NumberError::Parsing {
            value: s.to_string(),
        })
    }
}

// ===========================================================================================
// ========================== Tests ==========================================================
// ===========================================================================================

#[cfg(test)]
mod test {
    use crate::{number::conversion::ToNumber, *};
    use rstest::*;
    use std::str::FromStr as _;

    #[test]
    fn from_str_1() {
        let a = Number::from_str("1.1").unwrap();
        let ea = 1.1.to_number();
        assert_eq!(a, ea, "expected {ea:?} got {a:?}");

        let b = Number::from_str("1").unwrap();
        let eb = 1.to_number();
        assert_eq!(b, eb, "expected {eb:?} got {b:?}");
    }

    #[rstest]
    #[case::from_str1("1.1", "1.1", NumberOrder::Decimal)]
    #[case::from_str2("1", "1", NumberOrder::Int)]
    #[case::binary_plain_int(
        "11110011001110000110000001100011100011110",
        "2089245787934",
        NumberOrder::Int
    )]
    #[case::binary_scientific_dec(
        "1.11110010011100100101010110101100101101001110111111110011000000101011100000110101101001101000011100000000101100101101111100010000101001010011000100011010111010010011101011101101010001001001e+111111",
        "17958432089245743489.3597843208120587934",
        NumberOrder::Decimal
    )]
    /// FOR FROMSTR ON BINARY STRINGS, NEED TO FORCE PPLL TO USE A 0b PREFIX OR ELSE
    /// THERE IS NO WAY TO DISTINGUISH `1010` (integer, as in the number 1,010) FROM `1010` (binary, as in the number 10)
    #[case::binary_scientific_int("1.10e+3", "1010", NumberOrder::Int)]
    fn from_str(#[case] s: &str, #[case] expect_str: &str, #[case] expect_order: NumberOrder) {
        let expect_str = expect_str.to_string();
        let x = s.parse::<Number>().unwrap();
        assert_eq!(
            x.order(),
            expect_order,
            "expected order '{expect_order:?}' got order '{:?}'",
            x.order()
        );
        assert_eq!(
            x.to_string(),
            expect_str,
            "expected string '{expect_str}' got string '{}'",
            x
        );
    }

    #[test]
    fn from_f64() {
        let a = Number::from_f64(1.1).unwrap();
        assert_eq!(a.order(), NumberOrder::Decimal);
    }
}
