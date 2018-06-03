use num;
use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, FromPrimitive)]
pub enum WhiteBlack {
    White = 0,
    Black,
}

impl fmt::Display for WhiteBlack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = match self {
            WhiteBlack::White => "White",
            WhiteBlack::Black => "Black",
        };
        write!(f, "WhiteBlack({})", color)
    }
}

impl From<usize> for WhiteBlack {
    fn from(c: usize) -> Self {
        num::FromPrimitive::from_usize(c).unwrap()
    }
}

impl WhiteBlack {
    pub fn toggle(self) -> WhiteBlack {
        if self == WhiteBlack::White {
            WhiteBlack::Black
        } else {
            WhiteBlack::White
        }
    }
}
