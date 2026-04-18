use calcinum::Number;

#[derive(Debug)]
pub enum State {
    Start,
    Width,
    ZeroPad,
    Kind,
    Group,
}

#[derive(Default, Debug)]
pub struct FormatSpec {
    zero_pad: bool,
    width: Option<usize>,
    // the only part of the spec that is required.
    kind: char,
    group: Option<usize>,
}

impl FormatSpec {
    pub fn parse(spec: &str) -> Result<FormatSpec, String> {
        let mut zero_pad = false;
        let mut width = String::new();
        let mut group = String::new();
        let mut kind = None;

        let mut state = State::Start;

        for c in spec.chars() {
            match state {
                State::Start => match c {
                    '0' => {
                        zero_pad = true;
                        state = State::ZeroPad;
                    }
                    c if c.is_ascii_digit() => {
                        width.push(c);
                        state = State::Width;
                    }
                    c if c.is_ascii_alphabetic() => {
                        kind = Some(c);
                        state = State::Kind;
                    }
                    _ => return Err(format!("unexpected char '{c}' in Start")),
                },
                State::ZeroPad => match c {
                    c if c.is_ascii_digit() => {
                        width.push(c);
                        state = State::Width;
                    }
                    c if c.is_alphabetic() => {
                        kind = Some(c);
                        state = State::Kind;
                    }
                    _ => return Err(format!("unexpected char '{c}' after Start")),
                },
                State::Width => match c {
                    c if c.is_ascii_digit() => {
                        width.push(c);
                        state = State::Width;
                    }
                    c if c.is_ascii_alphabetic() => {
                        kind = Some(c);
                        state = State::Kind;
                    }
                    _ => return Err(format!("unexxpected char '{c}' in Width")),
                },
                State::Kind => match c {
                    c if c.is_ascii_digit() => {
                        group.push(c);
                        state = State::Group;
                    }
                    _ => return Err(format!("unexpected char '{c}' in Kind")),
                },
                State::Group => match c {
                    c if c.is_ascii_digit() => group.push(c),
                    _ => return Err(format!("unexpected char '{c}' in Group")),
                },
            }
        }

        let Some(kind) = kind else {
            return Err("kind is required!".to_string());
        };

        let width = if width.is_empty() {
            None
        } else {
            Some(width.parse().unwrap())
        };
        let group = if group.is_empty() {
            None
        } else {
            Some(group.parse().unwrap())
        };

        Ok(Self {
            zero_pad,
            width,
            kind,
            group,
        })
    }
}

pub struct Formatter;

impl Formatter {
    pub fn format_number(number: &Number, spec: FormatSpec) -> Result<String, String> {
        let num_str = match spec.kind {
            'b' => number.to_binary_str(),
            c if c == 'x' || c == 'X' => number.to_hexadecimal_str(c == 'X'),
            'B' => number.to_base64_str(),
            _ => return Err(format!("unrecognized type '{}'", spec.kind)),
        };

        let mut group_pad = 0;
        let mut width_pad = 0;

        if let Some(w) = spec.width {
            width_pad += w.saturating_sub(num_str.len());
        }
        if let Some(group) = spec.group {
            let min_len = num_str.len() + width_pad;
            group_pad = Self::next_multiple(group, min_len) - min_len;
        }

        let mut num_fmtd = String::with_capacity(width_pad + group_pad);
        let pad_char = if spec.zero_pad { '0' } else { ' ' };

        for _ in 0..width_pad {
            num_fmtd.push(pad_char);
        }
        for _ in 0..group_pad {
            num_fmtd.push('0');
        }

        // Now we have padding in our 'formatted' string,
        // push our converted Number string into it.
        num_fmtd.push_str(&num_str);

        if let Some(group) = spec.group {
            let mut s = String::with_capacity(num_fmtd.len());
            for (i, c) in num_fmtd.chars().enumerate() {
                if i != 0 && i % group == 0 {
                    s.push(' ');
                }
                s.push(c);
            }
            num_fmtd = s;
        }

        Ok(num_fmtd)
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

    #[rstest]
    #[case::fmt_spec1("8b4", false, Some(8), 'b', Some(4))]
    #[case::fmt_spec2("0x4", true, None, 'x', Some(4))]
    #[case::fmt_spec3("16x2", false, Some(16), 'x', Some(2))]
    #[case::fmt_spec4("b", false, None, 'b', None)]
    #[case::fmt_spec5("08b4", true, Some(8), 'b', Some(4))]
    #[case::fmt_spec6("0b", true, None, 'b', None)]
    fn format_spec(
        #[case] spec_str: &str,
        #[case] expected_zero_pad: bool,
        #[case] expected_width: Option<usize>,
        #[case] expected_kind: char,
        #[case] expected_group: Option<usize>,
    ) {
        let parsed = FormatSpec::parse(spec_str).unwrap();
        assert_eq!(parsed.zero_pad, expected_zero_pad);
        assert_eq!(parsed.width, expected_width);
        assert_eq!(parsed.kind, expected_kind);
        assert_eq!(parsed.group, expected_group);
    }
}
