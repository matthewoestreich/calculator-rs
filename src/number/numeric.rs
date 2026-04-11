use crate::{Number, NumberError, number::ASTRO_CONSTS};
use astro_float::{BigFloat, RoundingMode as AstroRoundingMode};
use bigdecimal::{BigDecimal, RoundingMode as BigDecimalRoundingMode};
use num_traits::Signed;

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
                let bd = d.with_scale_round(0, BigDecimalRoundingMode::Ceiling);
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
                let bd = d.with_scale_round(0, BigDecimalRoundingMode::Floor);
                Number::Decimal(bd)
            }
        }
    }

    pub fn sin(&self) -> Result<Self, NumberError> {
        Ok(match self {
            Number::Int(i) => Self::sin_str(&i.to_string())?,
            Number::Decimal(d) => Self::sin_str(&d.to_string())?,
        })
    }

    fn sin_str(s: &str) -> Result<Number, NumberError> {
        ASTRO_CONSTS.with(|cc| {
            let bf = s.parse::<BigFloat>()?;
            let res = bf.sin(53, AstroRoundingMode::None, &mut cc.borrow_mut());
            let bd = res.to_string().parse::<BigDecimal>()?;
            Ok(Number::Decimal(bd))
        })
    }
}

// ===========================================================================================
// ========================== Tests ==========================================================
// ===========================================================================================

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case::abs1("10", "10")]
    #[case::abs1_1("10.123", "10.123")]
    #[case::abs2("-10", "10")]
    #[case::abs2_1("-10.123", "10.123")]
    #[case::abs3("0", "0")]
    #[case::abs3_1("-0", "0")]
    fn abs(#[case] n: &str, #[case] expect: &str) {
        let x = n.parse::<Number>().unwrap();
        let e = expect.parse::<Number>().unwrap();
        let r = x.abs();
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::ceil1("14.7572", "15")]
    #[case::ceil2("0.1", "1")]
    #[case::ceil3("-2.3", "-2")]
    #[case::ceil4("-0.9", "0")]
    #[case::ceil5("-7.5", "-7")]
    #[case::ceil6("5.0", "5")]
    #[case::ceil7("-4.0", "-4")]
    #[case::ceil8("0.0", "0")]
    #[case::ceil9("-0.0", "-0")]
    #[case::ceil10(
        "0.0000000000000000000000000000000000000000000000000000000000000000000000001",
        "1"
    )]
    #[case::ceil11(
        "-0.0000000000000000000000000000000000000000000000000000000000000000000000001",
        "0"
    )]
    fn ceil(#[case] n: &str, #[case] expect: &str) {
        let x = n.parse::<Number>().unwrap();
        let bd = expect.parse::<BigDecimal>().unwrap();
        let e = Number::from(bd);
        let r = x.ceil();
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::flor1("14.7572", "14")]
    #[case::flor2("0.1", "0")]
    #[case::flor3("-2.3", "-3")]
    #[case::flor4("-0.9", "-1")]
    #[case::flor5("-7.5", "-8")]
    #[case::flor6("5.0", "5")]
    #[case::flor7("-4.0", "-4")]
    #[case::flor8("0.0", "0")]
    #[case::flor9("-0.0", "-0")]
    #[case::floor10(
        "0.0000000000000000000000000000000000000000000000000000000000000000000000001",
        "0"
    )]
    #[case::floor11(
        "-0.0000000000000000000000000000000000000000000000000000000000000000000000001",
        "-1"
    )]
    fn floor(#[case] n: &str, #[case] expect: &str) {
        let x = n.parse::<Number>().unwrap();
        let bd = expect.parse::<BigDecimal>().unwrap();
        let e = Number::from(bd);
        let r = x.floor();
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }
}
