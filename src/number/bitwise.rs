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

// ===========================================================================================
// ========================== Tests ==========================================================
// ===========================================================================================

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;
    use std::str::FromStr as _;

    #[rstest]
    #[case::bitxor1("55", "84", "99")]
    #[case::bitxor2("57.284", "98.345", "91")]
    fn bitxor(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        let r = x ^ y;
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::bitxor_assign1("55", "84", "99")]
    #[case::bitxor_assign2("57.284", "98.345", "91")]
    fn bitxor_assign(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let mut x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        x ^= y;
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[rstest]
    #[case::bitand1("55", "84", "20")]
    #[case::bitand2("55.4", "77.475", "5")]
    fn bitand(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        let r = x & y;
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::bitand_assign1("55", "84", "20")]
    #[case::bitand_assign2("55.4", "77.475", "5")]
    fn bitand_assign(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let mut x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        x &= y;
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[rstest]
    #[case::bitor1("55", "84", "119")]
    #[case::bitor2(
        "97014118346046923173168730371588434847849321057273236539018427",
        "56473890472713285943048728314",
        "97014118346046923173168730371588439898750848355010217494179579"
    )]
    #[case::bitor3("55.432", "84.2113485", "119")]
    fn bitor(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        let r = x | y;
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::bitor_assign1("55", "84", "119")]
    #[case::bitor_assign2(
        "97014118346046923173168730371588434847849321057273236539018427",
        "56473890472713285943048728314",
        "97014118346046923173168730371588439898750848355010217494179579"
    )]
    #[case::bitor_assign3("55.432", "84.2113485", "119")]
    fn bitor_assign(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let mut x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        x |= y;
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[rstest]
    #[case::shl1("55", "8", "14080")]
    #[case::shl2(
        "9701411834604692317316873037158843484784932105727",
        "2",
        "38805647338418769269267492148635373939139728422908"
    )]
    #[case::shl_lhs_decimal("10.5", "2", "40")]
    #[case::shl_lhs_decimal("10.534", "2.234", "40")]
    fn shl(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        let r = x << y;
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::shl_assign1("55", "8", "14080")]
    #[case::shl_assign2(
        "9701411834604692317316873037158843484784932105727",
        "2",
        "38805647338418769269267492148635373939139728422908"
    )]
    #[case::shl_assign_lhs_decimal("10.5", "2", "40")]
    #[case::shl_assign_lhs_decimal("10.534", "2.234", "40")]
    fn shl_assign(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let mut x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        x <<= y;
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[rstest]
    #[case::shr1("873", "5", "27")]
    #[case::shr2(&i128::MAX.to_string(), "2", "42535295865117307932921825928971026431")]
    #[case::shr_lhs_gt_i128_max(
        "34028236692093846346337460743176821145434832943245",
        "2",
        "8507059173023461586584365185794205286358708235811"
    )]
    fn shr(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        let r = x >> y;
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::shr_assign1("873", "5", "27")]
    #[case::shr_assign2(&i128::MAX.to_string(), "2", "42535295865117307932921825928971026431")]
    #[case::shr_lhs_gt_i128_max(
        "34028236692093846346337460743176821145434832943245",
        "2",
        "8507059173023461586584365185794205286358708235811"
    )]
    fn shr_assign(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let mut x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        x >>= y;
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[rstest]
    #[case::not1("55", "-56")]
    #[case::not2(
        "97014118346046923173168730371588434847849321057273236539018427",
        "-97014118346046923173168730371588434847849321057273236539018428"
    )]
    #[case::not3("55.432", "-56")]
    fn not(#[case] lhs: &str, #[case] expect: &str) {
        let x = Number::from_str(lhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        let rr = !&x;
        assert_eq!(rr, e, "[by ref] expected {e:?}, got {rr:?}");
        let r = !x;
        assert_eq!(r, e, "[by val] expected {e:?} got {r:?}");
    }
}
