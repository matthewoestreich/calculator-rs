use core::fmt;
use std::{
    cmp::Ordering,
    ops::{Div, DivAssign, Rem},
    str::FromStr,
};

use astro_float::{BigFloat, RoundingMode};
use num_bigint::BigInt;

use crate::ValueError;

#[derive(Debug, Clone)]
pub enum Number {
    Int(BigInt),
    Float(BigFloat),
}

impl Number {
    pub fn to_float(&self) -> Self {
        if let Self::Int(n) = self {
            let bf = BigFloat::from_str(&n.to_string()).expect("[to_float] BigInt -> BigFloat");
            return Self::Float(bf);
        }
        self.clone()
    }

    pub(crate) fn order(&self) -> NumberOrder {
        NumberOrder::from(self)
    }

    pub(crate) fn match_order(&mut self, other: &mut Self) {
        match self.order().cmp(&other.order()) {
            Ordering::Less => self.promote(),
            Ordering::Greater => other.promote(),
            Ordering::Equal => {}
        }
    }

    pub(crate) fn promote(&mut self) {
        if let Self::Int(n) = self {
            let bf = BigFloat::from_str(&n.to_string()).expect("[promote] BigInt -> BigFloat");
            *self = Self::Float(bf);
        }
    }
}

// ===========================================================================================
// ========================== From ===========================================================
// ===========================================================================================

macro_rules! impl_number_from {
    ($t:ty => $variant:ident => $big_kind:ident) => {
        impl From<$t> for Number {
            fn from(value: $t) -> Self {
                Number::$variant($big_kind::from(value))
            }
        }
    };
}

impl_number_from!(u8 => Int => BigInt);
impl_number_from!(u16 => Int => BigInt);
impl_number_from!(u32 => Int => BigInt);
impl_number_from!(u64 => Int => BigInt);
impl_number_from!(u128 => Int => BigInt);
impl_number_from!(i8 => Int => BigInt);
impl_number_from!(i16 => Int => BigInt);
impl_number_from!(i32 => Int => BigInt);
impl_number_from!(i64 => Int => BigInt);
impl_number_from!(i128 => Int => BigInt);
impl_number_from!(f64 => Float => BigFloat);

// ===========================================================================================
// ========================== FromStr ========================================================
// ===========================================================================================

impl FromStr for Number {
    type Err = ValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(".") {
            return s
                .parse::<BigFloat>()
                .map(Self::Float)
                .map_err(|_| ValueError::Parsing {
                    value: s.to_string(),
                });
        }
        s.parse::<BigInt>()
            .map(Self::Int)
            .map_err(|_| ValueError::Parsing {
                value: s.to_string(),
            })
    }
}

// ===========================================================================================
// ========================== Display ========================================================
// ===========================================================================================

fn scientific_to_decimal(s: &str) -> String {
    if let Some((mantissa, exp)) = s.split_once('e') {
        let exp: i32 = exp.parse().unwrap();
        let mut digits = mantissa.replace('.', "");
        let decimal_pos = mantissa.find('.').unwrap_or(digits.len()) as i32;
        let new_pos = decimal_pos + exp;
        if new_pos <= 0 {
            format!("0.{}{}", "0".repeat(-new_pos as usize), digits)
        } else if new_pos as usize >= digits.len() {
            format!("{}{}", digits, "0".repeat(new_pos as usize - digits.len()))
        } else {
            digits.insert(new_pos as usize, '.');
            digits
        }
    } else {
        s.to_string()
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Int(big_int) => write!(f, "{big_int}"),
            Number::Float(big_float) => {
                write!(f, "{}", scientific_to_decimal(&big_float.to_string()))
            }
        }
    }
}

// ===========================================================================================
// ========================== Div ============================================================
// ===========================================================================================

#[allow(clippy::clone_on_copy)]
impl<Rhs> DivAssign<Rhs> for Number
where
    Rhs: Into<Number>,
{
    fn div_assign(&mut self, rhs: Rhs) {
        let mut rhs = rhs.into();
        self.match_order(&mut rhs);

        *self = match (&self, &rhs) {
            (Number::Float(x), Number::Float(y)) => {
                Number::Float(x.div(y, 4096, RoundingMode::None))
            }
            (Number::Int(x), Number::Int(y)) => {
                if x % y == BigInt::ZERO {
                    Number::Int(x / y)
                } else {
                    let l = BigFloat::from_str(&x.to_string()).expect("[div] BigFloat from BigInt");
                    let r = BigFloat::from_str(&y.to_string()).expect("[div] BigFloat from BigInt");
                    println!("l={l} | r={r}");
                    let result = Number::Float(l.div(&r, 4096, RoundingMode::None));
                    println!("    result = {result}");
                    result
                }
            }
            _ => unreachable!("we know orders match"),
        }
    }
}

impl DivAssign<&Number> for Number {
    fn div_assign(&mut self, rhs: &Number) {
        *self = &*self / rhs;
    }
}

impl<Rhs> Div<Rhs> for Number
where
    Rhs: Into<Number>,
{
    type Output = Number;

    fn div(mut self, rhs: Rhs) -> Self::Output {
        self.div_assign(rhs);
        self
    }
}

impl<'a> Div<&'a Number> for &Number {
    type Output = Number;

    fn div(self, rhs: &'a Number) -> Number {
        let mut lhs = self.clone();
        lhs /= rhs.clone();
        lhs
    }
}

// ===========================================================================================
// ========================== NumberOrder ====================================================
// ===========================================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NumberOrder {
    Int,
    Float,
}

impl From<Number> for NumberOrder {
    fn from(value: Number) -> Self {
        match value {
            Number::Int(_) => Self::Int,
            Number::Float(_) => Self::Float,
        }
    }
}

impl From<&Number> for NumberOrder {
    fn from(value: &Number) -> Self {
        match value {
            Number::Int(_) => Self::Int,
            Number::Float(_) => Self::Float,
        }
    }
}

// ===========================================================================================
// ========================== Tests ==========================================================
// ===========================================================================================

#[cfg(test)]
mod test {
    use super::*;
    use astro_float::Consts;
    use astro_float::Radix;
    use astro_float::RoundingMode;

    #[test]
    fn astrofoo() {
        let mut cc = Consts::new().unwrap();
        let bf = BigFloat::parse("0.5", Radix::Dec, 128, RoundingMode::ToEven, &mut cc);
        println!("{}", scientific_to_decimal(&bf.to_string()));
    }

    #[test]
    fn numberfoo() {
        let x = Number::Int(1.into());
        let y = Number::Int(2.into());
        let expected = Number::Float(0.5.into());
        let r = x / y;
        println!("result = {r}");
        //assert_eq!(r, expected, "expected {expected:?} got {r:?}");
    }
}
