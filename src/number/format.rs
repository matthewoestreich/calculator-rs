use crate::Number;
use bigdecimal::BigDecimal;
use std::fmt;

/// Converts scientific notation to standard notation.
fn bd_fmt_standard(bd: &BigDecimal) -> String {
    if bd.is_integer() {
        format!("{bd:.0}")
    } else {
        let (_, scale) = bd.as_bigint_and_scale();
        format!("{bd:.*}", (scale as usize).max(0))
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Int(i) => write!(f, "{i}"),
            Number::Decimal(d) => write!(f, "{}", bd_fmt_standard(d)),
        }
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "Number::Int({i})"),
            Self::Decimal(d) => write!(f, "Number::Decimal({})", bd_fmt_standard(d)),
        }
    }
}

impl fmt::Binary for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_binary_str())
    }
}

#[allow(dead_code)]
pub(crate) fn scientific_to_decimal(s: &str) -> String {
    if !s.contains("e") && !s.contains("E") {
        return s.to_string();
    }

    // split into coefficient and exponent
    let (coeff, exp) = s
        .split_once('e')
        .or_else(|| s.split_once('E'))
        .expect("invalid scientific notation");
    let exp: i64 = exp.parse().expect("invalid exponent");
    // split coefficient into integer + fractional parts
    let mut parts = coeff.split('.');
    let int_part = parts.next().unwrap();
    let frac_part = parts.next().unwrap_or("");
    let mut digits = format!("{int_part}{frac_part}");
    let decimal_pos = int_part.len() as i64;
    let new_pos = decimal_pos + exp;

    if new_pos <= 0 {
        // decimal goes before all digits
        let zeros = "0".repeat((-new_pos) as usize);
        format!("0.{zeros}{digits}")
    } else if new_pos >= digits.len() as i64 {
        // decimal goes after all digits
        let zeros = "0".repeat((new_pos - digits.len() as i64) as usize);
        digits.push_str(&zeros);
        digits
    } else {
        // decimal goes somewhere in the middle
        let pos = new_pos as usize;
        digits.insert(pos, '.');
        digits
    }
}
