use num;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, FromPrimitive)]
pub enum WhiteBlack {
    White = 0,
    Black,
}

impl From<usize> for WhiteBlack {
    fn from(c: usize) -> Self {
        num::FromPrimitive::from_usize(c).unwrap()
    }
}

impl WhiteBlack {
    pub fn toggle(&self) -> WhiteBlack {
        if *self == WhiteBlack::White { WhiteBlack::Black } else { WhiteBlack::White }
    }
}