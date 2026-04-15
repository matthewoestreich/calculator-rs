use crate::NumberError;
use std::{fmt, str::FromStr};

#[repr(u8)]
pub enum Nibble {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Eleven = 11,
    Twelve = 12,
    Thirteen = 13,
    Fourteen = 14,
    Fifteen = 15,
}

impl Nibble {
    pub fn from_str_unchecked(s: &str) -> Self {
        Self::from_str(s).expect("this method is unchecked")
    }

    pub fn to_hex(&self, uppercase: bool) -> String {
        let s = match self {
            Nibble::Ten => "a",
            Nibble::Eleven => "b",
            Nibble::Twelve => "c",
            Nibble::Thirteen => "d",
            Nibble::Fourteen => "e",
            Nibble::Fifteen => "f",
            _ => &format!("{self}"),
        };
        if uppercase {
            s.to_uppercase()
        } else {
            s.to_lowercase()
        }
    }
}

impl FromStr for Nibble {
    type Err = NumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "0" => Self::Zero,
            "1" => Self::One,
            "2" => Self::Two,
            "3" => Self::Three,
            "4" => Self::Four,
            "5" => Self::Five,
            "6" => Self::Six,
            "7" => Self::Seven,
            "8" => Self::Eight,
            "9" => Self::Nine,
            "10" => Self::Ten,
            "11" => Self::Eleven,
            "12" => Self::Twelve,
            "13" => Self::Thirteen,
            "14" => Self::Fourteen,
            "15" => Self::Fifteen,
            _ => {
                return Err(NumberError::Parsing {
                    value: format!("'{s}' out of Nibble range 0..=15"),
                });
            }
        })
    }
}

impl fmt::Display for Nibble {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = String::from(match self {
            Nibble::Zero => "0",
            Nibble::One => "1",
            Nibble::Two => "2",
            Nibble::Three => "3",
            Nibble::Four => "4",
            Nibble::Five => "5",
            Nibble::Six => "6",
            Nibble::Seven => "7",
            Nibble::Eight => "8",
            Nibble::Nine => "9",
            Nibble::Ten => "10",
            Nibble::Eleven => "11",
            Nibble::Twelve => "12",
            Nibble::Thirteen => "13",
            Nibble::Fourteen => "14",
            Nibble::Fifteen => "15",
        });
        write!(f, "{s}")
    }
}
