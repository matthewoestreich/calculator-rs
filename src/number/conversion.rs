use crate::{Number, NumberError, number::hexchar::HexChar};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{Num, Signed, ToPrimitive};
use std::str::FromStr;

impl Number {
    /// Converts the value of `self` to an `i64`. If the value cannot be represented by
    /// an `i64`, then `None` is returned.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let a = Number::from(10);
    /// assert_eq!(a.to_i64(), Some(10i64));
    ///
    /// let b = Number::from(i128::MAX);
    /// assert_eq!(b.to_i64(), None);
    /// ```
    pub fn to_i64(&self) -> Option<i64> {
        match self {
            Number::Int(i) => i.to_i64(),
            Number::Decimal(d) => d.to_i64(),
        }
    }

    /// Converts the value of `self` to an `i64`. If the value cannot be represented by
    /// an `i64`, then the value is truncated to fit within `i64` bounds, or saturated..
    ///
    /// <div class="warning">Lossy!</div>
    ///
    /// If `self` is variant `Number::Decimal(_)`, calling this method may cause data loss!
    /// Naturally, converting from a decimal (with a fractional part) to an integer is
    /// lossy.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let a = Number::from(i128::MAX);
    /// assert_eq!(a.to_i64_saturating(), i64::MAX);
    ///
    /// let b = Number::from(i128::MIN);
    /// assert_eq!(b.to_i64_saturating(), i64::MIN);
    ///
    /// let c = "123.123".parse::<Number>().expect("Number::Decimal");
    /// assert!(c.is_decimal());
    /// assert_eq!(c.to_i64_saturating(), 123i64);
    /// ````
    pub fn to_i64_saturating(&self) -> i64 {
        match self {
            Number::Int(i) => Self::saturating_i64(i),
            Number::Decimal(d) => Self::saturating_i64(d),
        }
    }

    /// Converts the value of `self` to an `i32`. If the value cannot be
    /// represented by an `i32`, then `None` is returned.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let a = Number::from(10);
    /// assert_eq!(a.to_i32(), Some(10i32));
    ///
    /// let b = Number::from(i64::MAX);
    /// assert_eq!(b.to_i32(), None);
    /// ```
    pub fn to_i32(&self) -> Option<i32> {
        match self {
            Number::Int(i) => i.to_i32(),
            Number::Decimal(d) => d.to_i32(),
        }
    }

    /// Converts the value of `self` to an `i128`. If the value cannot be
    /// represented by an `i128`, then `None` is returned.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let a = Number::from(10);
    /// assert_eq!(a.to_i128(), Some(10i128));
    ///
    /// let b = Number::from(u128::MAX);
    /// assert_eq!(b.to_i128(), None);
    /// ```
    pub fn to_i128(&self) -> Option<i128> {
        match self {
            Number::Int(i) => i.to_i128(),
            Number::Decimal(d) => d.to_i128(),
        }
    }

    /// Converts the value of `self` to an `i128`. If the value cannot be represented by
    /// an `i128`, then the value is truncated to fit within `i128` bounds, or saturated..
    ///
    /// <div class="warning">Lossy!</div>
    ///
    /// If `self` is variant `Number::Decimal(_)`, calling this method may cause data loss!
    /// Naturally, converting from a decimal (with a fractional part) to an integer is
    /// lossy.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let a = Number::from(i128::MAX);
    /// assert_eq!(a.to_i128_saturating(), i128::MAX);
    ///
    /// let b = Number::from(i128::MIN);
    /// assert_eq!(b.to_i128_saturating(), i128::MIN);
    ///
    /// // This number won't fit into `i128` or `u128`.
    /// let big = "999999999999999999999999999999999999999999999999999999999";
    /// let big_num = big.parse::<Number>().expect("Number::Int");
    /// assert_eq!(big_num.to_i128_saturating(), i128::MAX);
    ///
    /// // `u128::MAX` as decimal with fractional part.
    /// let d = "340282366920938463463374607431768211455.123456789"
    ///     .parse::<Number>()
    ///     .expect("Number::Decimal");
    /// assert!(d.is_decimal());
    /// assert_eq!(d.to_i128_saturating(), i128::MAX);
    /// ````
    pub fn to_i128_saturating(&self) -> i128 {
        match self {
            Number::Int(i) => Self::saturating_i128(i),
            Number::Decimal(d) => Self::saturating_i128(d),
        }
    }

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
    ///     Panics! if something goes wrong while converting <code>n</code> into <code>BigDecimal</code>.
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
    /// [!IMPORTANT]
    /// - We expect a hexadecimal string to start with `"0x"`.
    /// - An empty input string will return an `Err`.
    /// - A hexadecimal string can contain (in any order):
    ///   - Digits `0` - `9`.
    ///   - Characters (case insensitive) `A`, `B`, `C`, `D`, `E`, `F`.
    /// - Non hexadecimal characters
    ///   - `'-'` : a single negative sign; required to be at the start of the string, after the `"0x"` prefix
    ///   - `'.'` : a single decimal; allowed anywhere to the right of the negative sign.
    pub fn from_hexadecimal_str(hex_str: &str) -> Result<Number, NumberError> {
        if !Self::is_hexadecimal_str(hex_str) {
            return Err(NumberError::Parsing {
                value: format!("'{hex_str}' is not a hexadecimal string"),
            });
        }

        let s = hex_str.strip_prefix("0x").unwrap_or(hex_str);
        let (is_signed, s) = match s.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, s),
        };

        let (int_part, fract_part) = s.split_once('.').unwrap_or((s, ""));
        let int_part_len = int_part.len();
        let fract_part_len = fract_part.len();
        let base = Number::from(16);

        let mut int = int_part.chars().enumerate().try_fold(
            Number::ZERO,
            |acc, (i, c)| -> Result<_, NumberError> {
                let exponent = int_part_len as u32 - 1 - i as u32;
                let multiplier = base.pow(exponent as i64)?;
                let hexchar = HexChar::try_from(&c)?;
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
                    let multiplyer = base.pow(exponent as i64).ok()?;
                    let hexchar = HexChar::try_from(&c).ok()?;
                    let digit = Number::from(hexchar);
                    Some(acc + digit * multiplyer)
                })
        };

        if let Some(fract) = maybe_fract {
            // shift fract into decimal position, e.g., `int + fract / 10.pow(fract_digit_count)`
            let scale = Number::from(10).pow(fract.digit_count() as i64)?;
            int += fract / scale;
        }

        Ok(if is_signed { -int } else { int })
    }

    /// Returns `None` if `decimal_str` is not considered to be a valid decimal string.
    /// **Empty strings are allowed, we simply return `Some(String::from("0"))`.**
    /// A valid decimal string meets the following requirements:
    /// - May contain a negative sign, e.g., `-` at the start of the string.
    /// - May contain a single decimal, e.g., '.'.
    /// - Outside of '-' or '.', can only contain digits '0'-'9'.
    pub(crate) fn decimal_str_to_binary_str(decimal_str: &str) -> Option<String> {
        if decimal_str == "0" || decimal_str.is_empty() {
            return Some("0".to_string());
        }
        if !Self::is_decimal_str(decimal_str) {
            // Since `is_decimal_str` will return `false` for empty strings, but we want to
            // allow empty strings, we only return `None` if the string is not actually empty.
            if !decimal_str.is_empty() {
                return None;
            }
        }
        let is_negative = decimal_str.starts_with('-');
        let decimal_str = decimal_str.trim_start_matches('-');
        let mut digits = Vec::with_capacity(decimal_str.len());
        for c in decimal_str.chars() {
            if let Some(d) = c.to_digit(10) {
                digits.push(d as u8);
            } else {
                return None;
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
        Some(binary_bits.chars().rev().collect())
    }

    /// Returns `None` if `decimal_str` is not considered to be a valid decimal string.
    /// **Empty strings are allowed, we simply return `Some(String::from("0"))`.**
    /// A valid decimal string meets the following requirements:
    /// - May contain a negative sign, e.g., `-` at the start of the string.
    /// - May contain a single decimal, e.g., '.'.
    /// - Outside of '-' or '.', can only contain digits '0'-'9'.
    pub(crate) fn decimal_str_to_hexadecimal_str(
        decimal_str: &str,
        uppercase: bool,
    ) -> Option<String> {
        if decimal_str == "0" || decimal_str.is_empty() {
            return Some("0".to_string());
        }

        if !Self::is_decimal_str(decimal_str) {
            return None;
        }

        let (sign, dec_str) = match decimal_str.strip_prefix('-') {
            Some(rest) => ("-", rest),
            None => ("", decimal_str),
        };

        let mut dividend = dec_str.parse::<Number>().ok()?;
        let mut hex_str = String::new();

        loop {
            // Divisor is 16 - remainder will always fit in a HexChar
            let (quotient, remainder) = dividend.div_mod(16);

            hex_str.push(
                HexChar::from_str(&remainder.to_string())
                    .ok()?
                    .to_char(uppercase),
            );

            if quotient.is_zero() {
                break;
            }

            dividend = quotient;
        }

        hex_str.push_str(sign);
        Some(hex_str.chars().rev().collect())
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

    /// Performs binary string validation to ensure we were given a binary string,
    /// then converts the binary string into `Number`.
    fn from_binary_str(s: &str) -> Result<Self, NumberError> {
        let s = s.trim();

        // We were give "" or just the prefix to a binary string "0b"
        if s.is_empty() || s == "0b" {
            return Err(NumberError::Parsing {
                value: "'' binary str cannot be empty".to_string(),
            });
        }
        if !Self::is_binary_str(s) {
            return Err(NumberError::Parsing {
                value: format!("'{s}' is not a binary string, binary strings start with '0b'"),
            });
        }

        let s = s.strip_prefix("0b").unwrap_or(s);

        // binary string has no decimal, parse binary string as Int variant.
        Ok(if !s.contains('.') {
            let bi = BigInt::from_str_radix(s, 2)?;
            Number::Int(bi)
        } else {
            // binary string has a decimal, parse binary string as Decimal variant.
            let is_negative = s.starts_with('-');
            let (lhs, rhs) = s.split_once('.').unwrap_or((s, ""));
            let mut dec_str = Self::binary_str_to_decimal_str(lhs);
            if !rhs.is_empty() {
                dec_str.push('.');
                dec_str.push_str(&Self::binary_str_to_decimal_str(rhs));
            }
            if is_negative {
                dec_str = format!("-{dec_str}");
            }
            Number::Decimal(BigDecimal::from_str_radix(&dec_str, 10)?)
        })
    }

    /// Assumes you have already validated that what you are passing in is ACTUALLY a binary string!
    fn binary_str_to_decimal_str(bin: &str) -> String {
        let base_u64: u64 = 1_000_000_000;
        let base_u32: u32 = base_u64 as u32;
        let mut digits: Vec<u32> = vec![0]; // little-endian (least significant first)

        for c in bin.chars() {
            let mut carry: u64 = 0;
            for d in digits.iter_mut() {
                let val = (*d as u64) * 2 + carry;
                *d = (val % base_u64) as u32;
                carry = val / base_u64;
            }
            if carry > 0 {
                digits.push(carry as u32);
            }

            if c == '1' {
                let mut carry = 1;
                for d in digits.iter_mut() {
                    let val = *d + carry;
                    *d = val % base_u32;
                    carry = val / base_u32;

                    if carry == 0 {
                        break;
                    }
                }
                if carry > 0 {
                    digits.push(carry);
                }
            }
        }

        let mut s = String::new();
        for (i, &d) in digits.iter().rev().enumerate() {
            if i == 0 {
                s.push_str(&d.to_string());
            } else {
                s.push_str(&format!("{:09}", d)); // zero-pad
            }
        }
        s
    }
}

// ===========================================================================================
// ========================== ToNumber =======================================================
// ===========================================================================================

/// Implementors know how to become a `Number`.
pub trait ToNumber {
    fn to_number(&self) -> Number;
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

impl From<HexChar> for Number {
    fn from(n: HexChar) -> Self {
        Number::Int((n as u8).into())
    }
}

impl From<&HexChar> for Number {
    fn from(n: &HexChar) -> Self {
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

// ===========================================================================================
// ========================== TryFrom ========================================================
// ===========================================================================================

impl TryFrom<f64> for Number {
    type Error = NumberError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        let bd = value.to_string().parse::<BigDecimal>()?;
        Ok(Number::Decimal(bd))
    }
}

// ===========================================================================================
// ========================== FromStr ========================================================
// ===========================================================================================

impl FromStr for Number {
    type Err = NumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // If we were given a binary string.
        if let Ok(n) = Number::from_binary_str(s) {
            return Ok(n);
        }
        // If we were given a hexadecimal string.
        if let Ok(n) = Number::from_hexadecimal_str(s) {
            return Ok(n);
        }
        // If we were given a decimal string.
        if let Ok(i) = s.parse::<BigInt>() {
            return Ok(Number::Int(i));
        }
        if let Ok(d) = s.parse::<BigDecimal>() {
            return Ok(Number::Decimal(d));
        }
        // Fall through to error.
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
    use crate::{number::ToNumber, *};
    use rstest::*;
    use std::str::FromStr as _;

    #[test]
    fn round_trip_binary_conversion() {
        let i = 123.to_number(); // Number::Int(123)
        let bs = format!("{i:b}"); // "1111011"
        // Parse binary string back into `Number` - needs "0b" prefix.
        let s = format!("0b{bs}");
        let n = s.parse::<Number>().unwrap(); // Number::Int(123)
        assert_eq!(i, n);

        let i = 382.619.to_number(); // Number::Decimal(382.619)
        let bs = format!("{i:b}"); // "1111011"
        // Parse binary string back into `Number` - needs "0b" prefix.
        let s = format!("0b{bs}");
        let n = s.parse::<Number>().unwrap(); // Number::Decimal(382.619)
        assert_eq!(i, n);
    }

    #[rstest]
    #[case::from_str1("2.2", "2.2")]
    #[case::from_str2("1", "1")]
    #[case::from_str3("0b00000000000001110001110101110101.1000011011", "466293.539")]
    #[case::from_str4("0b-00000000000001110001110101110101.1000011011", "-466293.539")]
    #[case::no_binary_prefix_dont_treat_as_binary("10101011001", "10101011001")]
    #[case::from_str5("0b1010", "10")]
    #[case::from_str6("0b1010.1010", "10.10")]
    #[case::from_str7("0b-11110000010100011111", "-984351")]
    #[should_panic]
    #[case::from_str_panic("abcd", "")]
    #[should_panic]
    #[case::from_str_panic_contains_invalid_num_3("0b101010131001", "")]
    #[should_panic]
    #[case::from_str_panic_multiple_neg("0b-101010-131001", "")]
    #[should_panic]
    #[case::from_str_panic_multiple_decimals("0b1010.1013.1001", "")]
    #[should_panic]
    #[case::from_str_panic("   ", "")]
    #[should_panic]
    #[case::from_str_panic("0b", "")]
    fn from_str(#[case] number: &str, #[case] expect: &str) {
        let x = Number::from_str(number).expect("Number::from_str");
        let e = expect.parse::<Number>().expect("to parse 'expect' param");
        assert_eq!(x, e, "expected '{e:?}' got '{x:?}'");
    }

    #[rstest]
    #[case::from_str_hex1("0x20FDE.3CBD04", "135134.3980548")]
    #[case::from_str_hex2("0x-20FDE.3CBD04", "-135134.3980548")]
    #[case::from_str_hex3("0x1", "1")]
    #[case::from_str_hex4(
        "0xd0d0c7c5742a63ee3d89fb998ca24c7a",
        "277563472713248395635956171186146266234"
    )]
    //#[case::from_str_hex5()]
    //#[case::from_str_hex6()]
    //#[case::from_str_hex7()]
    //#[case::from_str_hex8()]
    //#[case::from_str_hex9()]
    //#[case::from_str_hex10()]
    fn from_hex_str(#[case] number: &str, #[case] expect: &str) {
        let x = Number::from_hexadecimal_str(number).expect("hex to Number");
        let e = expect.parse::<Number>().expect("control string to parse");
        assert_eq!(x, e, "expected '{e:?}' got '{x:?}'");
    }

    #[rstest]
    #[case::bin_str_to_number1("0b1010", "10")]
    #[case::bin_str_to_number2("0b-1010", "-10")]
    #[case::bin_str_to_number3("0b00000000000001110001110101110101.1000011011", "466293.539")]
    #[case::bin_str_to_number4("0b-00000000000001110001110101110101.1000011011", "-466293.539")]
    fn binary_str_to_number(#[case] number: &str, #[case] expect: &str) {
        let x = match Number::from_str(number) {
            Ok(r) => r,
            Err(e) => panic!("ERROR => '{number}' is not a binary string => {e:?}"),
        };
        let e = expect
            .parse::<Number>()
            .expect("expected 'expect' argument to parse just fine into Number");
        assert_eq!(x, e, "expected '{e:?}' got '{x:?}'");
    }

    #[test]
    fn from_f64() {
        let a = Number::from_f64(1.1).unwrap();
        assert_eq!(a.order(), NumberOrder::Decimal);
    }
}
