use crate::Number;
use std::fmt;

// ===========================================================================================
// ========================== Number impl ====================================================
// ===========================================================================================

impl Number {
    pub fn format(&self, formatting: Formatting) -> String {
        formatting.apply(self)
    }

    /// If variant is `Number::Decimal` we return the integer part is binary
    /// and the fractional part as binary, separated by a period.
    /// For example, if you have a `Number::Decimal(100.773)` this method
    /// returns : `"1100100.1100000101"`
    pub fn to_binary_str(&self) -> String {
        match self {
            Number::Int(i) => format!("{i:b}"),
            Number::Decimal(d) => {
                let s = d.to_string();
                let (int_part, fract_part) = s.split_once('.').unwrap_or((&s, ""));
                let mut bin_str = Self::decimal_str_to_binary_str(int_part);

                if !fract_part.is_empty() {
                    let fract_bin = Self::decimal_str_to_binary_str(fract_part);
                    bin_str.push_str(&format!(".{fract_bin}"));
                }

                bin_str
            }
        }
    }

    pub(crate) fn is_binary_str(s: &str) -> bool {
        s.starts_with("0b")
    }

    // Helper for `.to_binary_str`
    fn decimal_str_to_binary_str(decimal_str: &str) -> String {
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
        write!(f, "{}", self.to_binary_str())
    }
}

// ===========================================================================================
// ========================== Formatting =====================================================
// ===========================================================================================

pub enum Formatting {
    /// How many digits to show after the decimal
    Decimal { keep_n_decimal_digits: usize },
    /// How many total digits to show. Symbols like `-` and `.` do not
    /// count towards digit count.
    /// If you have a decimal, `12345.678` and you format with `Digits { keep_n_digits: 6 }`
    /// the output will be `12345.6`.
    Digits { keep_n_digits: usize },
    /// Format as binary with separator and grouping.
    /// e.g., `1101011010101101010000011100101` using `Binary { separator: "x", grouping: 4 }` will
    /// output `1101x0110x1010x1101x0100x0001x1100x101`.
    /// e.g., `1101011010101101010000011100101` using Binary { separator: " ", grouping: 8 }` will
    /// output `11010110 10101101 01000001 1100101`.
    Binary { separator: String, grouping: usize },
}

impl Formatting {
    pub fn apply(&self, number: &Number) -> String {
        match self {
            Formatting::Decimal {
                keep_n_decimal_digits,
            } => Self::apply_decimal_digits_formatting(number, *keep_n_decimal_digits),
            Formatting::Binary {
                separator,
                grouping,
            } => Self::apply_binary_formatting(number, separator, *grouping),
            Formatting::Digits { keep_n_digits } => {
                Self::apply_digits_formatting(number, *keep_n_digits)
            }
        }
    }

    fn apply_decimal_digits_formatting(number: &Number, keep_n_decimal_digits: usize) -> String {
        let num_str = number.to_string();

        if number.is_int() {
            return num_str;
        }

        let (int_part, fract_part) = num_str.split_once('.').unwrap_or((&num_str, ""));

        if keep_n_decimal_digits == 0 {
            return int_part.to_string();
        }

        if fract_part.len() <= keep_n_decimal_digits {
            return num_str;
        }

        let truncated = &fract_part[..keep_n_decimal_digits];
        format!("{int_part}.{truncated}")
    }

    fn apply_digits_formatting(number: &Number, keep_n_digits: usize) -> String {
        let num_str = number.to_string();
        let mut fmted = String::new();
        let mut seen_digits = 0;

        for c in num_str.chars() {
            if seen_digits >= keep_n_digits {
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
    /// Accounts for negative binary strings ('-' at start of string).
    ///
    /// ```rust
    /// let bin_str = "1111111111";
    /// let separator = "_";
    /// let grouping = 8;
    ///
    /// let formatted = format_binary_str(bin_str, separator, grouping);
    /// assert_eq!(formatted, "00000011_11111111".to_string());
    /// ````
    fn format_binary_str(bin_str: &str, separator: &str, grouping: usize) -> String {
        if bin_str.is_empty() {
            return String::new();
        }

        let (sign, digits) = if let Some(stripped) = bin_str.strip_prefix('-') {
            ("-", stripped)
        } else {
            ("", bin_str)
        };

        let len = digits.len();
        let target_len = Formatting::next_pos_multiple_inclusive(grouping, len);
        let pad_by = target_len - len;

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
    ///
    /// ```rust
    /// // Get the next multiple of 3 starting from 34.
    /// let ans = next_pos_multiple(3, 34);
    /// assert_eq!(ans, 36);
    /// ````
    fn next_pos_multiple_inclusive(m: usize, n: usize) -> usize {
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
    fn next_pos_multiple(#[case] m: usize, #[case] n: usize, #[case] expect: usize) {
        let a = Formatting::next_pos_multiple_inclusive(m, n);
        assert_eq!(a, expect, "expected {expect} got {a}");
    }

    #[rstest]
    #[case::formatting_decimals1("-12345.6789", 2, "-12345.67")]
    #[case::formatting_decimals2("12345.6", 0, "12345")]
    #[case::formatting_decimals3("12345.6789", 20, "12345.6789")]
    fn formatting_decimals(
        #[case] number: &str,
        #[case] keep_n_decimal_digits: usize,
        #[case] expect: &str,
    ) {
        let n = number.parse::<Number>().unwrap();
        let decimals = n.format(Formatting::Decimal {
            keep_n_decimal_digits,
        });
        assert_eq!(
            decimals, expect,
            "expected decimals '{expect}' got decimals '{decimals}'"
        );
    }

    #[rstest]
    #[case::formatting_digits1("-12345.6789", 7, "-12345.67")]
    fn formatting_digits(#[case] number: &str, #[case] keep_n_digits: usize, #[case] expect: &str) {
        let n = number.parse::<Number>().expect("number str to Number");
        let digits = n.format(Formatting::Digits { keep_n_digits });
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
        #[case] grouping: usize,
        #[case] expect: &str,
    ) {
        let n = number.parse::<Number>().expect("number str to Number");
        let binary = n.format(Formatting::Binary {
            separator: separator.to_string(),
            grouping,
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
}
