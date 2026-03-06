use crate::{ValueError, value::Value};

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
        let right = right.into();
        match self {
            Self::UnsignedInt(n) => {
                let right = right.as_u32()?;
                Ok(n.pow(right).into())
            }
            Self::UnsignedBigInt(n) => {
                let right = right.as_u32()?;
                Ok(n.pow(right).into())
            }
            Self::SignedInt(n) => {
                let right = right.as_u32()?;
                Ok(n.pow(right).into())
            }
            Self::SignedBigInt(n) => {
                let right = right.as_u32()?;
                Ok(n.pow(right).into())
            }
            Self::Float(n) => {
                if let Self::Float(e) = right {
                    Ok(n.powf(e).into())
                } else {
                    let right = right
                        .as_u32()?
                        .try_into()
                        .map_err(|_| ValueError::Overflow)?;
                    Ok(n.powi(right).into())
                }
            }
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

    #[test]
    fn as_u32() {
        assert_eq!(Value::UnsignedInt(10).as_u32().unwrap(), 10_u32);
        assert_eq!(Value::UnsignedBigInt(10).as_u32().unwrap(), 10_u32);
        assert_eq!(Value::SignedInt(10).as_u32().unwrap(), 10_u32);
        assert_eq!(Value::SignedBigInt(10).as_u32().unwrap(), 10_u32);
        assert_eq!(Value::Float(10.0).as_u32().unwrap(), 10_u32);
    }

    #[test]
    fn trunc_div() {
        assert_eq!(Value::UnsignedInt(10).trunc_div(2), Value::UnsignedInt(5));
        assert_eq!(
            Value::UnsignedBigInt(10).trunc_div(2),
            Value::UnsignedInt(5)
        );
        assert_eq!(Value::SignedInt(10).trunc_div(2), Value::UnsignedInt(5));
        assert_eq!(Value::SignedInt(-10).trunc_div(2), Value::SignedInt(-5));
        assert_eq!(Value::SignedBigInt(10).trunc_div(2), Value::UnsignedInt(5));
        assert_eq!(Value::SignedBigInt(-10).trunc_div(2), Value::SignedInt(-5));
        assert_eq!(
            Value::SignedBigInt(-i128::MAX).trunc_div(2),
            Value::SignedBigInt(-85070591730234615865843651857942052863)
        );
        assert_eq!(Value::Float(10.5).trunc_div(2), Value::UnsignedInt(5));
        assert_eq!(
            Value::Float(-f64::MAX).trunc_div(2),
            Value::Float(-8.988465674311579e307)
        );
    }
}
