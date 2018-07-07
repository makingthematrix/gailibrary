use num;
use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, FromPrimitive)]
pub enum Dir2D {
    Up = 0,
    Right,
    Down,
    Left,

    None,
}

impl fmt::Display for Dir2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dir = match *self {
            Dir2D::None => "None",
            Dir2D::Up => "Up",
            Dir2D::Right => "Right",
            Dir2D::Down => "Down",
            Dir2D::Left => "Left",
        };
        write!(f, "Dir2D({})", dir)
    }
}

impl From<usize> for Dir2D {
    fn from(d: usize) -> Self {
        num::FromPrimitive::from_usize(d).unwrap()
    }
}

pub struct Dir2DIter {
    handle: Option<Dir2D>,
}

impl Iterator for Dir2DIter {
    type Item = Dir2D;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        match self.handle {
            None => None,
            Some(Dir2D::None) => None,
            Some(d) => {
                self.handle = num::FromPrimitive::from_usize((d as usize) + 1);
                Some(d)
            }
        }
    }
}

impl Dir2D {
    pub fn iter() -> Dir2DIter {
        Dir2DIter {
            handle: Some(Dir2D::Up),
        }
    }

    pub fn turn_right(self) -> Dir2D {
        match self {
            Dir2D::None => Dir2D::None,
            Dir2D::Up => Dir2D::Right,
            Dir2D::Right => Dir2D::Down,
            Dir2D::Down => Dir2D::Left,
            Dir2D::Left => Dir2D::Up,
        }
    }

    pub fn turn_left(self) -> Dir2D {
        match self {
            Dir2D::None => Dir2D::None,
            Dir2D::Up => Dir2D::Left,
            Dir2D::Right => Dir2D::Up,
            Dir2D::Down => Dir2D::Right,
            Dir2D::Left => Dir2D::Down,
        }
    }

    pub fn turn_around(self) -> Dir2D {
        match self {
            Dir2D::None => Dir2D::None,
            Dir2D::Up => Dir2D::Down,
            Dir2D::Right => Dir2D::Left,
            Dir2D::Down => Dir2D::Up,
            Dir2D::Left => Dir2D::Right,
        }
    }
}
