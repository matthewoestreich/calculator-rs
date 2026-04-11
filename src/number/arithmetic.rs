use crate::Number;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

// ===========================================================================================
// ========================== AddAssign/Add ==================================================
// ===========================================================================================

impl AddAssign<Number> for Number {
    fn add_assign(&mut self, rhs: Number) {
        self.add_assign(&rhs);
    }
}

impl AddAssign<&Number> for Number {
    fn add_assign(&mut self, rhs: &Number) {
        match_arithmetic_assign!(self, rhs, +);
    }
}

impl Add<Number> for Number {
    type Output = Number;

    fn add(mut self, rhs: Number) -> Self::Output {
        self.add_assign(&rhs);
        self
    }
}

impl Add<&Number> for &Number {
    type Output = Number;

    fn add(self, rhs: &Number) -> Self::Output {
        match_arithmetic!(self, rhs, +)
    }
}

// ===========================================================================================
// ========================== SubAssign/Sub ==================================================
// ===========================================================================================

impl SubAssign<Number> for Number {
    fn sub_assign(&mut self, rhs: Number) {
        self.sub_assign(&rhs);
    }
}

impl SubAssign<&Number> for Number {
    fn sub_assign(&mut self, rhs: &Number) {
        match_arithmetic_assign!(self, rhs, -);
    }
}

impl Sub<Number> for Number {
    type Output = Number;

    fn sub(mut self, rhs: Number) -> Self::Output {
        self.sub_assign(&rhs);
        self
    }
}

impl Sub<&Number> for &Number {
    type Output = Number;

    fn sub(self, rhs: &Number) -> Self::Output {
        match_arithmetic!(self, rhs, -)
    }
}

// ===========================================================================================
// ========================== DivAssign/Div ==================================================
// ===========================================================================================

impl DivAssign<Number> for Number {
    fn div_assign(&mut self, rhs: Number) {
        self.div_assign(&rhs);
    }
}

impl DivAssign<&Number> for Number {
    fn div_assign(&mut self, rhs: &Number) {
        *self = match (&self, rhs) {
            // Delegate to `impl Div<&Number> for &Number` since we would call the same code regardless
            (Number::Decimal(_), Number::Decimal(_)) | (Number::Decimal(_), Number::Int(_)) => {
                &*self / rhs
            }
            (Number::Int(_), Number::Decimal(_)) => {
                self.promote(); // Both sides must be Number::Decimal
                &*self / rhs
            }
            (Number::Int(x), Number::Int(y)) => {
                if x % y != BigInt::ZERO {
                    // There is a remainder, need to convert both sides to `Number::Decimal`
                    // so we perform decimal division, not integer division.
                    self.promote();
                    &*self / rhs
                } else {
                    // Integer division would not produce a remainder, ok to use integer division.
                    Number::Int(x / y)
                }
            }
        }
    }
}

impl Div<Number> for Number {
    type Output = Number;

    fn div(mut self, rhs: Number) -> Self::Output {
        self.div_assign(&rhs);
        self
    }
}

impl Div<&Number> for &Number {
    type Output = Number;

    fn div(self, rhs: &Number) -> Self::Output {
        match (self, rhs) {
            (Number::Decimal(x), Number::Decimal(y)) => Number::Decimal(x / y),
            (Number::Int(x), Number::Int(y)) => {
                if x % y != BigInt::ZERO {
                    // There is a remainder, need to convert both sides to
                    // `Number::Decimal` so we perform decimal division, not integer division.
                    let l = BigDecimal::from(x.clone());
                    let r = BigDecimal::from(y.clone());
                    Number::Decimal(l / r)
                } else {
                    Number::Int(x / y)
                }
            }
            (Number::Int(x), Number::Decimal(y)) => {
                let x = BigDecimal::from(x.clone());
                Number::Decimal(x / y)
            }
            (Number::Decimal(x), Number::Int(y)) => {
                let y = BigDecimal::from(y.clone());
                Number::Decimal(x / y)
            }
        }
    }
}

// ===========================================================================================
// ========================== MulAssign/Mul ==================================================
// ===========================================================================================

impl MulAssign<Number> for Number {
    fn mul_assign(&mut self, rhs: Number) {
        self.mul_assign(&rhs);
    }
}

impl MulAssign<&Number> for Number {
    fn mul_assign(&mut self, rhs: &Number) {
        match_arithmetic_assign!(self, rhs, *);
    }
}

impl Mul<Number> for Number {
    type Output = Number;

    fn mul(mut self, rhs: Number) -> Self::Output {
        self.mul_assign(&rhs);
        self
    }
}

impl Mul<&Number> for &Number {
    type Output = Number;

    fn mul(self, rhs: &Number) -> Self::Output {
        match_arithmetic!(self, rhs, *)
    }
}

// ===========================================================================================
// ========================== RemAssign/Rem ==================================================
// ===========================================================================================

impl RemAssign<Number> for Number {
    fn rem_assign(&mut self, rhs: Number) {
        self.rem_assign(&rhs);
    }
}

impl RemAssign<&Number> for Number {
    fn rem_assign(&mut self, rhs: &Number) {
        match_arithmetic_assign!(self, rhs, %);
    }
}

impl Rem<Number> for Number {
    type Output = Number;

    fn rem(mut self, rhs: Number) -> Self::Output {
        self.rem_assign(&rhs);
        self
    }
}

impl Rem<&Number> for &Number {
    type Output = Number;

    fn rem(self, rhs: &Number) -> Self::Output {
        match_arithmetic!(self, rhs, %)
    }
}

// ===========================================================================================
// ========================== Neg ============================================================
// ===========================================================================================

impl Neg for Number {
    type Output = Number;

    fn neg(self) -> Self::Output {
        match self {
            Number::Int(i) => Number::Int(-i),
            Number::Decimal(d) => Number::Decimal(-d),
        }
    }
}

impl Neg for &Number {
    type Output = Number;

    fn neg(self) -> Self::Output {
        match self {
            Number::Int(i) => Number::Int(-i),
            Number::Decimal(d) => Number::Decimal(-d),
        }
    }
}
