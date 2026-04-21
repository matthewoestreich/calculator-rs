use crate::{
    Number,
    number::{
        fmt::spec::{FormatSpec, Formatter},
        hexchar::HexChar,
    },
};
use std::fmt;

impl Number {
    /// Applies custom formatting logic.
    /// See [`Formatting`](crate#cli-formatting) for more examples.
    ///
    /// If you provide an invalid format string, we just return `self` as a `String`
    /// without any formatting.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let n = Number::from(123);
    ///
    /// // Format number as binary.
    /// let a = n.format("b");
    /// assert_eq!(&a, "1111011");
    ///
    /// // Format number as binary with a width of 12, non zero padded.
    /// let b = n.format("12b");
    /// assert_eq!(&b, "     1111011");
    ///
    /// // Format number as binary with a width o 12, zero padded.
    /// let c = n.format("012b");
    /// assert_eq!(&c, "000001111011");
    ///
    /// // Format number as binary width a width of 12, zero padded, groups of 4.
    /// let d = n.format("012b4");
    /// assert_eq!(&d, "0000 0111 1011");
    /// ```
    ///
    /// See [`Formatting`](crate#cli-formatting) for more examples.
    pub fn format(&self, fmt_spec_str: &str) -> String {
        FormatSpec::parse(fmt_spec_str).map_or_else(
            |_| self.to_string(),
            |spec| Formatter::format_number(self, spec).unwrap_or(self.to_string()),
        )
    }

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

    /// We expect a binary string to start with `"0b"` or `"-0b"` for negative binary strings.
    /// A binary string can contain:
    /// - Digits `0` or `1`.
    /// - A single negative sign, e.g., `-`, required to be at the start of the string
    /// - A decimal, e.g., `.` to denote a fractional number in binary form.
    pub(crate) fn is_binary_str(s: &str) -> bool {
        if !s.starts_with("0b") && !s.starts_with("-0b") || s.is_empty() {
            return false;
        }

        let s = s.strip_prefix('-').unwrap_or(s);
        let s = s.strip_prefix("0b").unwrap_or(s);
        let mut seen_decimal = false;

        for c in s.chars() {
            match c {
                // We should not see any other '-' signs.
                '-' => return false,
                '.' if !seen_decimal => seen_decimal = true,
                c if c == '0' || c == '1' => {}
                _ => return false,
            }
        }

        true
    }

    /// We expect a hexadecimal string to start with `"0x"`.
    /// An empty string will return `false`.
    /// A hexadecimal string can contain (in any order):
    /// - Digits `0` - `9`.
    /// - Characters (case insensitive) `A`, `B`, `C`, `D`, `E`, `F`.
    /// - A single negative sign, e.g., `-`, required to be at the start of the string, after the `"0b"` prefix.
    /// - A decimal, e.g., `.` to denote a fractional number in binary form.
    pub(crate) fn is_hexadecimal_str(s: &str) -> bool {
        if (!s.starts_with("-0x") && !s.starts_with("0x")) || s.is_empty() {
            return false;
        }

        let s = s.strip_prefix('-').unwrap_or(s);
        let s = s.strip_prefix("0x").unwrap_or(s);
        let mut seen_decimal = false;

        for c in s.chars() {
            match c {
                // We should not see any other '-' signs.
                '-' => return false,
                '.' if !seen_decimal => seen_decimal = true,
                c if HexChar::try_from(c).is_ok() => {}
                _ => return false,
            }
        }

        true
    }

    /// Checks to see if a string is considered a decimal.
    /// An empty decimal string returns `false`.
    /// We expect a decimal string to contain only:
    /// - Digits `0`-`9`.
    /// - A single negative sign, e.g., `-`, required to be at the start of the string.
    /// - A decimal, e.g., `.` to denote a decimal with a fractional part.
    pub(crate) fn is_decimal_str(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }

        let s = s.strip_prefix('-').unwrap_or(s);
        let mut seen_decimal = false;

        for c in s.chars() {
            match c {
                // We should not see any other '-' signs.
                '-' => return false,
                '.' if !seen_decimal => seen_decimal = true,
                c if c.is_ascii_digit() => {}
                _ => return false,
            }
        }

        true
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Int(i) => write!(f, "{i}"),
            Number::Decimal(d) => write!(f, "{}", d.to_plain_string()),
        }
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "Number::Int({i})"),
            Self::Decimal(d) => write!(f, "Number::Decimal({})", d.to_plain_string()),
        }
    }
}

impl fmt::Binary for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Int(i) => write!(f, "{i:b}"),
            Number::Decimal(d) => {
                let s = d.to_string();
                let (int_part, fract_part) = s.split_once('.').unwrap_or((&s, ""));

                match Self::decimal_str_to_binary_str(int_part) {
                    Some(int_part_bin) => match Self::decimal_str_to_binary_str(fract_part) {
                        Some(fract_part_bin) => write!(f, "{int_part_bin}.{fract_part_bin}"),
                        None => write!(f, "{int_part_bin}"),
                    },
                    None => write!(f, "{d}"),
                }
            }
        }
    }
}

impl fmt::LowerHex for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Int(i) => fmt::LowerHex::fmt(i, f),
            Number::Decimal(d) => {
                let ds = d.to_plain_string();
                match Self::decimal_str_to_hexadecimal_str(&ds, false) {
                    Ok(s) => write!(f, "{s}"),
                    Err(_) => write!(f, "{ds}"),
                }
            }
        }
    }
}

impl fmt::UpperHex for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Int(i) => fmt::UpperHex::fmt(i, f),
            Number::Decimal(d) => {
                let ds = d.to_plain_string();
                match Self::decimal_str_to_hexadecimal_str(&ds, true) {
                    Ok(s) => write!(f, "{s}"),
                    Err(_) => write!(f, "{ds}"),
                }
            }
        }
    }
}

// ===========================================================================================
// ========================== Tests ==========================================================
// ===========================================================================================

#[cfg(test)]
mod test {
    use super::*;
    use crate::NumberOrder;
    use rstest::*;

    #[rstest]
    #[case::formatting_binary1(
        "-12345.6789",
        true,
        Some(32),
        Some(4),
        "-0011 0000 0011 1001.0001 1010 1000 0101"
    )]
    #[case::formatting_binary2("0", false, None, Some(4), "0000")]
    #[case::formatting_binary3("1", false, None, Some(4), "0001")]
    #[case::formatting_binary3a("1", false, None, Some(4), "0001")]
    #[case::formatting_binary4("15", false, None, Some(4), "1111")]
    #[case::formatting_binary5("16", true, Some(8), Some(4), "0001 0000")]
    #[case::formatting_binary6("255", false, None, Some(4), "1111 1111")]
    #[case::formatting_binary7("256", true, Some(12), Some(4), "0001 0000 0000")]
    #[case::formatting_binary8("-1", true, Some(4), None, "-0001")]
    #[case::formatting_binary9("-255", false, None, Some(4), "-1111 1111")]
    #[case::formatting_binary10("1023", true, Some(16), Some(8), "00000011 11111111")]
    #[case::formatting_binary11("42", false, None, Some(3), "101 010")]
    #[case::formatting_binary12(&u64::MAX.to_string(), false, None, Some(8), "11111111 11111111 11111111 11111111 11111111 11111111 11111111 11111111")]
    #[case::formatting_binary13("3", true, Some(8), None, "00000011")]
    fn formatting_binary(
        #[case] number: &str,
        #[case] is_zero_pad: bool,
        #[case] width: Option<usize>,
        #[case] group_by: Option<usize>,
        #[case] expect: &str,
    ) {
        let n = number.parse::<Number>().expect("number str to Number");
        let mut fmt_str = String::new();
        if is_zero_pad {
            fmt_str.push('0');
        }
        if let Some(width) = width {
            fmt_str.push_str(&format!("{width}"));
        }
        fmt_str.push('b');
        if let Some(group) = group_by {
            fmt_str.push_str(&format!("{group}"));
        }
        let binary = n.format(&fmt_str);
        assert_eq!(
            binary, expect,
            "expected binary '{expect}' got binary '{binary}'"
        );
    }

    #[rstest]
    #[case::fmt_display1("11.1", "11.1", NumberOrder::Decimal)]
    fn fmt_display(
        #[case] number: &str,
        #[case] expect_display: &str,
        #[case] expect_order: NumberOrder,
    ) {
        let x = number.parse::<Number>().unwrap();
        let r = x.order();
        assert_eq!(
            r, expect_order,
            "expected order '{expect_order:?}' got order '{r:?}'",
        );
        let r = format!("{x}");
        assert_eq!(
            r, expect_display,
            "expected display '{expect_display}' got display '{r}'"
        );
    }

    #[rstest]
    #[case::fmt_debug1("11.1", "Number::Decimal(11.1)")]
    fn fmt_debug(#[case] number: &str, #[case] expect_display: &str) {
        let x = number.parse::<Number>().unwrap();
        let r = format!("{x:?}");
        assert_eq!(
            r, expect_display,
            "expected debug '{expect_display}' got debug '{r}'"
        );
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
    fn fmt_binary_str(#[case] number: &str, #[case] expect: &str) {
        let n = number.parse::<Number>().unwrap();
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

    #[rstest]
    #[case::to_hex_str2("0", "0", true)]
    #[case::to_hex_str3("1", "1", true)]
    #[case::to_hex_str4("10", "A", true)]
    #[case::to_hex_str5("15", "F", true)]
    #[case::to_hex_str6("16", "10", true)]
    #[case::to_hex_str7("255", "FF", true)]
    #[case::to_hex_str8("256", "100", true)]
    #[case::to_hex_str9("-1", "-1", true)]
    #[case::to_hex_str10("-255", "-FF", true)]
    #[case::to_hex_str11("123", "7b", false)]
    #[case::to_hex_str12("4095", "FFF", true)]
    #[case::to_hex_str13("4096", "1000", true)]
    #[case::to_hex_str14("65535", "FFFF", true)]
    #[case::to_hex_str15("65536", "10000", true)]
    #[case::to_hex_str16("4294967295", "FFFFFFFF", true)]
    #[case::to_hex_str17("4294967296", "100000000", true)]
    #[case::to_hex_str18("-4095", "-FFF", true)]
    #[case::to_hex_str19("123456789", "75BCD15", true)]
    #[case::to_hex_str20("123456789", "75bcd15", false)]
    #[case::to_hex_str21("-123.123", "-7B.7B", true)]
    #[case::to_hex_str22("0.5", "0.5", true)]
    #[case::to_hex_str23("1.5", "1.5", true)]
    #[case::to_hex_str24("10.25", "A.19", true)]
    #[case::to_hex_str25("15.5", "F.5", true)]
    #[case::to_hex_str26("16.75", "10.4B", true)]
    #[case::to_hex_str27("255.5", "FF.5", true)]
    #[case::to_hex_str28("-0.5", "-0.5", true)]
    #[case::to_hex_str29("-10.25", "-A.19", true)]
    #[case::to_hex_str30("1.5", "1.5", false)]
    #[case::to_hex_str31("10.25", "a.19", false)]
    fn number_to_hex_str(#[case] number: &str, #[case] expect: &str, #[case] uppercase: bool) {
        let n = number.parse::<Number>().expect("Number::<t>");
        let e = expect.to_string();
        let r = n.to_hexadecimal_str(uppercase);
        assert_eq!(r, e, "expected hex '{e}' got hex '{r}'");
    }
}
