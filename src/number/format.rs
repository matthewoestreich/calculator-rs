//
// See [here](https://docs.rs/calcinum/latest/calcinum/index.html#library-usage) for more info.
//

use crate::{Number, number::conversion};
use std::fmt;
use varienum::VariantsVec;

impl Number {
    /// Applies custom formatting logic.
    /// See [`Formatting`](crate#formatting) for more examples.
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

                match conversion::decimal_str_to_binary_str(int_part) {
                    Some(int_part_bin) => match conversion::decimal_str_to_binary_str(fract_part) {
                        Some(fract_part_bin) => write!(f, "{int_part_bin}.{fract_part_bin}"),
                        None => write!(f, "{int_part_bin}"),
                    },
                    None => write!(f, "{d}"),
                }
            }
        }
    }
}

impl fmt::Octal for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Number::Int(i) => i.to_string(),
            Number::Decimal(d) => d.to_string(),
        };
        match conversion::decimal_str_to_octal_str(&s) {
            Ok(ds) => write!(f, "{ds}"),
            Err(_) => write!(f, "{s}"),
        }
    }
}

impl fmt::LowerHex for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Int(i) => fmt::LowerHex::fmt(i, f),
            Number::Decimal(d) => {
                let ds = d.to_plain_string();
                match conversion::decimal_str_to_hexadecimal_str(&ds, false) {
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
                match conversion::decimal_str_to_hexadecimal_str(&ds, true) {
                    Ok(s) => write!(f, "{s}"),
                    Err(_) => write!(f, "{ds}"),
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum State {
    Start,
    Width,
    ZeroPad,
    Scale,
    Kind,
    Group,
}

#[derive(Debug, Default, PartialEq, Eq, VariantsVec)]
pub enum Kind {
    #[description = "b (binary)"]
    Binary,
    #[description = "X (hex upper)"]
    HexadecimalUpper,
    #[description = "x (hex lower"]
    HexadecimalLower,
    #[description = "B (base64)"]
    Base64,
    #[description = "N (Number)"]
    Number,
    #[default]
    Null,
}

impl Kind {
    pub fn is_null(&self) -> bool {
        matches!(self, Kind::Null)
    }
}

impl From<char> for Kind {
    fn from(c: char) -> Self {
        match c {
            'b' => Self::Binary,
            'X' => Self::HexadecimalUpper,
            'x' => Self::HexadecimalLower,
            'B' => Self::Base64,
            'N' => Self::Number,
            _ => Self::Null,
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Kind::Binary => write!(f, "b"),
            Kind::HexadecimalUpper => write!(f, "X"),
            Kind::HexadecimalLower => write!(f, "x"),
            Kind::Base64 => write!(f, "B"),
            Kind::Number => write!(f, "N"),
            Kind::Null => write!(f, ""),
        }
    }
}

#[derive(Default, Debug)]
pub struct FormatSpec {
    zero_pad: bool,
    width: Option<usize>,
    #[allow(dead_code)]
    scale: Option<usize>,
    // the only part of the spec that is required.
    kind: Kind,
    group: Option<usize>,
}

impl FormatSpec {
    pub fn parse(spec: &str) -> Result<FormatSpec, String> {
        let mut zero_pad = false;
        let mut width = String::new();
        let mut group = String::new();
        let mut scale = String::new();
        let mut kind = Kind::Null;
        let mut state = State::Start;

        for c in spec.chars() {
            match state {
                State::Start => {
                    if c == '0' {
                        zero_pad = true;
                        state = State::ZeroPad;
                    } else if c.is_ascii_digit() {
                        width.push(c);
                        state = State::Width;
                    } else if c.is_ascii_alphabetic() {
                        kind = Kind::from(c);
                        state = State::Kind;
                    } else {
                        return Err(format!("unexpected char '{c}' in Start"));
                    }
                }
                State::ZeroPad => {
                    if c.is_ascii_digit() {
                        width.push(c);
                        state = State::Width;
                    } else if c.is_alphabetic() {
                        kind = Kind::from(c);
                        state = State::Kind;
                    } else {
                        return Err(format!("unexpected char '{c}' after Start"));
                    }
                }
                State::Width => {
                    if c == '.' {
                        state = State::Scale;
                    } else if c.is_ascii_digit() {
                        width.push(c);
                        state = State::Width;
                    } else if c.is_ascii_alphabetic() {
                        kind = Kind::from(c);
                        state = State::Kind;
                    } else {
                        return Err(format!("unexxpected char '{c}' in Width"));
                    }
                }
                State::Kind => {
                    if c.is_ascii_digit() {
                        group.push(c);
                        state = State::Group;
                    } else {
                        return Err(format!("unexpected char '{c}' in Kind"));
                    }
                }
                State::Scale => {
                    if c.is_ascii_digit() {
                        scale.push(c);
                    } else {
                        return Err(format!("unexpect char '{c}' in Scale"));
                    }
                }
                State::Group => {
                    if c.is_ascii_digit() {
                        group.push(c);
                    } else {
                        return Err(format!("unexpected char '{c}' in Group"));
                    }
                }
            }
        }

        if kind.is_null() {
            return Err("kind is required!".to_string());
        };

        let width = if width.is_empty() {
            None
        } else {
            width
                .parse()
                .map(Some)
                .map_err(|e| format!("unable to parse width : {e:?}"))?
        };

        let group = if group.is_empty() {
            None
        } else {
            group
                .parse()
                .map(Some)
                .map_err(|e| format!("unable to parse group size : {e:?}"))?
        };

        let scale = if scale.is_empty() {
            None
        } else {
            scale
                .parse()
                .map(Some)
                .map_err(|e| format!("unable to parse scale : {e:?} "))?
        };

        Ok(Self {
            zero_pad,
            width,
            scale,
            kind,
            group,
        })
    }
}

pub struct Formatter;

impl Formatter {
    pub fn format_number(number: &Number, spec: FormatSpec) -> Result<String, String> {
        Ok(match spec.kind {
            Kind::Number => number.to_string().parse::<Number>()?.to_string(),
            Kind::Binary => Self::fmt_radix(&number.to_binary_str(), &spec),
            Kind::HexadecimalLower => Self::fmt_radix(&number.to_hexadecimal_str(false), &spec),
            Kind::HexadecimalUpper => Self::fmt_radix(&number.to_hexadecimal_str(true), &spec),
            Kind::Base64 => number.to_base64_str(),
            Kind::Null => return Err(format!("unrecognized type '{:?}'", spec.kind)),
        })
    }

    /// This method formats per the spec.
    /// While you can use it on an encoded base64 string, it would affect the decoded value.
    fn fmt_radix(num_str: &str, spec: &FormatSpec) -> String {
        let (sign, s) = match num_str.strip_prefix('-') {
            Some(rest) => ("-", rest),
            None => ("", num_str),
        };

        let width = spec.width.unwrap_or(0);
        let group = spec.group.unwrap_or(0);
        let pad_char = if spec.zero_pad { "0" } else { " " };
        let (lhs, rhs) = s.split_once('.').unwrap_or((s, ""));
        let total_len = lhs.len() + rhs.len();
        let target_width = width.max(total_len);

        let pad = if group > 0 {
            let rhs_fmtd_len = Self::next_multiple(group, rhs.len());
            target_width.saturating_sub(lhs.len() + rhs_fmtd_len)
        } else {
            target_width.saturating_sub(total_len)
        };

        let mut lhs_out = pad_char.repeat(pad);
        let mut rhs_out = rhs.to_string();
        lhs_out.push_str(lhs);

        if group > 0 {
            lhs_out = Self::group_by(&lhs_out, group);
            if !rhs.is_empty() {
                rhs_out = Self::group_by(&rhs_out, group);
            }
        }

        let mut output = format!("{sign}{lhs_out}");
        if !rhs.is_empty() {
            output.push_str(&format!(".{rhs_out}"));
        }

        output
    }

    fn group_by(s: &str, n: usize) -> String {
        let total = Self::next_multiple(n, s.len());
        let pad = total - s.len();
        let sbytes = s.as_bytes();
        let mut output = String::new();

        for i in 0..total {
            if i != 0 && i % n == 0 {
                output.push(' ');
            }
            if i < pad {
                output.push('0');
            } else {
                output.push(sbytes[i - pad] as char);
            }
        }

        output
    }

    /// Finds the next multiple, `m`,  starting at `n`.
    /// If `n` is already a multiple of `m`, we return `n`.
    /// If `m` or `n` are 0, we return 0.
    fn next_multiple(m: usize, n: usize) -> usize {
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
    use std::mem;

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

    #[rstest]
    #[case::fmt_spec1("8b4", false, Some(8), Kind::Binary, Some(4))]
    #[case::fmt_spec2("0x4", true, None, Kind::HexadecimalLower, Some(4))]
    #[case::fmt_spec3("16x2", false, Some(16), Kind::HexadecimalLower, Some(2))]
    #[case::fmt_spec4("b", false, None, Kind::Binary, None)]
    #[case::fmt_spec5("08b4", true, Some(8), Kind::Binary, Some(4))]
    #[case::fmt_spec6("0b", true, None, Kind::Binary, None)]
    #[should_panic]
    #[case::fmt_spec7("z", true, None, Kind::Null, None)]
    fn format_spec(
        #[case] spec_str: &str,
        #[case] expected_zero_pad: bool,
        #[case] expected_width: Option<usize>,
        #[case] expected_kind: Kind,
        #[case] expected_group: Option<usize>,
    ) {
        let parsed = FormatSpec::parse(spec_str).unwrap();
        assert_eq!(parsed.zero_pad, expected_zero_pad);
        assert_eq!(parsed.width, expected_width);
        assert_eq!(parsed.group, expected_group);
        assert_eq!(
            mem::discriminant(&parsed.kind),
            mem::discriminant(&expected_kind),
            "expected kind '{expected_kind:?}' got kind '{:?}'",
            parsed.kind
        );
    }

    #[rstest]
    #[case::octal1("123.123", "173.173")]
    #[case::octal2("-123.123", "-173.173")]
    fn to_octal(#[case] number: &str, #[case] expect: &str) {
        let n = number.parse::<Number>().expect("Number");
        let r = n.to_octal_str();
        assert_eq!(&r, expect, "expected octal '{expect}' got octal '{r}'");
    }
}
