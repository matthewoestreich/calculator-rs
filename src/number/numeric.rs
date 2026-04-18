use crate::{Number, NumberError, number::ASTRO_CONSTS};
use astro_float::{BigFloat, RoundingMode as AstroRoundingMode};
use bigdecimal::{BigDecimal, RoundingMode as BigDecimalRoundingMode};
use num_traits::Signed;

impl Number {
    /// Returns the numeric constant `pi` with specified precision.
    /// This method returns an error if the result of pi is NaN or Inf.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let pi = Number::pi(64);
    /// let expect = "3.1415926535897932383".parse::<Number>().expect("Number::Decimal");
    /// assert_eq!(pi, Ok(expect));
    ///
    /// let pi = Number::pi(128);
    /// let expect = "3.1415926535897932384626433832795028842".parse::<Number>().expect("Number::Decimal");
    /// assert_eq!(pi, Ok(expect));
    /// ```
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

    /// Get raw digit count, excluding `-` or `.` symbols.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let a = Number::from(3245342);
    /// assert_eq!(a.digit_count(), 7);
    ///
    /// let b = "3245.5323259".parse::<Number>().expect("Number::Decimal");
    /// assert_eq!(b.digit_count(), 11);
    ///
    /// let c = "-321145.5323259".parse::<Number>().expect("Number::Decimal");
    /// assert_eq!(c.digit_count(), 13);
    /// ```
    pub fn digit_count(&self) -> usize {
        match self {
            Number::Int(i) => {
                let l = i.to_string().len();
                if i.is_negative() { l - 1 } else { l }
            }
            Number::Decimal(d) => {
                let mut l = d.to_plain_string().len();
                if d.is_negative() {
                    l -= 1
                }
                if !d.is_integer() {
                    l -= 1
                }
                l
            }
        }
    }

    /// Exponentiation - raise `self` to `exponent.`
    ///
    /// <div class="warning">
    ///
    /// # Warning
    ///
    /// Even though this method takes an `i64`, if `self` is variant
    /// `Number::Int(_)`, then the exponent must fit in `u32`!
    ///
    /// If your exponent must fit into `i64` you can convert your
    /// `Number::Int(..)` instance into `Number::Decimal(..)` by calling
    /// `my_number_int.promote()` and then calling `my_number_int.pow(some_i64)`
    ///
    /// </div>
    ///
    /// ```rust
    /// use calcinum::{Number, NumberError};
    ///
    /// let a = Number::from(2);
    /// assert_eq!(a.pow(4), Ok(Number::from(16)));
    ///
    /// let b = "12.3".parse::<Number>().expect("Number::Decimal");
    /// assert_eq!(b.pow(4), Ok(Number::from_f64_unchecked(22888.6641)));
    ///
    /// // Demonstrating that even though the exponent fits in `i64` the call
    /// // still returns an error due to the variant being `Number::Int(_)`.
    /// let c = Number::from(1);
    /// let exponent_fits_in_i64 = u32::MAX as i64 + 1;
    /// let result = c.pow(exponent_fits_in_i64);
    /// assert!(matches!(result, Err(NumberError::InvalidExponent { .. })));
    /// ```
    pub fn pow(&self, exponent: i64) -> Result<Number, NumberError> {
        match self {
            Number::Int(i) => {
                let exponent_u32: u32 = exponent.try_into().map_err(|_| {
                    let m = format!("Number::Int exponent must fit in u32: {exponent} does not!");
                    NumberError::InvalidExponent { message: m }
                })?;
                Ok(Number::Int(i.pow(exponent_u32)))
            }
            Number::Decimal(d) => Ok(Number::Decimal(d.powi(exponent))),
        }
    }

    /// Same as [`pow`](crate::Number#method.pow), but mutates `self`.
    /// Exponentiation - raise `self` to `exponent.`
    ///
    /// <div class="warning">
    ///
    /// # Warning
    ///
    /// Even though this method takes an `i64`, if `self` is variant
    /// `Number::Int(_)`, then the exponent must fit in `u32`!
    ///
    /// If your exponent must fit into `i64` you can convert your
    /// `Number::Int(..)` instance into `Number::Decimal(..)` by calling
    /// `my_number_int.promote()` and then calling `my_number_int.pow(some_i64)`
    ///
    /// </div>
    ///
    /// ```rust
    /// use calcinum::{Number, NumberError};
    ///
    /// let mut a = Number::from(2);
    /// let _possible_error = a.pow_assign(4);
    /// assert_eq!(a, Number::from(16));
    ///
    /// let mut b = "12.3".parse::<Number>().expect("Number::Decimal");
    /// let _possible_error = b.pow_assign(4);
    /// assert_eq!(b, Number::from_f64_unchecked(22888.6641));
    ///
    /// // Demonstrating that even though the exponent fits in `i64` the call
    /// // still returns an error due to the variant being `Number::Int(_)`.
    /// let mut c = Number::from(1);
    /// let exponent_fits_in_i64 = u32::MAX as i64 + 1;
    /// let result = c.pow_assign(exponent_fits_in_i64);
    /// assert!(matches!(result, Err(NumberError::InvalidExponent { .. })));
    /// ```
    pub fn pow_assign(&mut self, exponent: i64) -> Result<(), NumberError> {
        *self = self.pow(exponent)?;
        Ok(())
    }

    /// The absolute, or non-negative distance of `self` from 0.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let a = Number::from(-5);
    /// assert_eq!(a.abs(), Number::from(5));
    ///
    /// let b = Number::from_f64_unchecked(-5.5);
    /// assert_eq!(b.abs(), Number::from_f64_unchecked(5.5));
    /// ```
    pub fn abs(&self) -> Self {
        match self {
            Number::Int(i) => Number::Int(i.abs()),
            Number::Decimal(d) => Number::Decimal(d.abs()),
        }
    }

    /// Same as [`abs`](crate::Number#method.abs), but with `self` assignment.
    /// The absolute, or non-negative, distance of `self` from 0.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let mut a = Number::from(-5);
    /// a.abs_assign();
    /// assert_eq!(a, Number::from(5));
    ///
    /// let mut b = Number::from_f64_unchecked(-5.5);
    /// b.abs_assign();
    /// assert_eq!(b, Number::from_f64_unchecked(5.5));
    /// ```
    pub fn abs_assign(&mut self) {
        *self = self.abs();
    }

    /// Smallest integer greater than or equal to `self`.
    ///
    /// Variant is not coerced. If you call `.ceil()` with variant `Number::Int`,
    /// we just clone it and return it.
    ///
    /// If you call `.ceil()` on variant `Number::Decimal`, even though the result
    /// will be an integer, we keep it as a `Number::Decimal`.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let a = Number::from_f64_unchecked(123.123);
    /// assert_eq!(a.ceil(), Number::from_f64_unchecked(124f64));
    /// ```
    pub fn ceil(&self) -> Self {
        match self {
            Number::Int(_) => self.clone(),
            Number::Decimal(d) => {
                let bd = d.with_scale_round(0, BigDecimalRoundingMode::Ceiling);
                Number::Decimal(bd)
            }
        }
    }

    /// Same as [`ceil`](crate::Number#method.ceil), but with `self` assignment. Smallest
    /// integer greater than or equal to `self`.
    ///
    /// Variant is not coerced. If you call `.ceil()` with variant `Number::Int`,
    /// this is essentially a no-op.
    ///
    /// If you call `.ceil()` on variant `Number::Decimal`, even though the result
    /// will be an integer, we keep it as a `Number::Decimal`.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let mut a = Number::from_f64_unchecked(123.123);
    /// a.ceil_assign();
    /// assert_eq!(a, Number::from_f64_unchecked(124f64));
    /// ```
    pub fn ceil_assign(&mut self) {
        if self.is_decimal() {
            *self = self.ceil();
        }
    }

    /// Greatest integer less than or equal to `self`.
    ///
    /// Variant is not coerced. If you call `.floor()` with variant `Number::Int`,
    /// we just clone it and return it.
    ///
    /// If you call `.floor()` on variant `Number::Decimal`, even though the result
    /// will be an integer, we keep it as a `Number::Decimal`.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let a = Number::from_f64_unchecked(123.123);
    /// assert_eq!(a.floor(), Number::from_f64_unchecked(123f64));
    /// ```
    pub fn floor(&self) -> Self {
        match self {
            Number::Int(_) => self.clone(),
            Number::Decimal(d) => {
                let bd = d.with_scale_round(0, BigDecimalRoundingMode::Floor);
                Number::Decimal(bd)
            }
        }
    }

    /// Same as [`floor`](crate::Number#method.floor), but with `self` assignment.
    /// Greatest integer less than or equal to `self`.
    ///
    /// Variant is not coerced. If you call `.floor()` with variant `Number::Int`,
    /// this is essentially a no-op.
    ///
    /// If you call `.floor()` on variant `Number::Decimal`, even though the result
    /// will be an integer, we keep it as a `Number::Decimal`.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let mut a = Number::from_f64_unchecked(123.123);
    /// a.floor_assign();
    /// assert_eq!(a, Number::from_f64_unchecked(123f64));
    /// ```
    pub fn floor_assign(&mut self) {
        if self.is_decimal() {
            *self = self.floor();
        }
    }

    /// Sine function. Computes the unit-circle y-coordinate for a given angle in radians.
    ///
    /// We attempt to retain native precision during calculations. If `self` is considered
    /// `NaN` or `Infinity`, we fall back to using 64-bits of precision.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let a = Number::from(12);
    /// let expect = "-0.53657291800043497166".parse::<Number>().expect("Number::Decimal");
    /// assert_eq!(a.sin(), Ok(expect));
    /// ```
    pub fn sin(&self) -> Result<Self, NumberError> {
        Ok(match self {
            Number::Int(i) => Self::sin_str(&i.to_string())?,
            Number::Decimal(d) => Self::sin_str(&d.to_string())?,
        })
    }

    /// Same as [`sin`](crate::Number#method.sin), but with `self` assignment.
    ///
    /// Sine function. Computes the unit-circle y-coordinate for a given angle in radians.
    ///
    /// We attempt to retain native precision during calculations. If `self` is considered
    /// `NaN` or `Infinity`, we fall back to using 64-bits of precision.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let mut a = Number::from(12);
    /// let _possible_error = a.sin_assign();
    /// let expect = "-0.53657291800043497166".parse::<Number>().expect("Number::Decimal");
    /// assert_eq!(a, expect);
    /// ```
    pub fn sin_assign(&mut self) -> Result<(), NumberError> {
        *self = self.sin()?;
        Ok(())
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

    /// Cosine function. Computes the unit-circle x-coordinate for a given angle in radians.
    ///
    /// We attempt to retain native precision during calculations. If `self` is considered
    /// `NaN` or `Infinity`, we fall back to using 64-bits of precision.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let a = Number::from(12);
    /// let expect = "0.84385395873249210465".parse::<Number>().expect("Number::Int");
    /// assert_eq!(a.cos(), Ok(expect));
    /// ```
    pub fn cos(&self) -> Result<Number, NumberError> {
        match self {
            Number::Int(i) => Self::cos_str(&i.to_string()),
            Number::Decimal(d) => Self::cos_str(&d.to_string()),
        }
    }

    /// Same as [`cos`](crate::Number#method.cos), but with `self` assignment.
    ///
    /// Cosine function. Computes the unit-circle x-coordinate for a given angle in radians.
    ///
    /// We attempt to retain native precision during calculations. If `self` is considered
    /// `NaN` or `Infinity`, we fall back to using 64-bits of precision.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let mut a = Number::from(12);
    /// let _possible_error = a.cos_assign();
    /// let expect = "0.84385395873249210465".parse::<Number>().expect("Number::Int");
    /// assert_eq!(a, expect);
    /// ```
    pub fn cos_assign(&mut self) -> Result<(), NumberError> {
        *self = self.cos()?;
        Ok(())
    }

    /// Please see comments on `cos` method.
    fn cos_str(s: &str) -> Result<Number, NumberError> {
        ASTRO_CONSTS.with(|cc| {
            let og_bf = s.to_string().parse::<BigFloat>()?;
            let prec = og_bf.precision().unwrap_or(64).max(64);
            let cos_bf = og_bf.cos(prec, AstroRoundingMode::None, &mut cc.borrow_mut());
            let result = cos_bf.to_string().parse::<BigDecimal>()?;
            Ok(Number::Decimal(result))
        })
    }

    /// Tangent function. Computes the unit-circle y/x ratio for a given angle in radians.
    ///
    /// We attempt to retain native precision during calculations. If `self` is considered
    /// `NaN` or `Infinity`, we fall back to using 64-bits of precision.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let a = Number::from(12);
    /// let expect = "-0.63585992866158079246".parse::<Number>().expect("Number::Decimal");
    /// assert_eq!(a.tan(), Ok(expect));
    /// ```
    pub fn tan(&self) -> Result<Self, NumberError> {
        match self {
            Number::Int(i) => Self::tan_str(&i.to_string()),
            Number::Decimal(d) => Self::tan_str(&d.to_string()),
        }
    }

    /// Same as [`tan`](crate::Number#method.tan), but with `self` assignment.
    ///
    /// Tangent function. Computes the unit-circle y/x ratio for a given angle in radians.
    ///
    /// We attempt to retain native precision during calculations. If `self` is considered
    /// `NaN` or `Infinity`, we fall back to using 64-bits of precision.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let mut a = Number::from(12);
    /// let _possible_error = a.tan_assign();
    /// let expect = "-0.63585992866158079246".parse::<Number>().expect("Number::Decimal");
    /// assert_eq!(a, expect);
    /// ```
    pub fn tan_assign(&mut self) -> Result<(), NumberError> {
        *self = self.tan()?;
        Ok(())
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

    /// Return `self` rounded to ‘round_digits’ precision after the decimal point.
    /// Rounding mode is half even; round to ‘nearest neighbor’, if equidistant, round
    /// towards nearest even digit.
    ///
    /// Variant is not coerced. If you call `.ceil()` with variant `Number::Int`,
    /// we just clone it and return it.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let a = Number::from_f64_unchecked(123.887);
    /// assert_eq!(a.round(2), Number::from_f64_unchecked(123.89));
    ///
    /// let b = Number::from_f64_unchecked(123.884);
    /// assert_eq!(b.round(2), Number::from_f64_unchecked(123.88));
    ///
    /// let c = Number::from_f64_unchecked(123.884);
    /// assert_eq!(c.round(0), Number::from_f64_unchecked(124f64));
    /// ```
    pub fn round(&self, round_digits: i64) -> Self {
        match self {
            Number::Int(_) => self.clone(),
            Number::Decimal(d) => Self::Decimal(d.round(round_digits)),
        }
    }

    /// Same as [`round`](crate::Number#method.round) but with `self` assignment.
    ///
    /// Return `self` rounded to ‘round_digits’ precision after the decimal point.
    /// Rounding mode is half even; round to ‘nearest neighbor’, if equidistant, round
    /// towards nearest even digit.
    ///
    /// Variant is not coerced. If you call `.ceil()` with variant `Number::Int`,
    /// this is essentially a no-op.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let mut a = Number::from_f64_unchecked(123.887);
    /// a.round_assign(2);
    /// assert_eq!(a, Number::from_f64_unchecked(123.89));
    ///
    /// let mut b = Number::from_f64_unchecked(123.884);
    /// b.round_assign(2);
    /// assert_eq!(b, Number::from_f64_unchecked(123.88));
    ///
    /// let mut c = Number::from_f64_unchecked(123.884);
    /// c.round_assign(0);
    /// assert_eq!(c, Number::from_f64_unchecked(124f64));
    /// ```
    pub fn round_assign(&mut self, round_digits: i64) {
        if self.is_decimal() {
            *self = self.round(round_digits);
        }
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
    #[case::cos1("12", "0.84385395873249210465")]
    #[case::cos2("-34.2", "-0.9367678684526686154")]
    #[case::cos3("-0", "1.0")]
    #[case::cos4("0", "1.0")]
    #[case::cos5("0.0", "1.0")]
    #[case::cos6("-0.0", "1.0")]
    #[case::cos7("27", "-0.29213880873383619335")]
    #[case::cos8("-27", "-0.29213880873383619335")]
    fn cos(#[case] n: &str, #[case] expect: &str) {
        let x = n.parse::<Number>().expect("cos");
        let e = expect.parse::<Number>().expect("expect");
        let r = x.cos().expect("no cos errors");
        assert_eq!(r, e, "expected cos {e} got cos {r}");
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
        let r = x.tan().expect("no errors in tan");
        assert_eq!(r, e, "expected {e} got {r}");
    }

    #[rstest]
    #[case::round1("12.1", 0, "12.0")]
    #[case::round2("654302", 0, "654302")]
    #[case::round3("42323.345456344", 7, "42323.3454563")]
    #[case::round4("42323.345456354", 7, "42323.3454564")]
    #[case::round5("42323.345456364", 7, "42323.3454564")]
    #[case::round6("-0.1115", 3, "-0.112")]
    #[case::tie_even_2("2.5", 0, "2.0")]
    #[case::tie_even_3("3.5", 0, "4.0")]
    #[case::tie_even_4("4.5", 0, "4.0")]
    #[case::carry1("999.999", 2, "1000.00")]
    #[case::carry2("1.9999", 3, "2.000")]
    #[case::carry3("9.9995", 3, "10.000")]
    #[case::carry_tie("9.995", 2, "10.00")]
    #[case::carry_tie2("1.005", 2, "1.00")]
    #[case::large_int("999999999999.6", 0, "1000000000000.0")]
    #[case::large_int2("1000000000000.4", 0, "1000000000000.0")]
    #[case::zero("0.0", 0, "0.0")]
    #[case::negative_zero("-0.0", 0, "0.0")]
    #[case::small_neg("-0.4", 0, "0.0")]
    #[case::idempotent1("12.000", 0, "12.0")]
    #[case::idempotent2("12.00", 1, "12.00")]
    #[case::idempotent3("12.3", 1, "12.3")]
    #[case::long_fraction("1.123456789123456789", 5, "1.12346")]
    #[case::down_vs_up1("1.499999", 0, "1.0")]
    #[case::down_vs_up2("1.500001", 0, "2.0")]
    fn round(#[case] n: &str, #[case] round_digits: i64, #[case] expect: &str) {
        let x = n.parse::<Number>().unwrap();
        let e = expect.parse::<Number>().unwrap();
        let r = x.round(round_digits);
        assert_eq!(r, e, "[round] expected {e} got {r}");

        let mut x = n.parse::<Number>().unwrap();
        let e = expect.parse::<Number>().unwrap();
        x.round_assign(round_digits);
        assert_eq!(x, e, "[round_assign] expected {e} got {r}");
    }
}
