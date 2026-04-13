use crate::Number;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use std::fmt;

impl Number {
    pub fn format(&self, spec: &str) -> String {
        let spec = FormatSpec::parse(spec);

        match self {
            Number::Int(n) => format_int(n, &spec),
            Number::Decimal(d) => format_decimal(d, &spec),
        }
    }

    pub(crate) fn is_binary(s: &str) -> bool {
        // If we got a binary str in scientific notation
        let s = Number::expand_scientific(s);
        s.chars().all(|c| c == '0' || c == '1')
    }

    /// Expands a scientific notation to "regular" format. Works for any base.
    /// Returns a tuple of the formatted String along with a bool. The bool is
    /// true if we expanded, and false if we didn't.
    /// For example; if the input is `1.01e+1` the output is `("10.10", true)`
    /// If the input is `10` the output is `("10", false)`.
    pub(crate) fn expand_scientific(s: &str) -> String {
        let (mantissa, exp) = s
            .split_once('e')
            .or_else(|| s.split_once('E'))
            .unwrap_or((s, ""));

        if exp.is_empty() {
            return s.to_string();
        }

        let exp: isize = exp.parse().unwrap_or(0);
        let mut parts = mantissa.split('.');
        let int_part = parts.next().unwrap_or("0");
        let frac_part = parts.next().unwrap_or("");
        let mut digits: String = format!("{}{}", int_part, frac_part);
        let point_pos = int_part.len() as isize;
        let new_pos = point_pos + exp;

        if new_pos <= 0 {
            let mut result = "0.".to_string();
            result.push_str(&"0".repeat((-new_pos) as usize));
            result.push_str(&digits);
            return result;
        }
        if new_pos as usize >= digits.len() {
            digits.push_str(&"0".repeat(new_pos as usize - digits.len()));
            return digits;
        }

        digits.insert(new_pos as usize, '.');
        digits
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Int(i) => write!(f, "{i}"),
            Number::Decimal(d) => write!(f, "{}", Number::expand_scientific(&d.to_string())),
        }
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "Number::Int({i})"),
            Self::Decimal(d) => write!(
                f,
                "Number::Decimal({})",
                Number::expand_scientific(&d.to_string())
            ),
        }
    }
}

impl fmt::Binary for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Int(i) => write!(f, "{i:b}"),
            Number::Decimal(d) => {
                let bf = d.to_string().parse::<astro_float::BigFloat>().unwrap();
                write!(f, "{bf:b}")
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct FormatSpec {
    pub width: Option<usize>,
    pub precision: Option<usize>,
    pub zero_pad: bool,
    pub show_sign: bool,
    pub base: Option<char>, // 'b', 'o', 'x'
    pub scientific: bool,
}

impl FormatSpec {
    pub fn parse(spec: &str) -> Self {
        let mut fs = FormatSpec::default();
        let mut chars = spec.chars().peekable();

        // sign
        if matches!(chars.peek(), Some('+')) {
            fs.show_sign = true;
            chars.next();
        }

        // zero padding
        if matches!(chars.peek(), Some('0')) {
            fs.zero_pad = true;
            chars.next();
        }

        // width
        let mut width = String::new();
        while matches!(chars.peek(), Some(c) if c.is_ascii_digit()) {
            width.push(chars.next().unwrap());
        }
        if !width.is_empty() {
            fs.width = width.parse().ok();
        }

        // precision
        if matches!(chars.peek(), Some('.')) {
            chars.next();
            let mut prec = String::new();
            while matches!(chars.peek(), Some(c) if c.is_ascii_digit()) {
                prec.push(chars.next().unwrap());
            }
            if !prec.is_empty() {
                fs.precision = prec.parse().ok();
            }
        }

        // type
        if let Some(c) = chars.next() {
            match c {
                'b' | 'o' | 'x' => fs.base = Some(c),
                'e' => fs.scientific = true,
                _ => {}
            }
        }

        fs
    }
}

fn format_int(n: &BigInt, spec: &FormatSpec) -> String {
    let mut s = match spec.base {
        Some('b') => format!("{:b}", n),
        Some('o') => format!("{:o}", n),
        Some('x') => format!("{:x}", n),
        _ => n.to_string(),
    };

    if spec.show_sign && !s.starts_with('-') {
        s = format!("+{}", s);
    }

    if let Some(width) = spec.width {
        if spec.zero_pad {
            s = format!("{:0>width$}", s, width = width);
        } else {
            s = format!("{:>width$}", s, width = width);
        }
    }

    s
}

fn format_decimal(d: &BigDecimal, spec: &FormatSpec) -> String {
    let mut s = if let Some(p) = spec.precision {
        d.round(p as i64).to_string()
    } else {
        d.to_string()
    };

    if spec.show_sign && !s.starts_with('-') {
        s = format!("+{}", s);
    }

    if let Some(width) = spec.width {
        if spec.zero_pad {
            s = format!("{:0>width$}", s, width = width);
        } else {
            s = format!("{:>width$}", s, width = width);
        }
    }

    s
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::number::ASTRO_CONSTS;
    use astro_float::{BigFloat, Radix as AstroRadix, RoundingMode as AstroRoundingMode};
    use rstest::*;
    use std::str::FromStr;

    #[test]
    fn foofoo() {
        let n = "352.81".parse::<Number>().unwrap();
        println!(
            "original = {n:?}\n\nbinary scientific = {n:b}\n\npure binary = {}",
            Number::expand_scientific(&format!("{n:b}"))
        );
        /*
        // Works : "1111011" = 123
        let xx = "1111011".parse::<Number>().unwrap();
        println!("should be '123' = {xx:?}");

        // Works
        let og_dec = "17958432089245743489.3597843208120587934";
        let og_dec_as_bin_str = "1.11110010011100100101010110101100101101001110111111110011000000101011100000110101101001101000011100000000101100101101111100010000101001010011000100011010111010010011101011101101010001001001e+111111";
        let n = og_dec_as_bin_str.parse::<Number>().unwrap();
        println!("should be = {og_dec} = {n:?}");

        // Doesn't work
        let nint = "123".parse::<Number>().unwrap();
        println!("shuld be '123' = {nint}");

        // Doesn't work
        let ndec = "123.123".parse::<Number>().unwrap();
        println!("should be 123.123 = {ndec}");

        let bf = BigFloat::from_str(
            "17958432089245413415653413453514743489.359784321343545243523414341353108120587934",
        )
        .unwrap();
        println!("\n\n{}", Number::expand_scientific(&bf.to_string()));
        println!("\n{bf}\n{}", Number::expand_scientific(&bf.to_string()));
        println!("{}", Number::expand_scientific("1123"));
        */

        /*
        ASTRO_CONSTS.with(|cc| {
            let bin_str = "1.11110010011100100101010110101100101101001110111111110011000000101011100000110101101001101000011100000000101100101101111100010000101001010011000100011010111010010011101011101101010001001001e+111111";
            let bf = BigFloat::parse(bin_str, astro_float::Radix::Bin, 128, astro_float::RoundingMode::None, &mut cc.borrow_mut());
            println!("{}", scientific_to_de:cimal(&bf.to_string()));
        })
        */
    }

    #[test]
    fn expand_scientific_bigdecimal() {
        let dec = "0.003267763643053385473474126528521658090820522829134405715571960013412761458430221895992568099832513317";
        let dec_sci = "3.267763643053385473474126528521658090820522829134405715571960013412761458430221895992568099832513317e-3";
        let bd = dec_sci.parse::<BigDecimal>().unwrap();
        let bd_exp = Number::expand_scientific(&bd.to_string());
        assert_eq!(dec, bd_exp, "expected '{dec}' got '{bd_exp}'");
    }

    #[rstest]
    #[case::binary_str1(
        "17958432089245743489.3597843208120587934",
        "1.11110010011100100101010110101100101101001110111111110011000000101011100000110101101001101000011100000000101100101101111100010000101001010011000100011010111010010011101011101101010001001001e+111111"
    )]
    #[case::binary_str_bigdecimal_neg(
        "-17958432089245743489.3597843208120587934",
        "-1.11110010011100100101010110101100101101001110111111110011000000101011100000110101101001101000011100000000101100101101111100010000101001010011000100011010111010010011101011101101010001001001e+111111"
    )]
    #[case::binary_str2(
        "17958432089245743489",
        "1111100100111001001010101101011001011010011101111111100110000001"
    )]
    #[case::binary_str_bigint_neg(
        "-17958432089245743489",
        "-1111100100111001001010101101011001011010011101111111100110000001"
    )]
    fn binary_str(#[case] number: &str, #[case] expect: &str) {
        let n = number.parse::<Number>().unwrap();
        let fr = format!("{n:b}");
        assert_eq!(
            expect, fr,
            "[format!(\"{n:b}\")] expected '{expect}' got '{fr}'"
        );
    }
}
