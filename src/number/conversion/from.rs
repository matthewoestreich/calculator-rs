use crate::{
    Number, NumberError,
    number::{
        conversion::{self, ByteOrder, number_from_bytes},
        digit::HexDigit,
        predicate,
    },
};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{FromBytes, FromPrimitive, Num as _};
use std::str::FromStr;

impl Number {
    /// Converts an `f64` info `Number`.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let number = Number::from_f64(12.12);
    /// let expect = "12.12".parse::<Number>();
    /// assert_eq!(number, expect);
    /// ```
    pub fn from_f64(n: f64) -> Result<Self, NumberError> {
        Self::try_from(n)
    }

    /// Converts an `f64` into `Number` without guardrails.
    ///
    /// <div class="warning">
    ///
    /// > # panics!
    /// > as an unchecked method, it will panic if something goes wrong during the conversion
    ///
    /// </div>
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let number = Number::from_f64_unchecked(12.12);
    /// let expect = "12.12".parse::<Number>().expect("Number::Decimal");
    /// assert_eq!(number, expect);
    /// ```
    pub fn from_f64_unchecked(n: f64) -> Self {
        let bd = n
            .to_string()
            .parse::<BigDecimal>()
            .expect("this method is unchecked");
        Self::Decimal(bd)
    }

    /// Performs hexadecimal validation to ensure we were given a hexadecimal string.
    /// Converts said hexadecimal string into `Number`.
    ///
    /// <div class="warning">
    ///
    /// # A valid hexadecimal string
    ///
    /// ```text
    ///   -0xFFA.FFA
    ///   | | | |
    ///   | | | +-- A single decimal anywhere after `0x` (or `-0x`) prefix
    ///   | | +-- Any amount of valid hexadecimal characters (see below)
    ///   | +-- `0x` (or `-0x` for negative numbers) is required as prefix
    ///   +---- A single negative sign; only allowed as first char
    /// ```
    ///
    /// </div>
    ///
    /// # Valid hexadecimal characters
    /// - Must start with `0x` or `-0x` for negative numbers
    /// - Any combination of:
    ///   - digits `0`-`9`
    ///   - characters (case **in**sensitive) `A`, `B`, `C`, `D`, `E`, `F`
    ///
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let a = "-0x63A.2675".parse::<Number>().expect("Number::Decimal");
    /// assert_eq!(a, "-1594.9845".parse::<Number>().expect("eq"));
    /// ```
    ///
    pub fn from_hexadecimal_str(hex_str: &str) -> Result<Number, NumberError> {
        if !predicate::is_hexadecimal_str(hex_str) {
            return Err(NumberError::Parsing {
                value: format!("'{hex_str}' is not a hexadecimal string"),
            });
        }

        let (is_signed, s) = match hex_str.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, hex_str),
        };

        let s = s.strip_prefix("0x").unwrap_or(hex_str);
        let (int_part, fract_part) = s.split_once('.').unwrap_or((s, ""));
        let int_part_len = int_part.len();
        let fract_part_len = fract_part.len();
        let base = Number::from(16);

        let mut int = int_part.chars().enumerate().try_fold(
            Number::ZERO,
            |acc, (i, c)| -> Result<_, NumberError> {
                let exponent = int_part_len as u32 - 1 - i as u32;
                let multiplier = base.pow(exponent as i64)?;
                let hexchar = HexDigit::try_from(&c)?;
                let digit = Number::from(hexchar);
                Ok(acc + digit * multiplier)
            },
        )?;

        let maybe_fract = if fract_part_len == 0 {
            None
        } else {
            fract_part
                .chars()
                .enumerate()
                .try_fold(Number::ZERO, |acc, (i, c)| -> Option<Number> {
                    let exponent = fract_part_len as u32 - 1 - i as u32;
                    let multiplier = base.pow(exponent as i64).ok()?;
                    let hexchar = HexDigit::try_from(&c).ok()?;
                    let digit = Number::from(hexchar);
                    Some(acc + digit * multiplier)
                })
        };

        if let Some(fract) = maybe_fract {
            // shift fract into decimal position, e.g., `int + fract / 10.pow(fract_digit_count)`
            let scale = Number::from(10).pow(fract.digit_count() as i64)?;
            int += fract / scale;
        }

        Ok(if is_signed { -int } else { int })
    }

    /// Base64 strings must be prefixed with `b64`!
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let a = Number::from_base64_str("b64OTk5OTk=").expect("Number::Int");
    /// assert_eq!(a, Number::from(99999));
    /// ```
    pub fn from_base64_str(s: &str) -> Result<Number, NumberError> {
        if s == "b64" || !s.starts_with("b64") {
            return Err(NumberError::InvalidArgument);
        }

        let s = s.strip_prefix("b64").unwrap_or(s);
        let d = Self::base64_decode(s);
        d.parse::<Number>()
    }

    /// Performs binary string validation to ensure we were given a binary string,
    /// then converts the binary string into `Number`.
    pub(crate) fn from_binary_str(s: &str) -> Result<Self, NumberError> {
        let s = s.trim();
        // We were given "" or just the prefix to a binary string "0b" or "-0b"
        if s.is_empty() || s == "-0b" || s == "0b" || s == "-" {
            return Err(NumberError::Parsing {
                value: format!("'{s}' either contains no binary or it is empty"),
            });
        }
        if !predicate::is_binary_str(s) {
            return Err(NumberError::Parsing {
                value: format!("'{s}' is not a binary string"),
            });
        }

        let (is_negative, s) = match s.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, s),
        };

        let s = s.strip_prefix("0b").unwrap_or(s);

        let number = if !s.contains('.') {
            Number::Int(BigInt::from_str_radix(s, 2)?)
        } else {
            let (lhs, rhs) = s.split_once('.').unwrap_or((s, ""));
            let mut dec_str = conversion::binary_str_to_decimal_str(lhs);
            if !rhs.is_empty() {
                let rhs_bin = format!(".{}", conversion::binary_str_to_decimal_str(rhs));
                dec_str.push_str(&rhs_bin);
            }
            Number::Decimal(BigDecimal::from_str_radix(&dec_str, 10)?)
        };

        Ok(if is_negative { -number } else { number })
    }
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

impl From<HexDigit> for Number {
    fn from(n: HexDigit) -> Self {
        Number::Int((n as u8).into())
    }
}

impl From<&HexDigit> for Number {
    fn from(n: &HexDigit) -> Self {
        Number::Int((*n as u8).into())
    }
}

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

impl TryFrom<f64> for Number {
    type Error = NumberError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        let bd = value.to_string().parse::<BigDecimal>()?;
        Ok(Number::Decimal(bd))
    }
}

impl FromStr for Number {
    type Err = NumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = Number::from_binary_str(s) {
            return Ok(n);
        }
        if let Ok(n) = Number::from_hexadecimal_str(s) {
            return Ok(n);
        }
        if let Ok(n) = Number::from_base64_str(s) {
            return Ok(n);
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

impl FromPrimitive for Number {
    fn from_i64(n: i64) -> Option<Self> {
        Some(Number::from(n))
    }

    fn from_u64(n: u64) -> Option<Self> {
        Some(Number::from(n))
    }
}

impl FromBytes for Number {
    type Bytes = Vec<u8>;

    /// If something goes wrong during converion we return Number::ZERO
    fn from_be_bytes(bytes: &Self::Bytes) -> Self {
        number_from_bytes(bytes.as_slice(), ByteOrder::BigEndian)
    }

    /// If something goes wrong during converion we return Number::ZERO
    fn from_le_bytes(bytes: &Self::Bytes) -> Self {
        number_from_bytes(bytes.as_slice(), ByteOrder::LittleEndian)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::NumberOrder;
    use rstest::*;

    #[rstest]
    #[case::num_from_be_bytes1(Vec::from([1, 0, 0, 0, 3, 1, 224, 243, 0, 0, 0, 0, 0, 0, 0, 3]), "123.123", NumberOrder::Decimal, false)]
    #[case::num_from_be_bytes2(Vec::from([0, 0, 0, 0, 3, 1, 134, 159]), "99999", NumberOrder::Int, false)]
    fn number_from_be_bytes(
        #[case] bytes: Vec<u8>,
        #[case] expected_num: &str,
        #[case] expect_order: NumberOrder,
        #[case] expect_zero: bool,
    ) {
        let e_num = expected_num.parse::<Number>().expect("Number");
        let r = Number::from_be_bytes(&bytes);
        assert_eq!(
            r.order(),
            expect_order,
            "expected order '{expect_order:?}' got order '{:?}'",
            r.order()
        );
        assert!(if expect_zero {
            r.is_zero()
        } else {
            !r.is_zero()
        });
        assert_eq!(e_num, r, "expected '{e_num}' got '{r}'");
    }

    #[rstest]
    #[case::num_from_le_bytes1(Vec::from([1, 3, 0, 0, 0, 1, 224, 243, 3, 0, 0, 0, 0, 0, 0, 0]), "123.123", NumberOrder::Decimal, false)]
    #[case::num_from_le_bytes2(Vec::from([0, 3, 0, 0, 0, 1, 134, 159]), "99999", NumberOrder::Int, false)]
    fn number_from_le_bytes(
        #[case] bytes: Vec<u8>,
        #[case] expected_num: &str,
        #[case] expect_order: NumberOrder,
        #[case] expect_zero: bool,
    ) {
        let e_num = expected_num.parse::<Number>().expect("Number");
        let r = Number::from_le_bytes(&bytes);
        assert_eq!(
            r.order(),
            expect_order,
            "expected order '{expect_order:?}' got order '{:?}'",
            r.order()
        );
        assert!(if expect_zero {
            r.is_zero()
        } else {
            !r.is_zero()
        });
        assert_eq!(e_num, r, "expected '{e_num}' got '{r}'");
    }
}
