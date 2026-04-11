use crate::{Number, NumberError, number::ASTRO_CONSTS};
use astro_float::BigFloat;
use bigdecimal::BigDecimal;
use num_traits::Signed;
use std::str::FromStr;

impl Number {
    pub fn pow(&self, exponent: i64) -> Result<Self, NumberError> {
        match self {
            Number::Decimal(d) => Ok(Number::Decimal(d.powi(exponent))),
            Number::Int(i) => {
                let exponent_u32: u32 = exponent.try_into().map_err(|_| {
                    let m = format!("Number::Int exponent must fit in u32: {exponent} does not!");
                    NumberError::InvalidExponent { message: m }
                })?;
                Ok(Number::Int(i.pow(exponent_u32)))
            }
        }
    }

    /// The distance of a number from zero on a number line, regardless of direction.
    /// As a distance, it is always non-negative, effectively turning negative numbers
    /// positive and leaving positive numbers (and zero) unchanged.
    pub fn abs(&self) -> Self {
        match self {
            Number::Int(i) => Number::Int(i.abs()),
            Number::Decimal(d) => Number::Decimal(d.abs()),
        }
    }

    /// Variant is not coerced. If you call `.ceil()` with variant `Number::Int`,
    /// we just clone it and return it. If you call `.ceil()` on variant `Number::Decimal`,
    /// even though the result is a whole number, we keep it as a `Number::Decimal`.
    pub fn ceil(&self) -> Self {
        match self {
            Number::Int(_) => self.clone(),
            Number::Decimal(d) => {
                let bd = d.with_scale_round(0, bigdecimal::RoundingMode::Ceiling);
                Number::Decimal(bd)
            }
        }
    }

    /// Variant is not coerced. If you call `.floor()` with variant `Number::Int`,
    /// we just clone it and return it. If you call `.floor()` on variant `Number::Decimal`,
    /// even though the result is a whole number, we keep it as a `Number::Decimal`.
    pub fn floor(&self) -> Self {
        match self {
            Number::Int(_) => self.clone(),
            Number::Decimal(d) => {
                let bd = d.with_scale_round(0, bigdecimal::RoundingMode::Floor);
                Number::Decimal(bd)
            }
        }
    }

    pub fn sin(&mut self) {
        *self = match self {
            Number::Int(i) => Self::sin_str(&i.to_string()),
            Number::Decimal(d) => Self::sin_str(&d.to_string()),
        }
    }

    fn sin_str(s: &str) -> Number {
        ASTRO_CONSTS.with(|cc| {
            let bf = s.parse::<BigFloat>().expect("bigfloat");
            let r = bf.sin(53, astro_float::RoundingMode::None, &mut cc.borrow_mut());
            Number::Decimal(BigDecimal::from_str(&r.to_string()).expect("bigdecimal"))
        })
    }
}
