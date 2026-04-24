use crate::{
    Number, NumberError,
    number::{conversion, digit::HexDigit, predicate},
};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{FromPrimitive, Num as _};
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

    /// Converts hexadecimal string into `Number`.
    ///
    /// <div class="warning">
    ///
    /// # A valid hexadecimal string
    ///
    /// ```text
    ///   -FFA.FFA
    ///   | | |
    ///   | | +---- A single decimal anywhere after `0x` (or `-0x`) prefix
    ///   | +------ Any amount of valid hexadecimal characters (see below)
    ///   +-------- A single negative sign; only allowed as first char
    /// ```
    ///
    /// </div>
    ///
    /// # Valid hexadecimal characters
    /// - Any combination of:
    ///   - digits `0`-`9`
    ///   - characters (case **in**sensitive) `A`, `B`, `C`, `D`, `E`, `F`
    ///
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let a = Number::from_hexadecimal_str("-63A.2675").expect("Number::Decimal");
    /// assert_eq!(a, "-1594.9845".parse::<Number>().expect("eq"));
    /// ```
    ///
    pub fn from_hexadecimal_str(hex_str: &str) -> Result<Number, NumberError> {
        Self::from_hexadecimal_str_with_prefix(hex_str, false)
    }

    /// See comments on [`from_hexadecimal_str`]
    pub(crate) fn from_hexadecimal_str_with_prefix(
        hex_str: &str,
        validate_prefix: bool,
    ) -> Result<Number, NumberError> {
        if !predicate::is_hexadecimal_str(hex_str, validate_prefix) {
            return Err(NumberError::Parsing {
                value: format!("'{hex_str}' is not a hexadecimal string"),
            });
        }

        let (is_signed, s) = match hex_str.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, hex_str),
        };

        let s = if validate_prefix {
            s.strip_prefix("0x").unwrap_or(s)
        } else {
            s
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

    /// Converts an octal string into a [`Number`].
    ///
    /// <div class="warning">
    ///
    /// # A valid octal string
    ///
    /// ```text
    ///   -173.173
    ///   | | |
    ///   | | +---- A single decimal anywhere after `0o` (or `-0o`) prefix
    ///   | +------ Any amount of valid octal characters (see below)
    ///   +-------- A single negative sign; only allowed as first char
    /// ```
    ///
    /// </div>
    ///
    /// # Valid Octal Characters
    /// - Any combination of digits `0`-`7`
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let os = "-173.173";
    /// let num = Number::from_octal_str(os).expect("octal to Number");
    /// let expect = "-123.123".parse::<Number>().expect("Number");
    /// assert_eq!(num, expect);
    /// ```
    pub fn from_octal_str(octal_str: &str) -> Result<Number, NumberError> {
        Self::from_octal_str_with_prefix(octal_str, false)
    }

    /// See comments on [`from_octal_str`](crate::Number#method.from_octal_str)
    pub(crate) fn from_octal_str_with_prefix(
        octal_str: &str,
        validate_prefix: bool,
    ) -> Result<Number, NumberError> {
        if !predicate::is_octal_str(octal_str, validate_prefix) {
            return Err(NumberError::Parsing {
                value: format!("string '{octal_str}' is not an octal string"),
            });
        }

        let (is_signed, s) = match octal_str.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, octal_str),
        };

        let s = if validate_prefix {
            s.strip_prefix("0o").unwrap_or(s)
        } else {
            s
        };

        let (int_part, fract_part) = s.split_once('.').unwrap_or((s, ""));
        let int_part_len = int_part.len();
        let fract_part_len = fract_part.len();
        let base = Number::from(8);

        let mut int = int_part.chars().enumerate().try_fold(
            Number::ZERO,
            |acc, (i, c)| -> Result<_, NumberError> {
                let exponent = int_part_len as u32 - 1 - i as u32;
                let multiplier = base.pow(exponent as i64)?;
                let digit = c.to_string().parse::<Number>()?;
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
                    let digit = c.to_string().parse::<Number>().ok()?;
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

    /// Converts a base64 string into [`Number`].
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let a = Number::from_base64_str("OTk5OTk=").expect("Number::Int");
    /// assert_eq!(a, Number::from(99999));
    /// ```
    pub fn from_base64_str(s: &str) -> Result<Number, NumberError> {
        Self::from_base64_str_with_prefix(s, false)
    }

    /// See comments on [`from_base64_str`]
    pub(crate) fn from_base64_str_with_prefix(
        s: &str,
        validate_prefix: bool,
    ) -> Result<Number, NumberError> {
        if validate_prefix && (s == "b64" || !s.starts_with("b64")) {
            return Err(NumberError::InvalidArgument);
        }

        let s = if validate_prefix {
            s.strip_prefix("b64").unwrap_or(s)
        } else {
            s
        };

        let d = base64_decode(s);
        d.parse::<Number>()
    }

    /// Converts the binary string into [`Number`].
    pub fn from_binary_str(s: &str) -> Result<Self, NumberError> {
        Self::from_binary_str_with_prefix(s, false)
    }

    /// See comments on [`from_base64_str`]
    pub(crate) fn from_binary_str_with_prefix(
        s: &str,
        validate_prefix: bool,
    ) -> Result<Number, NumberError> {
        let s = s.trim();
        // We were given "" or just the prefix to a binary string "0b" or "-0b"
        if s.is_empty() || s == "-0b" || s == "0b" || s == "-" {
            return Err(NumberError::Parsing {
                value: format!("'{s}' either contains no binary or it is empty"),
            });
        }
        if !predicate::is_binary_str(s, validate_prefix) {
            return Err(NumberError::Parsing {
                value: format!("'{s}' is not a binary string"),
            });
        }

        let (is_negative, s) = match s.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, s),
        };

        let s = if validate_prefix {
            s.strip_prefix("0b").unwrap_or(s)
        } else {
            s
        };

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

/// Decode a base64 string to it's original form.
pub(crate) fn base64_decode(s: &str) -> String {
    let s = s.trim_end_matches('=');
    let alpha = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut decoded = String::new();
    let mut buf = 0;
    let mut bits = 0;

    for byte in s.as_bytes() {
        let value = alpha.iter().position(|c| c == byte).unwrap_or(0);
        buf = (buf << 6) | value as u32;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            decoded.push(((buf >> bits) as u8) as char);
        }
    }

    decoded
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

    /// When using `from_str` each format, outside of a decimal string,
    /// requires a specific prefix (see below).
    ///
    /// **If you do not want to worry about a prefix, please use [`Number::from_str_radix`]**.
    ///
    /// The following radicies require a special prefix :
    ///
    /// | Radix | Format | Prefix |
    /// | :---: | :----: | :----: |
    /// | 2     | binary | `0b`   |
    /// | 6     |  hex   | '0x'   |
    /// | 8     | octal  | '0o'   |
    /// | 64    | base64 | 'b64'  |
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = Number::from_binary_str_with_prefix(s, true) {
            return Ok(n);
        }
        if let Ok(n) = Number::from_hexadecimal_str_with_prefix(s, true) {
            return Ok(n);
        }
        if let Ok(n) = Number::from_octal_str_with_prefix(s, true) {
            return Ok(n);
        }
        if let Ok(n) = Number::from_base64_str_with_prefix(s, true) {
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

#[cfg(test)]
mod test {
    use crate::*;
    use rstest::*;
    use std::str::FromStr as _;

    #[rstest]
    #[case::from_str1("2.2", "2.2")]
    #[case::from_str2("1", "1")]
    #[case::from_str3("0b00000000000001110001110101110101.1000011011", "466293.539")]
    #[case::from_str4("-0b00000000000001110001110101110101.1000011011", "-466293.539")]
    #[case::no_binary_prefix_dont_treat_as_binary("10101011001", "10101011001")]
    #[case::from_str5("0b1010", "10")]
    #[case::from_str6("0b1010.1010", "10.10")]
    #[case::from_str7("-0b11110000010100011111", "-984351")]
    #[should_panic]
    #[case::from_str_panic("abcd", "")]
    #[should_panic]
    #[case::from_str_panic_contains_invalid_num_3("0b101010131001", "")]
    #[should_panic]
    #[case::from_str_panic_multiple_neg("-0b101010-131001", "")]
    #[should_panic]
    #[case::from_str_panic_multiple_decimals("0b1010.1013.1001", "")]
    #[should_panic]
    #[case::from_str_panic("   ", "")]
    #[should_panic]
    #[case::from_str_panic("0b", "")]
    #[case::from_str_b64_1("b64LTIzNDUuMTIzNQ==", "-2345.1235")]
    #[case::from_str_b64_2("b64NDM1NDMuMzIyOTM4NDAz", "43543.322938403")]
    #[case::from_str_b64_3("b64NDM1MjQzOTg1MjQzMzE0OQ==", "4352439852433149")]
    #[case::from_str_b64_4("b64LTAwMDAwMDAwMC4wMDAwMDAwMDAw", "-000000000.0000000000")]
    ///
    /// `from_str` REQUIRES FORMATS HAVE A SPECIFIC PREFIX!
    ///
    fn from_str(#[case] number: &str, #[case] expect: &str) {
        let x = Number::from_str(number).expect("Number::from_str");
        let e = expect.parse::<Number>().expect("to parse 'expect' param");
        assert_eq!(x, e, "expected '{e:?}' got '{x:?}'");
    }

    #[rstest]
    #[case::from_str_hex1("20FDE.3CBD04", "135134.3980548")]
    #[case::from_str_hex2("-20FDE.3CBD04", "-135134.3980548")]
    #[case::from_str_hex3("1", "1")]
    #[case::from_str_hex4(
        "d0d0c7c5742a63ee3d89fb998ca24c7a",
        "277563472713248395635956171186146266234"
    )]
    fn from_hex_str(#[case] number: &str, #[case] expect: &str) {
        let x = Number::from_hexadecimal_str(number).expect("hex to Number");
        let e = expect.parse::<Number>().expect("control string to parse");
        assert_eq!(x, e, "expected '{e:?}' got '{x:?}'");
    }

    #[rstest]
    #[case::from_octal_str("726746425", "123456789")]
    #[case::from_octal_str("-173.173", "-123.123")]
    fn from_octal_str(#[case] number: &str, #[case] expect: &str) {
        let x = Number::from_octal_str(number).expect("octal to number");
        let e = expect.parse::<Number>().expect("control to parse");
        assert_eq!(x, e, "expected '{e:?}' got '{x:?}'");
    }

    #[rstest]
    #[case::from_b64_str_1("LTIzNDUuMTIzNQ==", "-2345.1235")]
    #[case::from_b64_str_2("NDM1NDMuMzIyOTM4NDAz", "43543.322938403")]
    #[case::from_b64_str_3("NDM1MjQzOTg1MjQzMzE0OQ==", "4352439852433149")]
    #[case::from_b64_str_4("LTAwMDAwMDAwMC4wMDAwMDAwMDAw", "-000000000.0000000000")]
    fn from_base64_str(#[case] number: &str, #[case] expect: &str) {
        let x = Number::from_base64_str(number).expect("b64 to number");
        let e = expect.parse::<Number>().expect("control to parse");
        assert_eq!(x, e, "expected '{e:?}' got '{x:?}'");
    }

    #[rstest]
    #[case::from_str3("00000000000001110001110101110101.1000011011", "466293.539")]
    #[case::from_str4("-00000000000001110001110101110101.1000011011", "-466293.539")]
    #[case::from_str5("1010", "10")]
    #[case::from_str6("1010.1010", "10.10")]
    #[case::from_str7("-11110000010100011111", "-984351")]
    fn from_binary_str(#[case] number: &str, #[case] expect: &str) {
        let x = Number::from_binary_str(number).expect("binary to number");
        let e = expect.parse::<Number>().expect("control to parse");
        assert_eq!(x, e, "expected '{e:?}' got '{x:?}'");
    }

    #[test]
    fn from_f64() {
        let a = Number::from_f64(1.1).unwrap();
        assert_eq!(a.order(), NumberOrder::Decimal);
    }
}
