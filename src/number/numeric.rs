use crate::{Number, NumberError, number::ASTRO_CONSTS};
use astro_float::{BigFloat, RoundingMode as AstroRoundingMode};
use bigdecimal::{BigDecimal, RoundingMode as BigDecimalRoundingMode};
use num_traits::Signed;

impl Number {
    /// This method returns an error if the result of pi is NaN or Inf.
    pub fn pi(precision: usize) -> Result<Number, NumberError> {
        ASTRO_CONSTS.with(|cc| {
            let mut ctx = cc.borrow_mut();
            let pi_bf = ctx.pi(precision, AstroRoundingMode::None);
            if pi_bf.is_nan() || pi_bf.is_inf() {
                return Err(NumberError::IsNaNOrInfinity);
            }
            let pi_bd = pi_bf.to_string().parse::<BigDecimal>()?;
            Ok(Number::Decimal(pi_bd))
        })
    }

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

    /// After converting to BigFloat from &str, we store the precision. We then use that precision
    /// in the result - this is an attempt to keep precision as close as possible to what was passed in.
    /// If the value you passed in is considered NaN or Inf, we default to 64 bits of precision.
    pub fn sin(&self) -> Result<Self, NumberError> {
        Ok(match self {
            Number::Int(i) => Self::sin_str(&i.to_string())?,
            Number::Decimal(d) => Self::sin_str(&d.to_string())?,
        })
    }

    /// Please see the comments on `sin` fn.
    fn sin_str(s: &str) -> Result<Number, NumberError> {
        ASTRO_CONSTS.with(|cc| {
            let og_bf = s.to_string().parse::<BigFloat>()?;
            let prec = og_bf.precision().unwrap_or(64);
            let sin_bf = og_bf.sin(prec, AstroRoundingMode::None, &mut cc.borrow_mut());
            let result = sin_bf.to_string().parse::<BigDecimal>()?;
            Ok(Number::Decimal(result))
        })
    }

    /// After converting to BigFloat from &str, we store the precision. We then use that precision
    /// in the result - this is an attempt to keep precision as close as possible to what was passed in.
    /// If the value you passed in is considered NaN or Inf, we default to 64 bits of precision.
    pub fn tan(&self) -> Result<Self, NumberError> {
        match self {
            Number::Int(i) => Self::tan_str(&i.to_string()),
            Number::Decimal(d) => Self::tan_str(&d.to_string()),
        }
    }

    /// Please see the comments on `tan` fn.
    fn tan_str(s: &str) -> Result<Number, NumberError> {
        ASTRO_CONSTS.with(|cc| {
            let og_bf = s.to_string().parse::<BigFloat>()?;
            let prec = og_bf.precision().unwrap_or(64);
            let tan_bf = og_bf.tan(prec, AstroRoundingMode::None, &mut cc.borrow_mut());
            let result = tan_bf.to_string().parse::<BigDecimal>()?;
            Ok(Number::Decimal(result))
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

    #[test]
    fn foofoo() {
        //let p = Number::pi(u64::MAX as usize).unwrap();
        //println!("{p}");

        ASTRO_CONSTS.with(|cc| {
            let mut ctx = cc.borrow_mut();
            let p = ctx.pi(u64::MAX as usize, astro_float::RoundingMode::None);
            println!("{p}");
        })
    }

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

    #[rstest]
    #[case::sin1("12", "-0.53657291800043497166")]
    #[case::sin2("-12", "0.53657291800043497166")]
    #[case::sin3("5.5", "-0.70554032557039190619")]
    #[case::sin4("0", "0.0")]
    #[case::sin5("0.0", "0.0")]
    #[case::sin6("-0", "0.0")]
    #[case::sin7("-0.0", "0.0")]
    #[case::sin8("0.1", "0.099833416646828152304")]
    #[case::sin9("18446744073709551615", "0.853986978245566353867800472464340994594")]
    #[case::sin10(
        "340282366920938463463374607431768211455",
        "0.3392599560368070091613048584893837401487259503307889734193"
    )]
    #[case::sin11(
        "3402823669209384634633746074317682114553242924593",
        "0.7003828704955936420115909443933312486949704627925119745478"
    )]
    fn sin(#[case] n: &str, #[case] expect: &str) {
        let x = n.parse::<Number>().unwrap();
        let e = expect.parse::<Number>().unwrap();
        let r = x.sin().expect("no errors in sin");
        assert_eq!(r, e, "expected {e} got {r}");
    }

    #[rstest]
    #[case::tan1("12", "-0.63585992866158079246")]
    #[case::tan2("-12", "0.63585992866158079246")]
    #[case::tan3("5.5", "-0.99558405221388501766")]
    #[case::tan4("0", "0.0")]
    #[case::tan5("0.0", "0.0")]
    #[case::tan6("-0", "0.0")]
    #[case::tan7("-0.0", "0.0")]
    #[case::tan8("0.1", "0.10033467208545054506")]
    #[case::tan9("18446744073709551615", "-1.64135345767507783113974557998746062216")]
    #[case::tan10(
        "340282366920938463463374607431768211455",
        "0.3606490941686779605133609111062213018578087604852921288697"
    )]
    #[case::tan11(
        "3402823669209384634633746074317682114553242924593",
        "-0.9812481156540047271297101056006535923508753040076364516032"
    )]
    fn tan(#[case] n: &str, #[case] expect: &str) {
        let x = n.parse::<Number>().unwrap();
        let e = expect.parse::<Number>().unwrap();
        let r = x.tan().expect("no errors in sin");
        assert_eq!(r, e, "expected {e} got {r}");
    }
}
