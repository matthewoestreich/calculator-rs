use crate::Number;
use num_bigint::ToBigInt;
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

// ===========================================================================================
// ========================== BitAndAssign/BitAnd ============================================
// ===========================================================================================
//
// IMPORTANT : we can only perform bitwise operations on Number::Int.
// IMPORTANT : If either side is Number::Decimal we convert the Decimal into an integer before
// calling the bitwise operation, which may result in unexpected calculations!
//

impl BitAndAssign<Number> for Number {
    fn bitand_assign(&mut self, rhs: Number) {
        self.bitand_assign(&rhs);
    }
}

impl BitAndAssign<&Number> for Number {
    fn bitand_assign(&mut self, rhs: &Number) {
        match_bitwise_assign!(self, rhs, &);
    }
}

impl BitAnd<Number> for Number {
    type Output = Number;

    fn bitand(mut self, rhs: Number) -> Self::Output {
        self.bitand_assign(&rhs);
        self
    }
}

impl BitAnd<&Number> for &Number {
    type Output = Number;

    fn bitand(self, rhs: &Number) -> Self::Output {
        match_bitwise!(self, rhs, &)
    }
}

// ===========================================================================================
// ========================== BitOrAssign/BitOr ==============================================
// ===========================================================================================
//
// IMPORTANT : we can only perform bitwise operations on Number::Int.
// IMPORTANT : If either side is Number::Decimal we convert the Decimal into an integer before
// calling the bitwise operation, which may result in unexpected calculations!
//

impl BitOrAssign<Number> for Number {
    fn bitor_assign(&mut self, rhs: Number) {
        self.bitor_assign(&rhs);
    }
}

impl BitOrAssign<&Number> for Number {
    fn bitor_assign(&mut self, rhs: &Number) {
        match_bitwise_assign!(self, rhs, |);
    }
}

impl BitOr<Number> for Number {
    type Output = Number;

    fn bitor(mut self, rhs: Number) -> Self::Output {
        self.bitor_assign(&rhs);
        self
    }
}

impl BitOr<&Number> for &Number {
    type Output = Number;

    fn bitor(self, rhs: &Number) -> Self::Output {
        match_bitwise!(self, rhs, |)
    }
}

// ===========================================================================================
// ========================== BitXorAssign/BitXor ============================================
// ===========================================================================================
//
// IMPORTANT : we can only perform bitwise operations on Number::Int.
// IMPORTANT : If either side is Number::Decimal we convert the Decimal into an integer before
// calling the bitwise operation, which may result in unexpected calculations!
//

impl BitXorAssign<Number> for Number {
    fn bitxor_assign(&mut self, rhs: Number) {
        self.bitxor_assign(&rhs);
    }
}

impl BitXorAssign<&Number> for Number {
    fn bitxor_assign(&mut self, rhs: &Number) {
        match_bitwise_assign!(self, rhs, ^);
    }
}

impl BitXor<Number> for Number {
    type Output = Number;

    fn bitxor(mut self, rhs: Number) -> Self::Output {
        self.bitxor_assign(&rhs);
        self
    }
}

impl BitXor<&Number> for &Number {
    type Output = Number;

    fn bitxor(self, rhs: &Number) -> Self::Output {
        match_bitwise!(self, rhs, ^)
    }
}

// ===========================================================================================
// ========================== ShlAssign/Shl ==================================================
// ===========================================================================================
//
// IMPORTANT : We can only left shift by numbers that fit within an i128! If your right
// hand side does not fit within an i128 it will be satured, which may result in data loss!
//

impl ShlAssign<Number> for Number {
    fn shl_assign(&mut self, rhs: Number) {
        self.shl_assign(&rhs);
    }
}

impl ShlAssign<&Number> for Number {
    fn shl_assign(&mut self, rhs: &Number) {
        match_shift_assign!(self, rhs, <<);
    }
}

impl Shl<Number> for Number {
    type Output = Number;

    fn shl(mut self, rhs: Number) -> Self::Output {
        self.shl_assign(&rhs);
        self
    }
}

impl Shl<&Number> for &Number {
    type Output = Number;

    fn shl(self, rhs: &Number) -> Self::Output {
        match_shift!(self, rhs, <<)
    }
}

// ===========================================================================================
// ========================== ShrAssign/Shr ==================================================
// ===========================================================================================
//
// IMPORTANT : We can only right shift by numbers that fit within an i128! If your right
// hand side does not fit within an i128 it will be satured, which may result in data loss!
//

impl ShrAssign<Number> for Number {
    fn shr_assign(&mut self, rhs: Number) {
        self.shr_assign(&rhs);
    }
}

impl ShrAssign<&Number> for Number {
    fn shr_assign(&mut self, rhs: &Number) {
        match_shift_assign!(self, rhs, >>);
    }
}

impl Shr<Number> for Number {
    type Output = Number;

    fn shr(mut self, rhs: Number) -> Self::Output {
        self.shr_assign(&rhs);
        self
    }
}

impl Shr<&Number> for &Number {
    type Output = Number;

    fn shr(self, rhs: &Number) -> Self::Output {
        match_shift!(self, rhs, >>)
    }
}

// ===========================================================================================
// ========================== Not ============================================================
// ===========================================================================================
//
// IMPORTANT : We can only call `Not` on `Number::Int` variants! If your variant is
// `Number::Decimal` we first demote it to `Number::Int` before calling `Not`, which
// may result in data loss/unexpected calculations!
//

impl Not for Number {
    type Output = Number;

    fn not(self) -> Self::Output {
        match self {
            Number::Int(i) => Number::Int(!i),
            Number::Decimal(d) => Number::Int(!d.to_bigint().expect("BigInt")),
        }
    }
}

impl Not for &Number {
    type Output = Number;

    fn not(self) -> Self::Output {
        match self {
            Number::Int(i) => Number::Int(!i),
            Number::Decimal(d) => Number::Int(!d.to_bigint().expect("BigInt")),
        }
    }
}
