use crate::Number;
use std::fmt;

// ===========================================================================================
// ========================== Number impl ====================================================
// ===========================================================================================

impl Number {
    /// Applies custom formatting logic.
    /// See [Formatting] for more information.
    ///
    /// ```rust
    /// use calcinum::{Number, Formatting};
    ///
    /// let n = "123.456".parse::<Number>().expect("Number::Decimal");
    /// let fmt = n.format(Formatting::Decimal { scale: 1 });
    /// assert_eq!(fmt, "123.4".to_string());
    /// ```
    ///
    /// See [`Formatting` variants](crate::Formatting#variants) for more examples.
    pub fn format(&self, formatting: Formatting) -> String {
        formatting.apply(self)
    }

    /// Formats `self` as it's binary string represenation.
    ///
    /// We format decimals that contain a fractional part literally. Meaning, we format
    /// the integer part and fractional part separately, then combine them via a decimal
    /// while preserving the sign.
    ///
    /// **`Number::Decimal(_)` Variant:**
    ///
    /// - **If we are unable to convert the integer part to a binary string we return `self.to_string()` instead.**
    /// - **If we are unable to convert the fractional part (if one exists) we only return the integer part**
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let number = "-123.123".parse::<Number>().expect("Number::Decimal");
    /// let expect = "-1111011.1111011".to_string();
    /// let number_bin = number.to_binary_str();
    /// assert_eq!(number_bin, expect);
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
    /// **`Number::Decimal(_)` Variant:**
    ///
    /// - **If we are unable to convert the integer part to a hexadecimal string we return `self.to_string()` instead.**
    /// - **If we are unable to convert the fractional part (if one exists) we only return the integer part**
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let n = "-123.123".parse::<Number>().expect("Number::Decimal");
    /// assert_eq!(n.to_hex_str(true), "-7B.7B".to_string());
    /// assert_eq!(n.to_hex_str(false), "-7b.7b".to_string());
    /// ```
    pub fn to_hex_str(&self, uppercase: bool) -> String {
        if uppercase {
            format!("{self:X}")
        } else {
            format!("{self:x}")
        }
    }

    /// We expect a binary string to start with `"0b"`.
    /// A binary string can contain:
    /// - Digits `0` or `1`.
    /// - A single negative sign, e.g., `-`, required to be at the start of the string, after the `"0b"` prefix.
    /// - A decimal, e.g., `.` to denote a fractional number in binary form.
    pub(crate) fn is_binary_str(s: &str) -> bool {
        if !s.starts_with("0b") || s.is_empty() {
            return false;
        }

        let rest = s.trim_start_matches("0b");
        let mut iter = rest.chars();
        let mut seen_decimal = false;

        if rest.starts_with('-') {
            iter.next();
        }

        for c in iter {
            match c {
                // We should not see any other '-' signs.
                '-' => return false,
                '.' => {
                    if seen_decimal {
                        return false;
                    }
                    seen_decimal = true;
                }
                _ => {
                    if c != '0' && c != '1' {
                        return false;
                    }
                }
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

        let mut iter = s.chars();
        let mut seen_decimal = false;

        if s.starts_with('-') {
            iter.next();
        }

        for c in iter {
            match c {
                // We should not see any other '-' signs.
                '-' => return false,
                '.' => {
                    if seen_decimal {
                        return false;
                    }
                    seen_decimal = true;
                }
                _ => {
                    if !c.is_ascii_digit() {
                        return false;
                    }
                }
            }
        }

        true
    }
}

// ===========================================================================================
// ========================== fmt impls ======================================================
// ===========================================================================================

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
                let d_str = d.to_plain_string();
                let (int_part, fract_part) = d_str.split_once('.').unwrap_or((&d_str, ""));

                match Self::decimal_str_to_hexadecimal_str(int_part, false) {
                    Some(int_part_hex) => {
                        match Self::decimal_str_to_hexadecimal_str(fract_part, false) {
                            Some(fract_part_hex) => write!(f, "{int_part_hex}.{fract_part_hex}"),
                            None => write!(f, "{int_part_hex}"),
                        }
                    }
                    None => write!(f, "{d}"),
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
                let d_str = d.to_plain_string();
                let (int_part, fract_part) = d_str.split_once('.').unwrap_or((&d_str, ""));

                match Self::decimal_str_to_hexadecimal_str(int_part, true) {
                    Some(int_part_hex) => {
                        match Self::decimal_str_to_hexadecimal_str(fract_part, true) {
                            Some(fract_part_hex) => write!(f, "{int_part_hex}.{fract_part_hex}"),
                            None => write!(f, "{int_part_hex}"),
                        }
                    }
                    None => write!(f, "{d}"),
                }
            }
        }
    }
}

// ===========================================================================================
// ========================== Formatting =====================================================
// ===========================================================================================

#[derive(Debug, Clone)]
/// [`Number`] can contain arbitrarily sized numbers, so we cannot use the built-in formatting.
pub enum Formatting {
    /// How many digits to show after the decimal.
    ///
    /// ```rust
    /// use calcinum::{Number, Formatting};
    ///
    /// let number = "12.3456789".parse::<Number>().expect("Number::Decimal");
    /// let expect = "12.3456".to_string();
    /// let format = Formatting::Decimal { scale: 4 };
    /// let formatted = format.apply(&number);
    /// assert_eq!(formatted, expect);
    /// // Can also use the [`.format(...)`] method on instances of [`Number`].
    /// let formatted = number.format(format);
    /// assert_eq!(formatted, expect);
    /// ```
    Decimal { scale: usize },

    /// How many total digits to show. Symbols like `-` and `.` do not affect digit count.
    ///
    /// ```rust
    /// use calcinum::{Number, Formatting};
    ///
    /// let number = "12345.678".parse::<Number>().expect("Number::Decimal");
    /// let expect = "12345.6".to_string();
    /// let format = Formatting::Digits { width: 6 };
    /// let formatted = format.apply(&number);
    /// assert_eq!(formatted, expect);
    /// // Can also use the [`.format(...)`] method on instances of [`Number`].
    /// let formatted = number.format(format);
    /// assert_eq!(formatted, expect);
    /// ```
    Digits { width: usize },

    /// Format as binary string with `separator` and `group_by` chunks.
    /// Formats decimals with a fractional part literally. Meaning, we format each side of
    /// the decimal separately, then combine them via a decimal while preserving the sign.
    ///
    /// This will also auto-pad the decimal string to make it 'even'.
    ///
    /// `separator` : delimiter used to separate groups.
    ///
    /// `group_by` : how many binary digits that will appear consecutively before a `separator`.
    ///
    /// ```rust
    /// use calcinum::{Number, Formatting};
    ///
    /// let number = Number::from(u64::MAX);
    /// let expect = String::from("11111111 11111111 11111111 11111111 11111111 11111111 11111111 11111111");
    /// let format = Formatting::Binary {
    ///     separator: " ".to_string(),
    ///     group_by: 8,
    /// };
    /// let formatted = format.apply(&number);
    /// assert_eq!(formatted, expect);
    /// let formatted = number.format(format);
    /// assert_eq!(formatted, expect);
    /// ````
    Binary { separator: String, group_by: usize },
}

impl Formatting {
    /// See the documentation for [`Formatting` variants](crate::Formatting#variants) for more details.
    pub fn apply(&self, number: &Number) -> String {
        match *self {
            Formatting::Digits { width } => Self::apply_digits_formatting(number, width),
            Formatting::Decimal { scale } => Self::apply_decimal_digits_formatting(number, scale),
            Formatting::Binary {
                ref separator,
                group_by,
            } => Self::apply_binary_formatting(number, separator, group_by),
        }
    }

    fn apply_decimal_digits_formatting(number: &Number, precision: usize) -> String {
        let num_str = number.to_string();

        if number.is_int() {
            return num_str;
        }

        let (int_part, fract_part) = num_str.split_once('.').unwrap_or((&num_str, ""));

        if precision == 0 {
            return int_part.to_string();
        }

        if fract_part.len() <= precision {
            return num_str;
        }

        let truncated = &fract_part[..precision];
        format!("{int_part}.{truncated}")
    }

    fn apply_digits_formatting(number: &Number, width: usize) -> String {
        let num_str = number.to_string();
        let mut fmted = String::new();
        let mut seen_digits = 0;

        for c in num_str.chars() {
            if seen_digits >= width {
                break;
            }

            fmted.push(c);

            if c != '-' && c != '.' {
                seen_digits += 1;
            }
        }

        fmted
    }

    fn apply_binary_formatting(number: &Number, separator: &str, grouping: usize) -> String {
        let bs = format!("{number:b}");
        let (ip, fp) = bs.split_once('.').unwrap_or((&bs, ""));

        let mut fmted = Self::format_binary_str(ip, separator, grouping);
        let bfp = Self::format_binary_str(fp, separator, grouping);

        if !bfp.is_empty() {
            fmted.push_str(&format!(".{bfp}"));
        }

        fmted
    }

    /// Assumes you have alredy verified the `bin_str` you are passing in is binary.
    /// Handles negative binary strings ('-' at start of string).
    fn format_binary_str(bin_str: &str, separator: &str, grouping: usize) -> String {
        if bin_str.is_empty() {
            return String::new();
        }
        if grouping == 0 {
            return bin_str.to_string();
        }

        let (sign, digits) = if let Some(stripped) = bin_str.strip_prefix('-') {
            ("-", stripped)
        } else {
            ("", bin_str)
        };

        let len = digits.len();
        let target_len = Formatting::next_pos_multiple_inclusive(grouping, len);
        let pad_by = target_len.saturating_sub(len);

        let mut padded = "0".repeat(pad_by);
        padded.push_str(digits);

        let mut result = String::new();
        result.push_str(sign);

        let chars: Vec<_> = padded.chars().collect();
        for (i, chunk) in chars.rchunks(grouping).rev().enumerate() {
            if i > 0 {
                result.push_str(separator);
            }
            for c in chunk {
                result.push(*c);
            }
        }

        result
    }

    /// Finds the next multiple, `m`,  for an unsigned size, `n`.
    /// If `n` is already a multiple of `m`, we return `n`.
    /// If `m` or `n` are 0, we return 0.
    fn next_pos_multiple_inclusive(m: usize, n: usize) -> usize {
        if m == 0 || n == 0 {
            return 0;
        }
        if n.is_multiple_of(m) {
            return n;
        };
        ((n / m) + 1) * m
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
    #[case::next_pos_multiple1(3, 34, 36)]
    #[case::next_pos_multiple2(103, 34, 103)]
    #[case::next_pos_multiple3(5, 0, 0)]
    #[case::next_pos_multiple4(1, 999, 999)]
    #[case::next_pos_multiple5(10, 10, 10)]
    #[case::next_pos_multiple6(10, 11, 20)]
    #[case::next_pos_multiple7(7, 14, 14)]
    #[case::next_pos_multiple8(7, 15, 21)]
    #[case::next_pos_multiple9(8, 64, 64)]
    #[case::next_pos_multiple10(8, 65, 72)]
    #[case::next_pos_multiple11(1024, 1_000_000, 1_000_448)]
    #[case::next_pos_multiple_edge(13, 26, 26)]
    #[case::next_pos_multiple_div_zero(0, 13, 0)]
    #[case::next_pos_multiple_div_zero(13, 0, 0)]
    fn next_pos_multiple(#[case] m: usize, #[case] n: usize, #[case] expect: usize) {
        let a = Formatting::next_pos_multiple_inclusive(m, n);
        assert_eq!(a, expect, "expected {expect} got {a}");
    }

    #[rstest]
    #[case::formatting_decimals1("-12345.6789", 2, "-12345.67")]
    #[case::formatting_decimals2("12345.6", 0, "12345")]
    #[case::formatting_decimals3("12345.6789", 20, "12345.6789")]
    fn formatting_decimals(#[case] number: &str, #[case] scale: usize, #[case] expect: &str) {
        let n = number.parse::<Number>().unwrap();
        let decimals = n.format(Formatting::Decimal { scale });
        assert_eq!(
            decimals, expect,
            "expected decimals '{expect}' got decimals '{decimals}'"
        );
    }

    #[rstest]
    #[case::formatting_digits1("-12345.6789", 7, "-12345.67")]
    fn formatting_digits(#[case] number: &str, #[case] width: usize, #[case] expect: &str) {
        let n = number.parse::<Number>().expect("number str to Number");
        let digits = n.format(Formatting::Digits { width });
        assert_eq!(
            digits, expect,
            "expected digits '{expect}' got digits '{digits}'"
        );
    }

    #[rstest]
    #[case::formatting_binary1("-12345.6789", " ", 4, "-0011 0000 0011 1001.0001 1010 1000 0101")]
    #[case::formatting_binary2("0", " ", 4, "0000")]
    #[case::formatting_binary3("1", " ", 4, "0001")]
    #[case::formatting_binary4("15", " ", 4, "1111")]
    #[case::formatting_binary5("16", " ", 4, "0001 0000")]
    #[case::formatting_binary6("255", " ", 4, "1111 1111")]
    #[case::formatting_binary7("256", " ", 4, "0001 0000 0000")]
    #[case::formatting_binary8("-1", " ", 4, "-0001")]
    #[case::formatting_binary9("-255", " ", 4, "-1111 1111")]
    #[case::formatting_binary10("1023", "_", 8, "00000011_11111111")]
    #[case::formatting_binary11("42", "-", 3, "101-010")]
    #[case::formatting_binary12(&u64::MAX.to_string(), " ", 8, "11111111 11111111 11111111 11111111 11111111 11111111 11111111 11111111")]
    #[case::formatting_binary13("3", " ", 8, "00000011")]
    fn formatting_binary(
        #[case] number: &str,
        #[case] separator: &str,
        #[case] group_by: usize,
        #[case] expect: &str,
    ) {
        let n = number.parse::<Number>().expect("number str to Number");
        let binary = n.format(Formatting::Binary {
            separator: separator.to_string(),
            group_by,
        });
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
        let r = n.to_hex_str(uppercase);
        assert_eq!(r, e, "expected hex '{e}' got hex '{r}'");
    }
}
