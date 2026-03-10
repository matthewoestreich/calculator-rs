use crate::value::Value;
use std::{error, fmt};

#[derive(Debug, Clone)]
pub enum ValueError {
    Converting { from: Value, to: String },
    Parsing { value: String },
    ImproperlyFloat,
    Overflow,
    Underflow,
    DivideByZero,
    NegativeExponent,
}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueError::Converting { from, to } => {
                write!(f, "cannot convert {:?} to {:?}", from, to)
            }
            ValueError::Parsing { value } => write!(f, "'{value}' cannot be parsed as Value"),
            ValueError::ImproperlyFloat => write!(
                f,
                "attempted to perform an operation which only makes sense for integers, but value is currently a float"
            ),
            ValueError::Overflow => write!(f, "overflow"),
            ValueError::Underflow => write!(f, "underflow"),
            ValueError::DivideByZero => write!(f, "attempt to divide by 0"),
            ValueError::NegativeExponent => write!(f, "attempt to pow using negative exponent"),
        }
    }
}

impl error::Error for ValueError {}
