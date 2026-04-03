use crate::{ValueError, value::Value};
use num_traits::{Signed, ToPrimitive};

impl Value {
    pub(crate) fn as_u32(self) -> Result<u32, ValueError> {
        match self {
            Self::UnsignedInt(n) => u32::try_from(n).map_err(|_| ValueError::Overflow),
            Self::UnsignedBigInt(n) => u32::try_from(n).map_err(|_| ValueError::Overflow),
            Self::SignedInt(n) => u32::try_from(n).map_err(|_| ValueError::Overflow),
            Self::SignedBigInt(n) => u32::try_from(n).map_err(|_| ValueError::Overflow),
            Self::Float(n) => {
                if n < 0.0 {
                    return Err(ValueError::Overflow);
                }
                if n.fract() != 0.0 {
                    return Err(ValueError::ImproperlyFloat);
                }
                // a 64-bit integer has at least enough precision to capture the integer part of this number
                let n = n as u64;

                u32::try_from(n).map_err(|_| ValueError::Overflow)
            }
        }
    }

    /// Divide this value by another, truncating (not flooring) the result to the next lowest integer.
    pub fn trunc_div(mut self, other: impl Into<Self>) -> Self {
        self /= other;
        if let Self::Float(n) = &mut self {
            *n = n.trunc();
        }
        self.demote();
        self
    }

    /// Raise this value by another.
    pub fn pow(self, right: impl Into<Self>) -> Result<Self, ValueError> {
        let exponent = right.into();

        match (self, exponent) {
            // Unsigned integer ^ unsigned integer
            (Value::UnsignedInt(base), Value::UnsignedInt(exp)) => {
                let result = base.checked_pow(exp as u32).ok_or(ValueError::Overflow)?;
                Ok(Value::UnsignedInt(result))
            }
            // Signed integer ^ unsigned integer
            (Value::SignedInt(base), Value::UnsignedInt(exp)) => {
                let result = base.checked_pow(exp as u32).ok_or(ValueError::Overflow)?;
                Ok(Value::SignedInt(result))
            }
            // UnsignedBigInt ^ UnsignedInt
            (Value::UnsignedBigInt(base), Value::UnsignedInt(exp)) => {
                Ok(Value::UnsignedBigInt(base.pow(exp as u32)))
            }
            // SignedBigInt ^ UnsignedInt
            (Value::SignedBigInt(base), Value::UnsignedInt(exp)) => {
                Ok(Value::SignedBigInt(base.pow(exp as u32)))
            }
            // Any ^ Float: promote base to f64 and compute
            (Value::UnsignedInt(b), Value::Float(e)) => Ok(Value::Float((b as f64).powf(e))),
            (Value::SignedInt(b), Value::Float(e)) => Ok(Value::Float((b as f64).powf(e))),
            (Value::UnsignedBigInt(b), Value::Float(e)) => {
                Ok(Value::Float(b.to_f64().unwrap().powf(e)))
            }
            (Value::SignedBigInt(b), Value::Float(e)) => {
                Ok(Value::Float(b.to_f64().unwrap().powf(e)))
            }
            (Value::Float(b), Value::Float(e)) => Ok(Value::Float(b.powf(e))),
            (Value::Float(b), Value::UnsignedInt(e)) => Ok(Value::Float(b.powi(e as i32))),
            (Value::Float(b), Value::SignedInt(e)) => Ok(Value::Float(b.powi(e as i32))),
            // Negative exponents for integer types
            (Value::UnsignedInt(_), Value::SignedInt(e))
            | (Value::SignedInt(_), Value::SignedInt(e))
            | (Value::UnsignedBigInt(_), Value::SignedInt(e))
            | (Value::SignedBigInt(_), Value::SignedInt(e))
                if e < 0 =>
            {
                Err(ValueError::NegativeExponent)
            }

            // Everything else is unsupported
            _ => Err(ValueError::Overflow),
        }
    }

    /// Compute the absolute value of this value.
    pub fn abs(self) -> Self {
        match self {
            Self::UnsignedInt(n) => n.into(),
            Self::UnsignedBigInt(n) => n.into(),
            Self::SignedInt(n) => n.abs().into(),
            Self::SignedBigInt(n) => n.abs().into(),
            Self::Float(n) => n.abs().into(),
        }
    }

    /// Compute the smallest integer greater than or equal to self.
    pub fn ceil(self) -> Self {
        if let Self::Float(n) = self {
            let mut out = Self::from(n.ceil());
            out.demote();
            out
        } else {
            self
        }
    }

    /// Compute the greatest integer less than or equal to self.
    pub fn floor(self) -> Self {
        if let Self::Float(n) = self {
            let mut out = Self::from(n.floor());
            out.demote();
            out
        } else {
            self
        }
    }

    /// Round self to the nearest integer; halfway cases away from 0.0.
    pub fn round(self) -> Self {
        if let Self::Float(n) = self {
            let mut out = Self::from(n.round());
            out.demote();
            out
        } else {
            self
        }
    }

    /// Compute the sine of self.
    pub fn sin(mut self) -> Self {
        {
            let f = self.promote_to_float();
            *f = f.sin();
        }
        self
    }

    /// Compute the cosine of self.
    pub fn cos(mut self) -> Self {
        {
            let f = self.promote_to_float();
            *f = f.cos();
        }
        self
    }

    /// Compute the tangent of self.
    pub fn tan(mut self) -> Self {
        {
            let f = self.promote_to_float();
            *f = f.tan();
        }
        self
    }

    /// Compute the hyperbolic sine of self.
    pub fn sinh(mut self) -> Self {
        {
            let f = self.promote_to_float();
            *f = f.sinh();
        }
        self
    }

    /// Compute the hyperbolic cosine of self.
    pub fn cosh(mut self) -> Self {
        {
            let f = self.promote_to_float();
            *f = f.cosh();
        }
        self
    }

    /// Compute the hyperbolic tangent of self.
    pub fn tanh(mut self) -> Self {
        {
            let f = self.promote_to_float();
            *f = f.tanh();
        }
        self
    }

    /// Compute the arcsine of self.
    pub fn asin(mut self) -> Self {
        {
            let f = self.promote_to_float();
            *f = f.asin();
        }
        self
    }

    /// Compute the arccosine of self.
    pub fn acos(mut self) -> Self {
        {
            let f = self.promote_to_float();
            *f = f.acos();
        }
        self
    }

    /// Compute the arctangent of self.
    pub fn atan(mut self) -> Self {
        {
            let f = self.promote_to_float();
            *f = f.atan();
        }
        self
    }

    /// Compute the inverse hyperbolic sine of self.
    pub fn asinh(mut self) -> Self {
        {
            let f = self.promote_to_float();
            *f = f.asinh();
        }
        self
    }

    /// Compute the inverse hyperbolic cosine of self.
    pub fn acosh(mut self) -> Self {
        {
            let f = self.promote_to_float();
            *f = f.acosh();
        }
        self
    }

    /// Compute the inverse hyperbolic tangent of self.
    pub fn atanh(mut self) -> Self {
        {
            let f = self.promote_to_float();
            *f = f.atanh();
        }
        self
    }

    /// Convert self as degrees to radians.
    pub fn rad(mut self) -> Self {
        {
            let f = self.promote_to_float();
            *f *= std::f64::consts::PI / 180.0;
        }
        self
    }

    /// Convert self as radians to degrees.
    pub fn deg(mut self) -> Self {
        {
            let f = self.promote_to_float();
            *f *= 180.0 / std::f64::consts::PI;
        }
        self
    }

    /// Determine the square root of self.
    pub fn sqrt(mut self) -> Self {
        {
            let f = self.promote_to_float();
            *f = f.sqrt();
        }
        self
    }

    /// Determine the cube root of self.
    pub fn cbrt(mut self) -> Self {
        {
            let f = self.promote_to_float();
            *f = f.cbrt();
        }
        self
    }

    /// Determine the base-10 logarithm of self.
    pub fn log(mut self) -> Self {
        {
            let f = self.promote_to_float();
            *f = f.log10();
        }
        self
    }

    /// Determine the base-2 logarithm of self
    pub fn lg(mut self) -> Self {
        {
            let f = self.promote_to_float();
            *f = f.log2();
        }
        self
    }

    /// Determine the base-`e` (natural) logarithm of self.
    pub fn ln(mut self) -> Self {
        {
            let f = self.promote_to_float();
            *f = f.ln();
        }
        self
    }

    /// Determine `e**self`
    pub fn exp(mut self) -> Self {
        {
            let f = self.promote_to_float();
            *f = f.exp();
        }
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::value::Order;
    use rstest::*;

    #[rstest]
    #[case(10, 10_u32)]
    #[case(10.0, 10_u32)]
    #[case(10_i128, 10_u32)]
    fn as_u32(#[case] value: impl Into<Value>, #[case] expect: u32) {
        let v: Value = value.into();
        let r = v.as_u32().unwrap();
        assert_eq!(r, expect);
    }

    #[rstest]
    #[case::trunc_div_1(10, 2, 5, Order::UnsignedInt)]
    #[case::trunc_div_2(10_i128, 2, 5, Order::UnsignedInt)]
    #[case::trunc_div_3(10_u64, 2, 5, Order::UnsignedInt)]
    #[case::trunc_div_4(-10_i64, 2, -5_i64, Order::SignedInt)]
    #[case::trunc_div_5(-10_i128, 2, -5_i64, Order::SignedInt)]
    #[case::trunc_div_6(-i128::MAX, 2, -85070591730234615865843651857942052863_i128, Order::SignedBigInt)]
    #[case::trunc_div_7(10.5, 2, 5, Order::UnsignedInt)]
    #[case::trunc_div_8(-f64::MAX, 2, -8.988465674311579e307, Order::Float)]
    fn trunc_div(
        #[case] value: impl Into<Value>,
        #[case] div_by: impl Into<Value>,
        #[case] expect_value: impl Into<Value>,
        #[case] expect_order: Order,
    ) {
        let v: Value = value.into();
        let r = v.trunc_div(div_by);
        assert_eq!(
            r.order(),
            expect_order,
            "expected {expect_order:?} got {:?}",
            r.order()
        );
        let expect_value = expect_value.into();
        assert_eq!(r, expect_value, "expected {expect_value:?} got {r:?}");
    }

    #[rstest]
    #[case::pow1(10, 2, 100)]
    #[case::pow2(i64::MAX, u32::MAX, f64::INFINITY)]
    #[case::pow3(
        4611686018427387903_i64,
        2,
        21267647932558653957237540927630737409_i128
    )]
    #[case::pow4(i128::MAX, 2, 2.894802230932905e76)]
    #[case::pow5(i128::MAX, 3, 4.92525077454931e114)]
    fn pow(
        #[case] value: impl Into<Value>,
        #[case] raise_to: impl Into<Value>,
        #[case] expect_value: impl Into<Value>,
    ) {
        let v: Value = value.into();
        println!("v = {v:?}");
        let r = v.pow(raise_to).unwrap();
        let expect_value = expect_value.into();
        assert_eq!(r, expect_value, "expected {expect_value:?} got {r:?}");
    }
}
