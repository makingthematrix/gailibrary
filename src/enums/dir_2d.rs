use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Dir2D {
    Up,
    Right,
    Down,
    Left,
}

impl fmt::Display for Dir2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dir = match *self {
            Dir2D::Up => "Up",
            Dir2D::Right => "Right",
            Dir2D::Down => "Down",
            Dir2D::Left => "Left",
        };
        write!(f, "Dir2D({})", dir)
    }
}

lazy_static! {
    pub static ref dirs4: [Dir2D; 4] = [Dir2D::Up, Dir2D::Right, Dir2D::Down, Dir2D::Left];
}

impl Dir2D {
    pub fn turn_right(self) -> Dir2D {
        match self {
            Dir2D::Up => Dir2D::Right,
            Dir2D::Right => Dir2D::Down,
            Dir2D::Down => Dir2D::Left,
            Dir2D::Left => Dir2D::Up,
        }
    }

    pub fn turn_left(self) -> Dir2D {
        match self {
            Dir2D::Up => Dir2D::Left,
            Dir2D::Right => Dir2D::Up,
            Dir2D::Down => Dir2D::Right,
            Dir2D::Left => Dir2D::Down,
        }
    }

    pub fn turn_around(self) -> Dir2D {
        match self {
            Dir2D::Up => Dir2D::Down,
            Dir2D::Right => Dir2D::Left,
            Dir2D::Down => Dir2D::Up,
            Dir2D::Left => Dir2D::Right,
        }
    }
}
