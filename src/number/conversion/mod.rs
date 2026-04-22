mod from;
mod to;

pub(crate) use to::*;

use crate::{Number, ToNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;

impl Number {
    /// Encodes a string into base64 encoded string.
    /// ```rust,ignore
    /// use calcinum::Number;
    /// let encoded = Number::base64_encode("abcd");
    /// assert_eq!(encoded, "YWJjZA==");
    /// ```
    pub(crate) fn base64_encode(s: &str) -> String {
        let alpha = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut encoded = String::new();
        let mut buf = 0;
        let mut bits = 0;

        for byte in s.as_bytes() {
            buf = (buf << 8) | *byte as u32;
            bits += 8;
            while bits >= 6 {
                bits -= 6;
                let i = (buf >> bits) & 0b111111;
                encoded.push(alpha[i as usize] as char);
            }
        }

        if bits > 0 {
            let i = (buf << (6 - bits)) & 0b111111;
            encoded.push(alpha[i as usize] as char);
        }

        // Padding
        while !encoded.len().is_multiple_of(4) {
            encoded.push('=');
        }

        encoded
    }

    /// Decode a base64 string to it's original form.
    pub(crate) fn base64_decode(s: &str) -> String {
        let s = s.trim_end_matches('=');
        let alpha = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut decoded = String::new();
        let mut buf = 0;
        let mut bits = 0;

        for byte in s.as_bytes() {
            let value = alpha.iter().position(|c| c == byte).unwrap_or(0);
            buf = (buf << 6) | value as u32;
            bits += 6;
            if bits >= 8 {
                bits -= 8;
                decoded.push(((buf >> bits) as u8) as char);
            }
        }

        decoded
    }
}

impl_to_number!(u8);
impl_to_number!(u16);
impl_to_number!(u32);
impl_to_number!(u64);
impl_to_number!(u128);
impl_to_number!(i8);
impl_to_number!(i16);
impl_to_number!(i32);
impl_to_number!(i64);
impl_to_number!(i128);

impl ToNumber for f64 {
    fn to_number(&self) -> Number {
        Number::from_f64(*self).expect("Number")
    }
}

impl ToNumber for BigInt {
    fn to_number(&self) -> Number {
        Number::from(self)
    }
}

impl ToNumber for BigDecimal {
    fn to_number(&self) -> Number {
        Number::from(self)
    }
}

// ===========================================================================================
// ========================== Tests ==========================================================
// ===========================================================================================

#[cfg(test)]
mod test {
    use crate::number::ToNumber;
    use crate::*;
    use rstest::*;
    use std::str::FromStr as _;

    #[test]
    fn round_trip_binary_conversion() {
        let i = 123.to_number(); // Number::Int(123)
        let bs = format!("{i:b}"); // "1111011"
        // Parse binary string back into `Number` - needs "0b" prefix.
        let s = format!("0b{bs}");
        let n = s.parse::<Number>().unwrap(); // Number::Int(123)
        assert_eq!(i, n);

        let i = 382.619.to_number(); // Number::Decimal(382.619)
        let bs = format!("{i:b}"); // "1111011"
        // Parse binary string back into `Number` - needs "0b" prefix.
        let s = format!("0b{bs}");
        let n = s.parse::<Number>().unwrap(); // Number::Decimal(382.619)
        assert_eq!(i, n);
    }

    #[rstest]
    #[case::from_str1("2.2", "2.2")]
    #[case::from_str2("1", "1")]
    #[case::from_str3("0b00000000000001110001110101110101.1000011011", "466293.539")]
    #[case::from_str4("-0b00000000000001110001110101110101.1000011011", "-466293.539")]
    #[case::no_binary_prefix_dont_treat_as_binary("10101011001", "10101011001")]
    #[case::from_str5("0b1010", "10")]
    #[case::from_str6("0b1010.1010", "10.10")]
    #[case::from_str7("-0b11110000010100011111", "-984351")]
    #[should_panic]
    #[case::from_str_panic("abcd", "")]
    #[should_panic]
    #[case::from_str_panic_contains_invalid_num_3("0b101010131001", "")]
    #[should_panic]
    #[case::from_str_panic_multiple_neg("-0b101010-131001", "")]
    #[should_panic]
    #[case::from_str_panic_multiple_decimals("0b1010.1013.1001", "")]
    #[should_panic]
    #[case::from_str_panic("   ", "")]
    #[should_panic]
    #[case::from_str_panic("0b", "")]
    #[case::from_str_b64_1("b64LTIzNDUuMTIzNQ==", "-2345.1235")]
    #[case::from_str_b64_2("b64NDM1NDMuMzIyOTM4NDAz", "43543.322938403")]
    #[case::from_str_b64_3("b64NDM1MjQzOTg1MjQzMzE0OQ==", "4352439852433149")]
    #[case::from_str_b64_4("b64LTAwMDAwMDAwMC4wMDAwMDAwMDAw", "-000000000.0000000000")]
    fn from_str(#[case] number: &str, #[case] expect: &str) {
        let x = Number::from_str(number).expect("Number::from_str");
        let e = expect.parse::<Number>().expect("to parse 'expect' param");
        assert_eq!(x, e, "expected '{e:?}' got '{x:?}'");
    }

    #[rstest]
    #[case::from_str_hex1("0x20FDE.3CBD04", "135134.3980548")]
    #[case::from_str_hex2("-0x20FDE.3CBD04", "-135134.3980548")]
    #[case::from_str_hex3("0x1", "1")]
    #[case::from_str_hex4(
        "0xd0d0c7c5742a63ee3d89fb998ca24c7a",
        "277563472713248395635956171186146266234"
    )]
    fn from_hex_str(#[case] number: &str, #[case] expect: &str) {
        let x = Number::from_hexadecimal_str(number).expect("hex to Number");
        let e = expect.parse::<Number>().expect("control string to parse");
        assert_eq!(x, e, "expected '{e:?}' got '{x:?}'");
    }

    #[rstest]
    #[case::bin_str_to_number1("0b1010", "10")]
    #[case::bin_str_to_number2("-0b1010", "-10")]
    #[case::bin_str_to_number3("0b00000000000001110001110101110101.1000011011", "466293.539")]
    #[case::bin_str_to_number4("-0b00000000000001110001110101110101.1000011011", "-466293.539")]
    fn binary_str_to_number(#[case] number: &str, #[case] expect: &str) {
        let x = match Number::from_str(number) {
            Ok(r) => r,
            Err(e) => panic!("ERROR => '{number}' is not a binary string => {e:?}"),
        };
        let e = expect
            .parse::<Number>()
            .expect("expected 'expect' argument to parse just fine into Number");
        assert_eq!(x, e, "expected '{e:?}' got '{x:?}'");
    }

    #[rstest]
    #[case::b64encode("-2345.1235", "LTIzNDUuMTIzNQ==")]
    #[case::b64encode("43543.322938403", "NDM1NDMuMzIyOTM4NDAz")]
    #[case::b64encode("4352439852433149", "NDM1MjQzOTg1MjQzMzE0OQ==")]
    #[case::b64encode("-000000000.0000000000", "LTAwMDAwMDAwMC4wMDAwMDAwMDAw")]
    fn base64_encode_decode(#[case] s: &str, #[case] expect: &str) {
        let encoded = Number::base64_encode(s);
        assert_eq!(
            encoded, expect,
            "expected encoded '{expect}' got encoded '{encoded}'"
        );
        let decoded = Number::base64_decode(&encoded);
        assert_eq!(
            decoded,
            s.to_string(),
            "expected decoded = '{s}' got decoded '{decoded}'"
        );
    }

    #[test]
    fn from_f64() {
        let a = Number::from_f64(1.1).unwrap();
        assert_eq!(a.order(), NumberOrder::Decimal);
    }
}
