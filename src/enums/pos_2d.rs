use enums::dir_2d::Dir2D;
use std::cmp;
use std::fmt;
use std::vec::Vec;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Pos2D {
    pub x: i64,
    pub y: i64,
}

const ZERO: Pos2D = Pos2D { x: 0, y: 0 };

impl Pos2D {
    pub fn new(x: i64, y: i64) -> Pos2D {
        Pos2D { x, y }
    }

    pub fn from_dim(dim: usize) -> Vec<Pos2D> {
        Pos2D::from_range(ZERO, Pos2D::new(dim as i64, dim as i64))
    }

    pub fn from_range(p1: Pos2D, p2: Pos2D) -> Vec<Pos2D> {
        let xfrom = cmp::min(p1.x, p2.x);
        let yfrom = cmp::min(p1.y, p2.y);
        let xto = cmp::max(p1.x, p2.x);
        let yto = cmp::max(p1.y, p2.y);

        let mut v = Vec::with_capacity(((xto - xfrom) * (yto - yfrom)) as usize);
        for x in xfrom..xto {
            for y in yfrom..yto {
                v.push(Pos2D::new(x, y))
            }
        }

        v
    }

    pub fn move_by_one(&self, dir: Dir2D) -> Pos2D {
        match dir {
            Dir2D::Up => Pos2D::new(self.x, self.y - 1),
            Dir2D::Right => Pos2D::new(self.x + 1, self.y),
            Dir2D::Down => Pos2D::new(self.x, self.y + 1),
            Dir2D::Left => Pos2D::new(self.x - 1, self.y),
        }
    }

    pub fn dir_to(&self, pos: Pos2D) -> Dir2D {
        Dir2D::approx4(pos.x as f64 - self.x as f64, pos.y as f64 - self.y as f64)
    }
}

impl fmt::Display for Pos2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "Pos2D({}, {})", self.x, self.y)
    }
}
