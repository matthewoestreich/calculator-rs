//! # Formatting
//!
//! - We use the same spec as the cli, without the need to start with `:`.
//! - You need to provide a string to the `.format("..")`method using the following
//!   grammar and syntax as described in the docs found at the top of `src/lib.rs` or
//!   here [Formatting Help](https://docs.rs/calcinum/latest/calcinum/index.html#cli-formatting).
//!
//! # Spec
//!
//! At a high level:
//!
//! ```text
//! 0999b8
//! | | ||
//! | | |+--   (8) GROUPING : Provide a number and we will group your output by `N` characters.
//! | | +---   (b) KIND : This is the format you want, e.g., binary, hex, base64, etc ...
//! | +----- (999) WIDTH : How many characters do you want your output to be.
//! +-------   (0) ZERO PAD : Do you want us to pad width with 0's? If not provided we pad with spaces.
//! ```
//!
//! # Examples
//!
//! ```rust
//! use calcinum::Number;
//!
//! let n = Number::from(123);
//!
//! // Format number as binary.
//! n.format("b"); // "1111011"
//!
//! // Format number as binary with a width of 12, non zero padded.
//! n.format("12b"); // "     1111011"
//!
//! // Format number as binary with a width o 12, zero padded.
//! n.format("012b"); // "000001111011"
//!
//! // Format number as binary width a width of 12, zero padded, groups of 4.
//! n.format("012b4"); // "0000 0111 1011"
//! ```
//!

use crate::Number;
use std::fmt;
use varienum::VariantsVec;

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
        match spec.kind {
            Kind::Number => Ok(number.to_string().parse::<Number>()?.to_string()),
            Kind::Binary => Ok(Self::fmt_binary(&number.to_binary_str(), &spec)),
            Kind::HexadecimalLower => Ok(Self::fmt_hex(&number.to_hexadecimal_str(false), &spec)),
            Kind::HexadecimalUpper => Ok(Self::fmt_hex(&number.to_hexadecimal_str(true), &spec)),
            Kind::Base64 => Ok(number.to_base64_str()),
            Kind::Null => Err(format!("unrecognized type '{:?}'", spec.kind)),
        }
    }

    fn fmt_hex(hex_str: &str, spec: &FormatSpec) -> String {
        Self::fmt_binary(hex_str, spec)
    }

    fn fmt_binary(num_str: &str, spec: &FormatSpec) -> String {
        let (sign, s) = match num_str.strip_prefix('-') {
            Some(rest) => ("-", rest),
            None => ("", num_str),
        };

        let width = spec.width.unwrap_or(0);
        let group = spec.group.unwrap_or(0);
        let pad_char = if spec.zero_pad { "0" } else { " " };
        let (lhs, rhs) = s.split_once('.').unwrap_or((s, ""));

        let mut lhs_out = String::new();
        let mut rhs_out = String::new();

        let total_len = lhs.len() + rhs.len();
        let target_width = width.max(total_len);
        let mut pad = target_width.saturating_sub(total_len);

        if group > 0 {
            let rhs_fmtd_len = Self::next_multiple(group, rhs.len());
            let lhs_fmtd_len = Self::next_multiple(group, lhs.len());
            let total_fmt_len = lhs_fmtd_len + rhs_fmtd_len;
            pad = target_width.saturating_sub(total_fmt_len);
            let expected_output_len = pad + lhs.len() + rhs_fmtd_len;
            let extra_pad = target_width.saturating_sub(expected_output_len);
            pad += extra_pad;
        }

        lhs_out.push_str(&pad_char.repeat(pad));
        lhs_out.push_str(lhs);
        rhs_out.push_str(rhs);

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
        println!("  [group_by] s.len= '{}' | n= '{n}'", s.len());
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

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;
    use std::mem;

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
}
