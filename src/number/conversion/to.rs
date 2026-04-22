use crate::{
    Number, NumberError,
    number::{digit::HexDigit, predicate},
};
use num_traits::{Signed, ToPrimitive};
use std::str::FromStr;

impl Number {
    /// Formats `self` as it's binary string represenation.
    ///
    /// We format decimals that contain a fractional part literally. Meaning, we format
    /// the integer part and fractional part separately, then combine them via a decimal
    /// while preserving the sign.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let number = "-123.123".parse::<Number>().expect("Number::Decimal");
    /// assert_eq!(number.to_binary_str(), "-1111011.1111011".to_string());
    /// ```
    pub fn to_binary_str(&self) -> String {
        format!("{self:b}")
    }

    /// Formats `self` as it's hexadecimal representation.
    ///
    /// We format decimals that contain a fractional part literally. Meaning, we format
    /// the integer part and fractional part separately, then combine them via a decimal
    /// while preserving the sign.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let n = "-123.123".parse::<Number>().expect("Number::Decimal");
    ///
    /// let uppercase = true;
    /// assert_eq!(n.to_hexadecimal_str(uppercase), "-7B.7B".to_string());
    ///
    /// let uppercase = false;
    /// assert_eq!(n.to_hexadecimal_str(uppercase), "-7b.7b".to_string());
    /// ```
    pub fn to_hexadecimal_str(&self, uppercase: bool) -> String {
        if uppercase {
            format!("{self:X}")
        } else {
            format!("{self:x}")
        }
    }

    /// Formats `self` as a base64 string.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let a = "-2345.1235".parse::<Number>().expect("Number::Decimal");
    /// assert_eq!(a.to_base64_str(), "LTIzNDUuMTIzNQ==".to_string());
    /// ```
    pub fn to_base64_str(&self) -> String {
        Self::base64_encode(&self.to_string())
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
            Number::Int(i) => saturating_i64(i),
            Number::Decimal(d) => saturating_i64(d),
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
            Number::Int(i) => saturating_i128(i),
            Number::Decimal(d) => saturating_i128(d),
        }
    }
}

impl ToPrimitive for Number {
    fn to_i64(&self) -> Option<i64> {
        match self {
            Number::Int(i) => i.to_i64(),
            Number::Decimal(d) => d.to_i64(),
        }
    }

    fn to_u64(&self) -> Option<u64> {
        match self {
            Number::Int(i) => i.to_u64(),
            Number::Decimal(d) => d.to_u64(),
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

/// Converts a decimal string to a binary string
/// # A valid decimal string
/// ```text
///   -123.123`
///   | | |
///   | | +-- A single decimal anywhere after `-`
///   | +-- Any amount off digits 0-9
///   +---- A negative sign; only allowed as first char
/// ```
pub(crate) fn decimal_str_to_binary_str(decimal_str: &str) -> Option<String> {
    if decimal_str == "0" || decimal_str.is_empty() {
        return Some("0".to_string());
    }
    if !predicate::is_decimal_str(decimal_str) {
        return None;
    }

    let (is_negative, decimal_str) = match decimal_str.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, decimal_str),
    };

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

    Some(binary_bits.chars().rev().collect())
}

/// Converts a decimal string to a hexadecimal string.
/// # A valid decimal string
/// ```text
///   -123.123`
///   | | |
///   | | +-- A single decimal anywhere after `-`
///   | +-- Any amount off digits 0-9
///   +---- A negative sign; only allowed as first char
/// ```
pub(crate) fn decimal_str_to_hexadecimal_str(
    decimal_str: &str,
    uppercase: bool,
) -> Result<String, NumberError> {
    if decimal_str == "0" || decimal_str.is_empty() {
        return Ok("0".to_string());
    }

    let mut iter = decimal_str.chars().peekable();
    let mut is_negative = false;

    if let Some(&p) = iter.peek()
        && p == '-'
    {
        is_negative = true;
        iter.next();
    };

    let mut int_part = String::new();
    let mut fract_part = String::new();
    let mut seen_decimal = false;

    for c in iter {
        match c {
            // Already checked front
            '-' => return Err(NumberError::InvalidArgument),
            '.' if !seen_decimal => seen_decimal = true,
            c if c.is_ascii_digit() => {
                if !seen_decimal {
                    int_part.push(c);
                } else {
                    fract_part.push(c);
                }
            }
            _ => return Err(NumberError::InvalidArgument),
        }
    }

    let mut int_dividend = int_part.parse::<Number>()?;
    let mut int_str = String::new();

    loop {
        let (int_quotient, int_remainder) = int_dividend.div_mod(16);
        let int_char = HexDigit::from_str(&int_remainder.to_string())?.to_char(uppercase);
        int_str.push(int_char);
        if int_quotient.is_zero() {
            break;
        }
        int_dividend = int_quotient;
    }

    if is_negative {
        int_str.push('-');
    }

    let mut output = int_str.chars().rev().collect();

    // RETURN if there is no more work to do.
    if !seen_decimal {
        return Ok(output);
    }

    output.push('.');

    let mut fract_dividend = fract_part.parse::<Number>()?;
    let mut fract_str = String::new();

    loop {
        let (fract_quotient, fract_remainder) = fract_dividend.div_mod(16);
        let fract_char = HexDigit::from_str(&fract_remainder.to_string())?.to_char(uppercase);
        fract_str.push(fract_char);
        if fract_quotient.is_zero() {
            break;
        }
        fract_dividend = fract_quotient;
    }

    let fract_output: String = fract_str.chars().rev().collect();
    output.push_str(&fract_output);

    Ok(output)
}

/// Assumes you have already validated that what you are passing in is ACTUALLY a binary string!
pub(crate) fn binary_str_to_decimal_str(bin: &str) -> String {
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
