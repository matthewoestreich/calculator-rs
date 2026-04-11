use crate::{Number, NumberError};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive};
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

    /// If variant is `Number::Decimal` we return the integer part is binary
    /// and the fractional part as binary, separated by a period.
    /// For example, if you have a `Number::Decimal(100.773)` this method
    /// returns : `"1100100.1100000101"`
    pub fn to_binary_str(&self) -> String {
        match self {
            Number::Int(big_int) => format!("{big_int:b}").to_string(),
            Number::Decimal(big_decimal) => {
                let s = big_decimal.to_string();
                let parts: Vec<_> = s.split('.').collect();
                let mut output = Self::to_bin_str(parts[0]);
                if parts[1].is_empty() {
                    output
                } else {
                    output.push('.');
                    output.push_str(&Self::to_bin_str(parts[1]));
                    output
                }
            }
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

    fn to_bin_str(decimal_str: &str) -> String {
        if decimal_str == "0" || decimal_str.is_empty() {
            return "0".to_string();
        }
        let is_negative = decimal_str.starts_with('-');
        let decimal_str = decimal_str.trim_start_matches('-');
        let mut digits = Vec::with_capacity(decimal_str.len());
        for c in decimal_str.chars() {
            if let Some(d) = c.to_digit(10) {
                digits.push(d as u8);
            } else {
                return format!("<INVALID_DIGIT_FOUND = '{c}'>");
            }
        }
        let mut binary_bits = String::new();
        while !digits.is_empty() {
            let mut remainder = 0;
            let mut next_digits = Vec::with_capacity(digits.len());
            // Long division by 2
            for &digit in &digits {
                let current = digit + remainder * 10;
                let quotient = current / 2;
                remainder = current % 2;
                // Only push if it's not a leading zero
                if !next_digits.is_empty() || quotient > 0 {
                    next_digits.push(quotient);
                }
            }
            // The remainder of the full division is our binary digit
            binary_bits.push(if remainder == 0 { '0' } else { '1' });
            digits = next_digits;
        }
        if is_negative {
            binary_bits.push('-');
        }
        // Reverse to get the correct order (MSB first)
        binary_bits.chars().rev().collect()
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

// ===========================================================================================
// ========================== FromStr ========================================================
// ===========================================================================================

impl FromStr for Number {
    type Err = NumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<BigInt>().map(Self::Int).or_else(|_| {
            s.parse::<BigDecimal>()
                .map(Self::Decimal)
                .map_err(|_| NumberError::Parsing {
                    value: s.to_string(),
                })
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
    fn from_str() {
        let a = Number::from_str("1.1").unwrap();
        let ea = 1.1.to_number();
        assert_eq!(a, ea, "expected {ea:?} got {a:?}");

        let b = Number::from_str("1").unwrap();
        let eb = 1.to_number();
        assert_eq!(b, eb, "expected {eb:?} got {b:?}");
    }

    #[test]
    fn from_f64() {
        let a = Number::from_f64(1.1).unwrap();
        assert_eq!(a.order(), NumberOrder::Decimal);
    }

    #[rstest]
    #[case::binary_str1(
        "17958432089245743489.3597843208120587934",
        "1111100100111001001010101101011001011010011101111111100110000001.11000111101110000110110101010111101001100101000101011010011110"
    )]
    #[case::binary_str_bigdecimal_neg(
        "-17958432089245743489.3597843208120587934",
        "-1111100100111001001010101101011001011010011101111111100110000001.11000111101110000110110101010111101001100101000101011010011110"
    )]
    #[case::binary_str2(
        "17958432089245743489",
        "1111100100111001001010101101011001011010011101111111100110000001"
    )]
    #[case::binary_str_bigint_neg(
        "-17958432089245743489",
        "-1111100100111001001010101101011001011010011101111111100110000001"
    )]
    fn binary_str(#[case] number: &str, #[case] expect: &str) {
        let n = Number::from_str(number).unwrap();
        let fr = format!("{n:b}");
        assert_eq!(
            expect, fr,
            "[format!(\"{n:b}\")] expected '{expect}' got '{fr}'"
        );
        let br = n.to_binary_str();
        assert_eq!(
            expect, br,
            "[n.to_binary_str()] expected '{expect}' got '{br}'"
        );
    }
}
