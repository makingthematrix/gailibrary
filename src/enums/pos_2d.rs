use std::cmp;
use std::vec::Vec;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Pos2D {
    x: usize,
    y: usize,
}

lazy_static! {
    static ref ZERO: Pos2D = Pos2D { x: 0, y: 0 };
}

impl Pos2D {
    pub fn new(x: usize, y: usize) -> Pos2D {
        Pos2D { x, y }
    }

    pub fn from_dim(dim: usize) -> Vec<Pos2D> {
        Pos2D::from_range(*ZERO, Pos2D::new(dim, dim))
    }

    pub fn from_range(p1: Pos2D, p2: Pos2D) -> Vec<Pos2D> {
        let xfrom = cmp::min(p1.x, p2.x);
        let yfrom = cmp::min(p1.y, p2.y);
        let xto = cmp::max(p1.x, p2.x);
        let yto = cmp::max(p1.y, p2.y);

        let mut v = Vec::with_capacity((xto - xfrom) * (yto - yfrom));
        for x in xfrom..=xto {
            for y in yfrom..=yto {
                v.push(Pos2D::new(x, y))
            }
        }

        v
    }
}
