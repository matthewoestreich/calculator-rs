pub mod error;
pub mod format;
pub mod predicate;

#[macro_use]
mod macros;
mod arithmetic;
mod bitwise;
mod comparison;
mod conversion;
mod digit;
mod numeric;

use astro_float::Consts;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use std::{cell::RefCell, cmp::Ordering};

/// Arbitrary-precision numeric system supporting integers, decimals, and binary-aware arithmetic.
#[derive(Clone)]
pub enum Number {
    Int(BigInt),
    Decimal(BigDecimal),
}

/// Implementors know how to become a `Number`.
pub trait ToNumber {
    fn to_number(&self) -> Number;
}

thread_local! {
    static ASTRO_CONSTS: RefCell<Consts> = RefCell::new(Consts::new().expect("astro-float consts"));
}

impl Number {
    pub const ZERO: Self = Self::Int(BigInt::ZERO);

    /// Sets the scale only on Number::Decimal
    pub fn set_scale(&mut self, scale: i64) {
        if let Self::Decimal(n) = self {
            *n = n.with_scale(scale);
        }
    }

    /// Sets the scale and rounding mode only on Number::Decimal
    pub fn set_scale_round(&mut self, scale: i64, rounding_mode: bigdecimal::RoundingMode) {
        if let Self::Decimal(n) = self {
            *n = n.with_scale_round(scale, rounding_mode);
        }
    }

    pub fn order(&self) -> NumberOrder {
        NumberOrder::from(self)
    }

    pub fn match_order(&mut self, other: &mut Self) {
        match self.order().cmp(&other.order()) {
            Ordering::Less => self.promote(),
            Ordering::Greater => other.promote(),
            Ordering::Equal => {}
        }
    }

    /// Converts Number::Int to Number::Decimal.
    /// Number::Decimal is already the highest 'order'.
    pub fn promote(&mut self) {
        if let Number::Int(this) = self {
            *self = Number::Decimal(BigDecimal::from_bigint(this.to_owned(), 0));
        }
    }

    /// Converts `Number::Decimal` to `Number::Int`.
    /// IMPORTANT : this may cause loss of data/precision!
    pub fn demote(&mut self) {
        if let Number::Decimal(this) = self {
            let (i, _) = this.with_scale(0).into_bigint_and_scale();
            *self = Number::Int(i);
        }
    }
}

// ===========================================================================================
// ========================== NumberOrder ====================================================
// ===========================================================================================

/// The order, or rank, of `Number` variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NumberOrder {
    Int,
    Decimal,
}

impl From<Number> for NumberOrder {
    fn from(value: Number) -> Self {
        match value {
            Number::Int(_) => Self::Int,
            Number::Decimal(_) => Self::Decimal,
        }
    }
}

impl From<&Number> for NumberOrder {
    fn from(value: &Number) -> Self {
        match value {
            Number::Int(_) => Self::Int,
            Number::Decimal(_) => Self::Decimal,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(dead_code)]
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

    #[test]
    fn demote_dec() {
        let mut dec = "123.123".parse::<Number>().expect("Number");
        let expect = Number::from(123);
        assert_eq!(dec.order(), NumberOrder::Decimal);
        dec.demote();
        assert_eq!(dec.order(), NumberOrder::Int);
        assert_eq!(dec, expect);
    }

    #[test]
    fn demote_int() {
        let mut int = Number::from(123);
        let expect = Number::from(123);
        assert_eq!(int.order(), NumberOrder::Int);
        int.demote();
        assert_eq!(int.order(), NumberOrder::Int);
        assert_eq!(int, expect);
    }

    #[test]
    fn promote_dec() {
        let mut dec = "123.123".parse::<Number>().expect("Number");
        let expect = "123.123".parse::<Number>().expect("Number");
        assert_eq!(dec.order(), NumberOrder::Decimal);
        dec.promote();
        assert_eq!(dec.order(), NumberOrder::Decimal);
        assert_eq!(dec, expect);
    }

    #[test]
    fn promote_int() {
        let mut int = Number::from(123);
        let expect = "123.0".parse::<Number>().expect("Number");
        assert_eq!(int.order(), NumberOrder::Int);
        int.promote();
        assert_eq!(int.order(), NumberOrder::Decimal);
        assert_eq!(int, expect);
    }
}
