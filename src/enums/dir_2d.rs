use num;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, FromPrimitive)]
pub enum Dir2D {
    None = 0,
    Up,
    Right,
    Down,
    Left,
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
        if let Some(d) = self.handle {
            self.handle = num::FromPrimitive::from_usize((d as usize) + 1);
        }
        self.handle
    }
}

impl Dir2D {
    pub fn iter() -> Dir2DIter {
        Dir2DIter { handle: Some(Dir2D::Up) }
    }

    pub fn turn_right(&self) -> Dir2D {
        match *self {
            Dir2D::None => Dir2D::None,
            Dir2D::Up => Dir2D::Right,
            Dir2D::Right => Dir2D::Down,
            Dir2D::Down => Dir2D::Left,
            Dir2D::Left => Dir2D::Up,
        }
    }

    pub fn turn_left(&self) -> Dir2D {
        match *self {
            Dir2D::None => Dir2D::None,
            Dir2D::Up => Dir2D::Left,
            Dir2D::Right => Dir2D::Up,
            Dir2D::Down => Dir2D::Right,
            Dir2D::Left => Dir2D::Down,
        }
    }

    pub fn turn_around(&self) -> Dir2D {
        match *self {
            Dir2D::None => Dir2D::None,
            Dir2D::Up => Dir2D::Down,
            Dir2D::Right => Dir2D::Left,
            Dir2D::Down => Dir2D::Up,
            Dir2D::Left => Dir2D::Right,
        }
    }
}