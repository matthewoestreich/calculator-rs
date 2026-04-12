use crate::Number;
use num_traits::Zero;

impl Number {
    pub fn is_zero(&self) -> bool {
        match self {
            Number::Int(i) => i.is_zero(),
            Number::Decimal(d) => d.is_zero(),
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l), Self::Int(r)) => l == r,
            (Self::Decimal(l), Self::Decimal(r)) => l == r,
            _ => false,
        }
    }
}

impl Eq for Number {}
