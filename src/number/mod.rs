pub mod error;

#[macro_use]
mod macros;
mod arithmetic;
mod bitwise;
mod comparison;
mod conversion;
mod format;
mod numeric;

use astro_float::Consts;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use std::{cell::RefCell, cmp::Ordering};

#[derive(Clone)]
pub enum Number {
    Int(BigInt),
    Decimal(BigDecimal),
}

thread_local! {
    static ASTRO_CONSTS: RefCell<Consts> = RefCell::new(Consts::new().expect("astro-float consts"));
}

impl Number {
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
        if let Some(n) = self.take_int() {
            *self = Self::Decimal(BigDecimal::from(n));
        }
    }

    /// Converts `Number::Decimal` to `Number::Int`.
    /// IMPORTANT : this may cause loss of data/precision!
    pub fn demote(&mut self) {
        if let Some(ref mut d) = self.take_decimal() {
            let (d, _) = d.with_scale(0).into_bigint_and_scale();
            *self = Self::Int(d);
        }
    }

    /// Takes the backing BigInt leaivng 0 in it's place.
    /// Returns None if variant isn't Number::Int
    pub fn take_int(&mut self) -> Option<BigInt> {
        if let Self::Int(n) = self {
            return Some(std::mem::take(n));
        }
        None
    }

    /// Takes the backing BigDecimal leaving 0 in it's place.
    /// Returns None if variant isn't Number::Decimal
    pub fn take_decimal(&mut self) -> Option<BigDecimal> {
        if let Self::Decimal(d) = self {
            return Some(std::mem::take(d));
        }
        None
    }
}

// ===========================================================================================
// ========================== NumberOrder ====================================================
// ===========================================================================================

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
